 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker run -dit -p 6060:6060 --name subservice us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
docker exec -it fennel-protocol /app/target/release/fennel-node --dev --tmp --unsafe-ws-external --rpc-external --prometheus-external --rpc-methods unsafe --rpc-cors all
