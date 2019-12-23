rust: ;RUST_LOG="trace,tokio_threadpool=info,tokio_reactor=info,mio=info" cargo run
npm-install: ;cd js; npm install
js:  npm-install; cd js ; DEBUG="*,-latency-monitor:LatencyMonitor,-libp2p:connection-manager" node run.js
js-rust-peerid: npm-install; cd js; DEBUG="*,-latency-monitor:LatencyMonitor,-libp2p:connection-manager" node run.js /ip4/127.0.0.1/tcp/30000/p2p/QmTESkr2vWDCKqiHVsyvf4iRQCBgvNDqBJ6P3yTTDb6haw