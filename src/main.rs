use futures::{future, prelude::*};
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    identity::PublicKey,
    ping::{Ping, PingConfig},
    swarm::NetworkBehaviourEventProcess,
    tokio_codec::{FramedRead, LinesCodec},
    tokio_io::{AsyncRead, AsyncWrite},
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};
use std::env;
use std::time::Duration;

use base64;

use identity::ed25519;
use identity::Keypair;
use libp2p_identify::{Identify, IdentifyEvent};

// TODO: connect with js
// TODO: secio
// TODO: what is webrtcStar? https://github.com/libp2p/js-libp2p/tree/master/examples/libp2p-in-the-browser/1/src
// TODO: refactor out common code (I tried and haven't succeeded: ExpandedSwarm type is a complex beast)

const PRIVATE_KEY: &str =
    "/O5p1cDNIyEkG3VP+LqozM+gArhSXUdWkKz6O+C6Wtr+YihU3lNdGl2iuH37ky2zsjdv/NJDzs11C1Vj0kClzQ==";

#[derive(NetworkBehaviour)]
struct MyBehaviour<TSubstream: AsyncRead + AsyncWrite> {
    floodsub: Floodsub<TSubstream>,
    identify: Identify<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<FloodsubEvent>
    for MyBehaviour<TSubstream>
{
    // Called when `floodsub` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            println!(
                "Received floodsub msg: '{:?}' from {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<IdentifyEvent>
    for MyBehaviour<TSubstream>
{
    fn inject_event(&mut self, event: IdentifyEvent) {
        //        println!("Received identify event {:?}", event);
    }
}

fn serve(port: i32) {
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

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key.clone());

    // Create a Floodsub topic
    let floodsub_topic = floodsub::TopicBuilder::new("chat").build();
    println!("floodsub topic is {:?}", floodsub_topic);

    let mut swarm = {
        //        let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
        //        Swarm::new(transport, behaviour, local_peer_id.clone())
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            identify: Identify::new("1.0.0".into(), "1.0.0".into(), local_key.public()),
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::new(transport, behaviour, local_peer_id.clone())
    };

    // Tell the swarm to listen on all interfaces and a random, OS-assigned port.
    let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", port).parse().unwrap();
    Swarm::listen_on(&mut swarm, addr.clone()).unwrap();
    //    Swarm::add_external_address(&mut swarm, addr.clone());

    let stdin = tokio_stdin_stdout::stdin(0);
    let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new());

    // Use tokio to drive the `Swarm`.
    let mut listening = false;
    tokio::run(future::poll_fn(move || -> Result<_, ()> {
        loop {
            match framed_stdin.poll().expect("Error while polling stdin") {
                Async::Ready(Some(line)) => {
                    println!("sending floodsub msg");
                    swarm.floodsub.publish(&floodsub_topic, line.as_bytes())
                }
                Async::Ready(None) => panic!("Stdin closed"),
                Async::NotReady => break,
            };
        }

        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(e)) => println!("event {:?}", e), //println!("sent {:?} to {:?}", e.result, e.peer),
                Async::Ready(None) | Async::NotReady => {
                    if !listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {}/p2p/{}", a, local_peer_id);
                            listening = true;
                        }

                        let ea = Swarm::external_addresses(&swarm).next();
                        println!("External address: {:?}", ea);

                        println!("Local peer id {}", Swarm::local_peer_id(&swarm));
                    } else {
                        println!("something happened in swarm runloop");
                    }

                    return Ok(Async::NotReady);
                }
            }
        }
    }));
}

fn main() {
    env_logger::init();

    serve(30000)
}
