#!/usr/bin/env bash

ROOT_DIR="$(dirname $(readlink -f $0))"
export JWT_SECRET="very-secret"

run() {
  cd $ROOT_DIR
  RUST_LOG=server=warn cargo run -p server --release
  # cargo build -p tic-tac-5 && cargo run -p server --release
}

protos() {
  cd $ROOT_DIR
  # pb-rs --dont_use_cow --include . --output_directory crates/tic-tac-5/src/proto protos/*.proto
  pb-rs --dont_use_cow --output_directory crates/tic-tac-5/src/proto protos/*.proto
  protoc --plugin=./packages/prototypes/node_modules/.bin/protoc-gen-ts_proto \
    --ts_proto_opt=exportCommonSymbols=false \
    --ts_proto_out=./packages/prototypes/protos -Iprotos protos/*.proto
  pnpm --filter prototypes build
}

if [ -n "$1" ]; then
  case "$1" in
  run)
    run
    ;;
  protos)
    protos
    ;;
  docker:build)
    docker build -t liquid-war -f ./crates/server/Dockerfile .
    ;;
  docker:run)
    docker run -it --rm -p 6464:6464 liquid-war
    ;;
  docker:push)
    AWS_ACCOUNT_ID="626386600593"
    AWS_REGION="eu-central-1"
    ECR_REPOSITORY="tic-tac-5"
    REGISTRY_URL=${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com
    IMAGE=${REGISTRY_URL}/${ECR_REPOSITORY}
    VERSION=$(cat crates/server/Cargo.toml | grep version | awk -F'[ "]' 'NR==1{print $4}')

    aws ecr get-login-password --region ${AWS_REGION} | docker login \
      --username=AWS \
      ${REGISTRY_URL} \
      --password-stdin

    echo "Building & pushing image with version ${VERSION} and 'latest'"
    # Eg 626386600593.dkr.ecr.eu-west-1.amazonaws.com/lw-server:0.2.0
    docker buildx build --push \
      -f ./crates/server/Dockerfile \
      -t ${IMAGE}:${VERSION} \
      -t ${IMAGE}:latest .
    ;;
  *)
    echo $"Usage: $0 docker:build|docker:run|docker:push"
    exit 1
    ;;
  esac
fi
