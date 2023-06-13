 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker run -dit -p 6060:6060 --name fennel-collator-2 us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
#docker exec -it fennel-protocol /app/target/release/fennel-node --collator --force-authoring --chain raw-parachain-chainspec.json --base-path /tmp/parachain/alice --port 40333 --ws-port 8844 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name Fennel0 -- --execution wasm --chain /app/polkadotspec.json --port 30343 --ws-port 9977 --bootnodes /ip4/192.168.10.30/tcp/30333/p2p/12D3KooWQpoPDFSH6TyzwgU2rrBXqNUq6Qkq3mZwzLTsRt9aVexF
