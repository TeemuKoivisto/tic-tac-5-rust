# tic-tac-5

I've always wondered how to make a multiplayer game. So I decided to try using Rust+tokio for the game engine and server, Svelte for the web client.

When I grew up it was somewhat common to play "jätkänsakki" with notebooks that used nice square grid. As I started to make this game, I noted there wasn't an equivalent name in English although there's a Japanese game called [Gomoku](https://en.wikipedia.org/wiki/Gomoku) which is exactly the same. It's translated to "Five in a Row" but I think TicTac5 is much better name for it.

## How to run

```sh
# Add axum::rejection=trace for logging Axum rejections https://docs.rs/axum/latest/axum/extract/index.html#logging-rejections
# Copy .env-example to .env
# Then run the server:
./ex.sh run

# In separate terminal
pnpm utils # run once
pnpm cli
```

## Architecture

I am using Rust for the backend as well as the game engine. The events between client/server are encoded in binary protobuf messages since I wasn't interested in making my own encoder. At the start I used mutexes for synchronization but quickly realised that was dumb and switched to actors for that lockless concurrency baby.

I've prototyped a WASM version but it's quite silly since the logic is so simple and the binary is quite big and bulky to load. But it's doable! Also I've tried to implement AI but that's still WIP.

For hosting, I am using a cheap Hetzner server that has some other stuff running in it as well. They're behind a Caddy reverse proxy under a CNAME but that setup I've not made public. Maybe some day I'll be able to host this on itch but let's see. And the client runs now on Cloudflare.

## TODO

- fix sometimes full lobby games fail to start -> reused senders in SessionHandle?
- allow leaving games when waiting on disconnected
- send AppState as first payload from server when connected -> lobby | in_game
- remove game correctly when all have left & game still running
- fix not showing WaitingPlayer modal when refreshed when it's open
- better error messages (eg when establishing ws connection with 403)
- generic GameMenu modal that opens with escape, allows leaving / setting settings or whatever
