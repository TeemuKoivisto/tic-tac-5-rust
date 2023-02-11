#!/usr/bin/env bash
set -e

case "$1" in
  db:connect)
    DB_HOST=localhost
    DB_PORT=5700
    DB_USER=pg-user
    DB_PASSWORD=my-pg-password
    DB_NAME=my_postgres_db

    psql postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
    ;;
  docker:push)
    AWS_ACCOUNT_ID="626386600593"
    AWS_REGION="eu-central-1"
    ECR_REPOSITORY="tic-tac-5"
    REGISTRY_URL=${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com
    IMAGE=${REGISTRY_URL}/${ECR_REPOSITORY}
    VERSION=$(cat crates/worker/Cargo.toml | grep version | awk -F'[ "]' 'NR==1{print $4}')

    aws ecr get-login-password --region ${AWS_REGION} | docker login \
      --username=AWS \
      ${REGISTRY_URL} \
      --password-stdin

    echo "Building & pushing image with version ${VERSION} and 'latest'"
    # Eg 626386600593.dkr.ecr.eu-west-1.amazonaws.com/tic-tac-5:0.2.0
    docker buildx build --push \
      -f ./crates/server/Dockerfile \
      -t ${IMAGE}:${VERSION} \
      -t ${IMAGE}:latest  .
    ;;
  *)
    echo $"Usage: $0 db:connect|docker:push"
    exit 1
esac
