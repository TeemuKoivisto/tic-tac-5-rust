# tic-tac-5

In Rust!

https://flourishing-cendol-f5a0f2.netlify.app/

```sh
# Add axum::rejection=trace for logging Axum rejections https://docs.rs/axum/latest/axum/extract/index.html#logging-rejections
RUST_LOG=server=trace,tower_http=trace ./ex.sh run

# In separate terminal
pnpm utils # run once
pnpm cli
```
