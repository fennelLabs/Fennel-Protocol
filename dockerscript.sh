#!/bin/bash
# For those days when you can't be asked to type.
docker build -t us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest .
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker push us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/fennel-protocol:latest
# Then you can just run `./dockerscript.sh` and it will build and push your image to the container registry. Magic.
