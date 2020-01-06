use base64;
use futures::{future, prelude::*};
use identity::ed25519;
use identity::Keypair;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    identity::PublicKey,
    Multiaddr,
    NetworkBehaviour,
    PeerId,
    ping::{Ping, PingConfig, PingEvent},
    Swarm, swarm::NetworkBehaviourEventProcess, tokio_codec::{FramedRead, LinesCodec}, tokio_io::{AsyncRead, AsyncWrite},
};
use libp2p_gossipsub::{Gossipsub, GossipsubEvent, GossipsubConfig, Topic, GossipsubConfigBuilder};
use libp2p_identify::{Identify, IdentifyEvent};

use crate::behaviour::{BehaviourEvent, EventEmittingBehaviour};
use crate::transport;
use std::time::Duration;

const PRIVATE_KEY: &str =
    "/O5p1cDNIyEkG3VP+LqozM+gArhSXUdWkKz6O+C6Wtr+YihU3lNdGl2iuH37ky2zsjdv/NJDzs11C1Vj0kClzQ==";

#[derive(NetworkBehaviour)]
pub struct Network<TSubstream: AsyncRead + AsyncWrite> {
    pub gossipsub: Gossipsub<TSubstream>,
//    pub floodsub: Floodsub<TSubstream>,
    pub identify: Identify<TSubstream>,
    pub ping: Ping<TSubstream>,
    pub logging: EventEmittingBehaviour<TSubstream>,
}

//impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<FloodsubEvent>
//for Network<TSubstream>
//{
//    // Called when `floodsub` produces an event.
//    fn inject_event(&mut self, event: FloodsubEvent) {
//        println!("Received floodsub event {:?}", event);
//
//        match event {
//            // TODO: for some reason, Message isn't passed here
//            //       In logs, message 'HELLO' published by JS for topic '5zKTH5FR' looks like this:
//            //       [2019-12-24T19:17:13Z TRACE libp2p_mplex] Received message: Data { substream_id: 2, endpoint: Dialer, data: b"N\x12L\n\"\x12 8E>\xc8\xc2h\x97\x18\x15\xa9\xce\xd5\x02\xd8\x85\xa5^\xfcU\xafI\xdd>\xacD'\xfe?T4\x15\x87\x12\x06HELLO\n\x1a\x14\x05\xe8\xde\xfah\x1a\xe2\x1fl}M\x8aK\x02\xbb\xcaG\x8f@\x06\"\x085zKTH5FR" }
//            FloodsubEvent::Message(message) => println!(
//                "Received floodsub msg: '{:?}' from {:?}",
//                String::from_utf8_lossy(&message.data),
//                message.source
//            ),
//            // Subscribed works
//            FloodsubEvent::Subscribed { peer_id, topic } => {
//                println!("{:?} subscribed to {:?}", peer_id, topic);
//                // TODO: Will always try to reconnect, basically a leak
//                // Nodes in partial view will receive subscriptions
////                self.floodsub.add_node_to_partial_view(peer_id)
//            }
//            FloodsubEvent::Unsubscribed { peer_id, topic } => {
//                println!("{:?} unsubscribed from {:?}", peer_id, topic);
//                // TODO: how to remove node when there's no more subscriptions from it?
////                self.floodsub.remove_node_from_partial_view(&peer_id)
//            }
//        };
//    }
//}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<GossipsubEvent> for Network<TSubstream> {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message(peer_id, message_id, msg) => {
                println!("gossipsub msg {:?}, {:?}, {:?}", peer_id, message_id, msg)
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                println!("gossipsub {:?} subscribed to {:?}", peer_id, topic)
            }
            GossipsubEvent::Unsubscribed { peer_id, topic } => {
                println!("gossipsub {:?} unsubcribed from {:?}", peer_id, topic)
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<IdentifyEvent>
for Network<TSubstream>
{
    fn inject_event(&mut self, _event: IdentifyEvent) {}
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<PingEvent>
for Network<TSubstream>
{
    fn inject_event(&mut self, _event: PingEvent) {}
}

impl<Substream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<BehaviourEvent> for Network<Substream> {
    fn inject_event(&mut self, event: BehaviourEvent) {
        match event {
            BehaviourEvent::Connected(peer) => {
//                println!("Adding peer {:?} to floodsub view", peer);
//                self.floodsub.add_node_to_partial_view(peer);
            }
            BehaviourEvent::Disconnected(peer) => {
//                println!("Removing peer {:?} from floodsub view", peer);
//                self.floodsub.remove_node_from_partial_view(&peer);
            }
        }
    }
}

pub fn serve(port: i32) {
    // Create a random PeerId
    let mut local_key = base64::decode(PRIVATE_KEY).unwrap();
    let local_key = local_key.as_mut_slice();
    let local_key = Keypair::Ed25519(ed25519::Keypair::decode(local_key).unwrap());
    let local_peer_id = PeerId::from(local_key.public());

    println!("peer id: {}", local_peer_id);

    match local_key.public() {
        PublicKey::Ed25519(key) => println!("Public Key: {}", base64::encode(&key.encode())),
        _ => println!("Key isn't ed25519!!!!!"),
    }

    match local_key.clone() {
        identity::Keypair::Ed25519(pair) => {
            println!("PrivateKey: {}", base64::encode(&pair.encode().to_vec()))
        }
        _ => println!("Key isn't ed25519!!!!!"),
    }

    // mplex + secio
    let transport = transport::build_mplex(local_key.clone());

    // Create a Floodsub topic
    let floodsub_topic = floodsub::TopicBuilder::new("chat").build();
    println!("floodsub topic is {:?}", floodsub_topic);

    let gossipsub_topic = libp2p_gossipsub::Topic::new("chat".into());
    println!("gossipsub topic is {:?}", gossipsub_topic);

    let mut swarm = {
        let gs_config = GossipsubConfigBuilder::new().heartbeat_interval(Duration::from_secs(20)).build();
        let mut behaviour = Network {
//            floodsub: Floodsub::new(local_peer_id.clone()),
            gossipsub: Gossipsub::new(local_peer_id.clone(), gs_config),
            identify: Identify::new("1.0.0".into(), "1.0.0".into(), local_key.public()),
            logging: EventEmittingBehaviour::new(),
            ping: Ping::new(PingConfig::with_keep_alive(PingConfig::new(), true)),
        };

//        behaviour.floodsub.subscribe(floodsub_topic.clone());
        behaviour.gossipsub.subscribe(gossipsub_topic.clone());
        Swarm::new(transport, behaviour, local_peer_id.clone())
    };

    // Tell the swarm to listen on all interfaces and a random, OS-assigned port.
    let addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", port).parse().unwrap();
    Swarm::listen_on(&mut swarm, addr.clone()).unwrap();

    let stdin = tokio_stdin_stdout::stdin(0);
    let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new());

    let mut listening = false;

    // Use tokio to drive the `Swarm`.
    tokio::run(future::poll_fn(move || -> Result<_, ()> {
        // Read input from stdin
        loop {
            match framed_stdin.poll().expect("Error while polling stdin") {
                Async::Ready(Some(line)) => {
//                    println!("sending floodsub msg");
//                    swarm.floodsub.publish(&floodsub_topic, line.as_bytes());
                    println!("sending gossipsub msg");
                    swarm.gossipsub.publish(&gossipsub_topic, line.as_bytes());
                }
                Async::Ready(None) => panic!("Stdin closed"),
                Async::NotReady => break,
            };
        }

        // Some comments on poll may be relevant https://github.com/libp2p/rust-libp2p/issues/1058
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(e)) => println!("Got {:?} ready", e),
                Async::Ready(None) | Async::NotReady => {
                    if !listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {}/p2p/{}", a, local_peer_id);
                            listening = true;
                        }
                    }

                    return Ok(Async::NotReady);
                }
            }
        }
    }));
}
