#!/usr/bin/env bash

if [ -f .env ]; then
  export $(cat .env | xargs)
fi

ROOT_DIR="$(dirname $(readlink -f $0))"

run() {
  cd $ROOT_DIR
  cargo run -p server --release
  # cargo build -p tic-tac-5 && cargo run -p server --release
}

protos() {
  cd $ROOT_DIR
  # pb-rs --dont_use_cow --include . --output_directory crates/tic-tac-5/src/proto protos/*.proto
  pb-rs --dont_use_cow --output_directory crates/tic-tac-5/src/proto protos/*.proto
  protoc --plugin=./packages/prototypes/node_modules/.bin/protoc-gen-ts_proto \
    --ts_proto_opt=exportCommonSymbols=false \
    --ts_proto_opt=esModuleInterop=true \
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
  wasm)
    # RUSTFLAGS="-C target-feature=+simd128" cargo build -p wasm
    RUSTFLAGS="-C target-feature=+simd128" wasm-pack build ./crates/wasm --out-dir ../../packages/wasm
    ;;
  docker:build)
    docker build -t tic-tac-5 -f ./crates/server/Dockerfile .
    ;;
  docker:run)
    docker run -it --rm -p 6464:6464 tic-tac-5
    ;;
  docker:push)
    REGISTRY_URL=${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com
    IMAGE=${REGISTRY_URL}/${ECR_REPOSITORY}

    aws ecr get-login-password --region ${AWS_REGION} | docker login \
      --username=AWS \
      ${REGISTRY_URL} \
      --password-stdin

    echo "Building & pushing image with version ${VERSION} and 'latest'"
    # docker buildx prune --all # deletes all build cache
    # Eg 626386600593.dkr.ecr.eu-west-1.amazonaws.com/xsync-worker:0.2.0
    docker buildx build --push \
      -f ${IMAGE_PATH} \
      -t ${IMAGE}:${VERSION} \
      -t ${IMAGE}:latest  .
    ;;
  *)
    echo $"Usage: $0 docker:build|docker:run|docker:push"
    exit 1
    ;;
  esac
fi
