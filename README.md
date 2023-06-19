# tic-tac-5

In Rust!

https://flourishing-cendol-f5a0f2.netlify.app/

# todo

- move game_manager stuff into context
- remove mutextes for game_manager & connection_manager

- create player_context for each connection
- keeps a ref to the joined game instance -> no need for locking game_manager again
- game also keeps refs to the joined players' connections
- each tick & broadcast will only lock connections that belong that game -> no more locking of the whole connection_manager
