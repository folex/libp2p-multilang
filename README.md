- [Problems](#problems)
  - [1. PeerID](#1-peerid)
    - [Reproduce:](#reproduce)
    - [Description](#description)
  - [2. SecIO handshake hangs](#2-secio-handshake-hangs)
    - [Reproduce:](#reproduce-1)
    - [JS logs](#js-logs)
    - [Rust logs](#rust-logs)

## Libp2p: Rust + JS
Attempt to connect libp2p in Rust with libp2p in JS

## Problems
### 1. PeerID
#### Reproduce:
```bash
# In a first terminal window/tab, execute
make rust 

# In a second terminal window/tab, execute
make js-rust-peerid
```
#### Description
- Rust gives PeerID in `Qm` multihash format: `QmTESkr2vWDCKqiHVsyvf4iRQCBgvNDqBJ6P3yTTDb6haw`
- JS says it's incorrect: `libp2p:error could not connect to discovered peer Error: dialed to the wrong peer, Ids do not match`
- Internally, JS derives the following PeerID from Rust's public key: `12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G`

### 2. SecIO handshake hangs
#### Reproduce:
```bash
# In a first terminal window/tab, execute
make rust 

# In a second terminal window/tab, execute
make js
```
#### JS logs
```
  libp2p:conn:out:QmSAmTQf dialing 12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G +0ms
  libp2p:conn:out:QmSAmTQf dialing transport TCP +1ms
  libp2p:switch:transport dialing TCP [ '/ip4/127.0.0.1/tcp/30000/ipfs/12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G' ] +32ms
  libp2p:switch:dialer dialMany:start +40ms
  libp2p:switch:dialer dialSingle: 12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G:/ip4/127.0.0.1/tcp/30000/ipfs/12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G +1ms
(node:84599) [DEP0079] DeprecationWarning: Custom inspection function on Objects via .inspect() is deprecated
  libp2p:switch:dialer:queue work:start +0ms
  libp2p:tcp:dial Connecting to 30000 127.0.0.1 +0ms
  libp2p:switch:dialer:queue work:success +3ms
  libp2p:switch:dialer dialMany:success +4ms
  libp2p:conn:out:QmSAmTQf successfully dialed 12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G +9ms
  mss:dialer	 (ee6tg1) dialer handle conn +0ms
  mss:dialer	 (ee6tg1) writing multicodec: /multistream/1.0.0 +0ms
  mss:dialer	 (ee6tg1) received ack: /multistream/1.0.0 +4ms
  mss:dialer	 (ee6tg1) handshake success +0ms
  libp2p:conn:out:QmSAmTQf selecting crypto /secio/1.0.0 to 12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G +6ms
  mss:dialer	 (ee6tg1) dialer select /secio/1.0.0 +1ms
  mss:dialer	 (ee6tg1) writing multicodec: /secio/1.0.0 +0ms
  mss:dialer	 (ee6tg1) received ack: /secio/1.0.0 +2ms
  libp2p:secio 1. propose - start +0ms
  libp2p:secio 1. propose - writing proposal +0ms
  libp2p:secio 1. propose - reading proposal <Buffer 0a 10 e4 45 11 3d 27 24 1c 03 a9 7b 79 ff e8 ef d1 cc 12 24 08 01 12 20 fe 62 28 54 de 53 5d 1a 5d a2 b8 7d fb 93 2d b3 b2 37 6f fc d2 43 ce cd 75 0b ... > +2ms
  libp2p:secio 1.1 identify +0ms
  libp2p:secio 1.1 identify - QmSAmTQf4nKgQypvNmq2XFvDFhnu4k8j1Jqo2wqUiaivff - identified remote peer as 12D3KooWSwNXzEeGjgwEocRJBzbdoDqxbz3LdrwgSuKmKeGvbM4G +1ms
  libp2p:secio 1.2 selection +0ms
  libp2p:secio 1. propose - finish +2ms
  libp2p:secio 2. exchange - start +0ms
  libp2p:secio 2. exchange - writing exchange +0ms
```

#### Rust logs
```
[2019-12-23T15:53:25Z TRACE multistream_select::protocol] Received message: Header(V1)
[2019-12-23T15:53:25Z TRACE multistream_select::protocol] Received message: Protocol(Protocol(b"/secio/1.0.0"))
[2019-12-23T15:53:25Z DEBUG multistream_select::listener_select] Listener: confirming protocol: /secio/1.0.0
[2019-12-23T15:53:25Z DEBUG multistream_select::listener_select] Listener: sent confirmed protocol: /secio/1.0.0
[2019-12-23T15:53:25Z DEBUG libp2p_secio] Starting secio upgrade
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] agreements proposition: P-256,P-384
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] ciphers proposition: AES-128,AES-256,TwofishCTR
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] digests proposition: SHA256,SHA512
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] starting handshake; local nonce = [228, 69, 17, 61, 39, 36, 28, 3, 169, 123, 121, 255, 232, 239, 209, 204]
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] sending proposition to remote
[2019-12-23T15:53:25Z TRACE tokio_io::framed_read] attempting to decode a frame
[2019-12-23T15:53:25Z TRACE tokio_io::framed_read] frame decoded from buffer
[2019-12-23T15:53:25Z DEBUG libp2p_secio::handshake] selected cipher: Aes128
[2019-12-23T15:53:25Z DEBUG libp2p_secio::handshake] selected hash: Sha256
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] received proposition from remote; pubkey = Rsa(PublicKey([48, 130, 1, 10, 2, 130, 1, 1, 0, 212, 41, 202, 110, 226, 110, 32, 177, 249, 0, 115, 233, 11, 35, 8, 73, 205, 182, 28, 240, 217, 58, 154, 214, 71, 110, 178, 63, 135, 92, 245, 87, 237, 21, 131, 213, 25, 213, 85, 146, 137, 207, 4, 160, 53, 176, 251, 16, 80, 99, 13, 182, 241, 55, 173, 72, 166, 231, 67, 55, 0, 95, 141, 176, 26, 2, 21, 3, 250, 15, 26, 220, 174, 114, 85, 5, 166, 238, 118, 87, 81, 251, 157, 38, 62, 234, 3, 119, 71, 172, 70, 91, 129, 149, 212, 7, 32, 229, 160, 149, 94, 188, 161, 33, 198, 37, 249, 241, 131, 117, 63, 39, 175, 239, 193, 23, 94, 49, 154, 121, 139, 252, 16, 130, 73, 191, 100, 161, 80, 147, 31, 20, 29, 8, 156, 107, 161, 95, 150, 80, 80, 155, 211, 173, 60, 175, 97, 215, 171, 78, 179, 24, 5, 143, 39, 95, 130, 53, 225, 216, 149, 192, 68, 213, 86, 223, 40, 31, 9, 140, 189, 77, 25, 75, 49, 216, 78, 153, 109, 142, 155, 126, 126, 44, 58, 69, 131, 39, 8, 140, 161, 233, 134, 134, 103, 23, 241, 131, 154, 239, 239, 173, 47, 173, 252, 98, 58, 60, 230, 49, 87, 237, 155, 42, 130, 132, 220, 238, 179, 101, 229, 52, 107, 137, 168, 165, 93, 248, 122, 90, 205, 173, 105, 37, 38, 24, 118, 145, 189, 83, 176, 207, 85, 49, 179, 80, 199, 141, 158, 207, 212, 128, 43, 124, 114, 219, 2, 3, 1, 0, 1])); nonce = [222, 187, 28, 207, 46, 171, 196, 16, 98, 6, 159, 200, 153, 28, 235, 17]
[2019-12-23T15:53:25Z TRACE libp2p_secio::handshake] sending exchange to remote
[2019-12-23T15:53:25Z TRACE tokio_io::framed_read] attempting to decode a frame
[2019-12-23T15:53:45Z DEBUG libp2p_core::transport::timeout] timeout elapsed for connection
[2019-12-23T15:53:45Z DEBUG libp2p_tcp] Dropped TCP connection to V4(127.0.0.1:49760)
something happened in swarm runloop
```