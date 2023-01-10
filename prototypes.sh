#!/usr/bin/env bash

pnpm --filter prototypes gen && pnpm --filter prototypes build
pb-rs --dont_use_cow -o crates/tic-tac-5/src/proto/proto_all.rs packages/prototypes/protos/proto_all.proto