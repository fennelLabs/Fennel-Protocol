 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io nginx snapd
sudo snap install core && sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
gsutil cat gs://whiteflag-0-admin/fennel-nginx-conf-validator.sh > /etc/nginx/sites-enabled/default
sudo systemctl enable nginx
sudo systemctl start nginx
sudo certbot --nginx --non-interactive --agree-tos --email info@fennellabs.com --domains relay-beta.fennellabs.com
sudo systemctl restart nginx
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker stop fennel-validator-1
docker rm fennel-validator-1
docker run -dt -p 30333:30333 -p 9944:9944 -p 9934:9934 --name fennel-validator-1 us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot-protocol:latest
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type babe --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type imon --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type para --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type asgn --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type audi --scheme sr25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type beef --scheme ecdsa --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot key insert --base-path /tmp/relay/bob --chain /app/chainspec.json --key-type gran --scheme ed25519 --suri "$(gsutil cat gs://whiteflag-0-admin/fennel-validator-key.sh)"
docker exec fennel-validator-1 /app/polkadot --validator --base-path /tmp/relay/bob --chain /app/chainspec.json --port 30333  --rpc-port 9944 --bootnodes /ip4/34.138.37.11/tcp/30333/p2p/12D3KooWLj8FgbwDtLQbA5ozWqqx6vM1n2rmjwQMczSiDveyWa5G --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name FnlDot1 --rpc-external --prometheus-external --rpc-cors all
