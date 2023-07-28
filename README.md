# tic-tac-5

In Rust!

https://flourishing-cendol-f5a0f2.netlify.app/

```sh
# Add axum::rejection=trace for logging Axum rejections https://docs.rs/axum/latest/axum/extract/index.html#logging-rejections
# Copy .env-example to .env
# Then run the server:
./ex.sh run

# In separate terminal
pnpm utils # run once
pnpm cli
```

## TODO

- remove expired sessions
- fix sometimes full lobby games fail to start -> reused senders in SessionHandle?
- allow leaving games when waiting on disconnected
- send AppState as first payload from server when connected -> lobby | in_game
- remove game correctly when all have left & game still running
- fix not showing WaitingPlayer modal when refreshed when it's open
- better error messages (eg when establishing ws connection with 403)
- generic GameMenu modal that opens with escape, allows leaving / setting settings or whatever
