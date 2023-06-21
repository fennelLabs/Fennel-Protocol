 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io nginx snapd
sudo snap install core && sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
gsutil cat gs://whiteflag-0-admin/fennel-nginx-conf-boot.sh > /etc/nginx/sites-enabled/default
sudo systemctl enable nginx
sudo systemctl start nginx
sudo certbot --nginx --non-interactive --agree-tos --email info@fennellabs.com --domains fennel-relay-1.fennellabs.com
sudo systemctl restart nginx
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker run -dt -p 9944:9944 -p 30333:30333 -p 9934:9934 --name relay-boot us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker exec relay-boot /app/target/release/polkadot key insert --base-path /tmp/relay/alice --chain /app/chainspec.json --key-type babe --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
docker exec relay-boot /app/target/release/polkadot key insert --base-path /tmp/relay/alice --chain /app/chainspec.json --key-type gran --scheme ed25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-boot-key.sh)"
docker exec relay-boot /app/target/release/polkadot --validator --base-path /tmp/relay/alice --chain /app/chainspec.json --port 30333 --ws-port 9944 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --name FnlDot0 --node-key $(gsutil cat gs://whiteflag-0-admin/fennel-node-key.sh) --rpc-cors all --prometheus-external --rpc-methods Unsafe --rpc-external --ws-external -lparachain::collator-protocol=trace