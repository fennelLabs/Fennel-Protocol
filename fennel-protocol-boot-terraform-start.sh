 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker run -t -p 9944:9944 -p 30333:30333 --name relay-boot us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker exec -t relay-boot /app/target/release/polkadot --validator --base-path /tmp/relay/alice --chain /app/chainspec.json --port 30333 --ws-port 9944 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name FnlDot0 --node-key $(gsutil cat gs://whiteflag-0-admin/fennel-node-key.sh)
