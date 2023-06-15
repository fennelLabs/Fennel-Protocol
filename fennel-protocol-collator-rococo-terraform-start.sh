 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io nginx snapd
sudo snap install core && sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
gsutil cat gs://whiteflag-0-admin/fennel-nginx-conf-collator-rococo.sh > /etc/nginx/sites-enabled/default
sudo systemctl enable nginx
sudo systemctl start nginx
sudo certbot --nginx --non-interactive --agree-tos --email info@fennellabs.com --domains rococo.fennellabs.com
sudo systemctl restart nginx
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker pull us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
docker run -dt -p 40333:40333 -p 9944:9944 -p 30343:30343 -p 9977:9977 -p 9934:9934 --name fennel-collator-2 us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
docker exec fennel-collator-2 /app/target/release/fennel-node --collator --force-authoring --chain /app/raw-rococo-parachain-chainspec.json --base-path /tmp/parachain/alice --port 40333 --ws-port 9944 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --name Fennel1 --unsafe-ws-external --rpc-external --rpc-cors=all -- --execution wasm --chain /app/rococo.json --port 30343 --ws-port 9977 --rpc-port 9934 --rpc-external --ws-external --prometheus-external --rpc-cors all