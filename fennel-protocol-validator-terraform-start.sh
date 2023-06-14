 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker run -dt -p 30333:30333 -p 9944:9944 -p 9934:9934 --name fennel-validator-1 us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker exec fennel-validator-1 /app/target/release/polkadot --validator --base-path /tmp/relay/bob --chain /app/chainspec.json --port 30333 --ws-port 9944 --rpc-port 9934 --bootnodes /ip4/$(gsutil cat gs://whiteflag-0-admin/fennel-protocol-boot-ip.sh)/tcp/30333/p2p/$(gsutil cat gs://whiteflag-0-admin/fennel-boot-node-id.sh) --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name FnlDot1
