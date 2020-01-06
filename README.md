- [Problems](#problems)
  - [1. PeerID](#1-peerid)
    - [Reproduce:](#reproduce)
    - [Description](#description)
  - [2. SecIO handshake hangs](#2-secio-handshake-hangs)
    - [Reproduce:](#reproduce-1)
    - [JS logs](#js-logs)
    - [Rust logs](#rust-logs)

## Libp2p: Rust + JS
Attempt to connect libp2p in Rust with libp2p in JS via gossipsub

## Problem: JS can't publish message to Rust
### Reproduce:
```bash
# In a first terminal window/tab, execute
make rust 

# In a second terminal window/tab, execute
make js
```
#### Description
I'm trying to connect JS to Rust using Gossipsub, and can't get messages to be published from JS to Rust. Rust-to-rust and rust-to-js works fine. It's the same if Floodsub is used on both sides, but Gossipsub gives better logs.

It seems that JS closes outbound peer right after attempt to publish a message. I don't understand the reason for that, and all my debugging attempts lead me to `pull-stream` suddenly closing. Here's a stacktrace for peer removal message:

```
    at GossipSub._removePeer (project/node_modules/libp2p-gossipsub/src/index.js:80:13)
    at Peer.peer.once (project/node_modules/libp2p-pubsub/src/index.js:110:37)
    at Object.onceWrapper (events.js:286:20)
    at Peer.emit (events.js:198:13)
    at pull.onEnd (project/node_modules/libp2p-pubsub/src/peer.js:92:14)
    at project/node_modules/pull-stream/sinks/drain.js:20:24
    at project/node_modules/pull-stream/throughs/map.js:19:9
    at project/node_modules/pull-reader/index.js:114:13
    at project/node_modules/pull-reader/index.js:114:13
    at onNext (project/node_modules/pull-catch/index.js:17:17)
```

Here's how it looks in logs:
```
1   Discovered peer: 123D3...
2   Connection started to: 123D3...
3   Connection established to: 123D3...
4     libp2p:gossipsub dialing /meshsub/1.0.0 123D3... +64ms
5   node dialed
6     libp2p:gossipsub JOIN chat +1ms
7   pubsub subscribed
8     libp2p:gossipsub new peer 123D3... +9ms
9     libp2p:gossipsub rpc from 123D3... +3ms
10    libp2p:gossipsub dial to 123D3... complete +0ms
11    libp2p:gossipsub connected 123D3... +0ms
12    libp2p:gossipsub HEARTBEAT: Add mesh link to 123D3... in chat +935ms
13  HEY!
14  will send to pubsub HEY!
15  
16    libp2p:gossipsub publish chat <Buffer 48 45 59 21 0a> +2m
17    libp2p:gossipsub remove 123D3... 2 +5ms
18    libp2p:gossipsub connection ended 123D3...  +29m
19    libp2p:gossipsub remove 123D3... 1 +0ms
20    libp2p:gossipsub delete peer 123D3... +0ms
21  Connection ended with: 123D3...
22    libp2p:gossipsub HEARTBEAT: Add mesh link to 123D3... in chat +757ms
```
Where `123D3...` is the shortened address of the remote peer.

As visible on lines 16-17, connection is closed right after publish. That message isn't received on the remote side, and all following publishes are processed since `peer.isWritable` is false. However, JS still receives messages from remote (Rust).