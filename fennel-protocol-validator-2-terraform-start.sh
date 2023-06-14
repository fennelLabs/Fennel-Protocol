 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io nginx snapd
sudo snap install core && sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
gsutil cat gs://whiteflag-0-admin/fennel-nginx-conf-validator-2.sh > /etc/nginx/sites-enabled/default
sudo systemctl enable nginx
sudo systemctl start nginx
sudo certbot --nginx --non-interactive --agree-tos --email info@fennellabs.com --domains validator-2.fennellabs.com
sudo systemctl restart nginx
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker run -dt -p 30333:30333 -p 9944:9944 -p 9934:9934 --name fennel-validator-2 us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker exec fennel-validator-2 /app/target/release/polkadot key insert --base-path /tmp/relay/charlie --chain /app/chainspec.json --key-type babe --scheme sr25519 --suri $(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)
docker exec fennel-validator-2 /app/target/release/polkadot key insert --base-path /tmp/relay/charlie --chain /app/chainspec.json --key-type grandpa --scheme ed25519 --suri $(gsutil cat gs://whiteflag-0-admin/fennel-validator-2-key.sh)
docker exec fennel-validator-2 /app/target/release/polkadot --validator --base-path /tmp/relay/charlie --chain /app/chainspec.json --port 30333 --ws-port 9944 --rpc-port 9934 --bootnodes /ip4/$(gsutil cat gs://whiteflag-0-admin/fennel-protocol-boot-ip.sh)/tcp/30333/p2p/$(gsutil cat gs://whiteflag-0-admin/fennel-boot-node-id.sh) --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name FnlDot2 --rpc-external --ws-external --prometheus-external --rpc-cors all
