 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
docker run -dt -p 40333:40333 -p 8844:8844 -p 30343:30343 -p 9977:9977 -p 9934:9934 --name fennel-collator-2 us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
docker exec fennel-collator-2 /app/target/release/fennel-node --collator --force-authoring --chain raw-parachain-chainspec.json --base-path /tmp/parachain/alice --port 40333 --ws-port 8844 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name Fennel0 --bootnodes  /ip4/$(gsutil cat gs://whiteflag-0-admin/fennel-protocol-collator-1-ip.sh)/tcp/40333/p2p/$(gsutil cat gs://whiteflag-0-admin/fennel-para-node-id.sh) -- --execution wasm --chain /app/polkadotspec.json --port 30343 --ws-port 9977 --rpc-port 9934 --bootnodes /ip4/$(gsutil cat gs://whiteflag-0-admin/fennel-protocol-boot-ip.sh)/tcp/30333/p2p/$(gsutil cat gs://whiteflag-0-admin/fennel-boot-node-id.sh)
