<script lang="ts">
  import Board from './Board.svelte'

  import { gameStatus, gameActions } from '../game'
</script>

<main>
  <h1>tic-tac-toe</h1>
  <p>{$gameStatus}</p>
  <button on:click={gameActions.rematch}>Rematch</button>
  <div class="board">
    <Board />
  </div>
</main>

<style lang="scss">
  .board {
    @apply border;
  }
</style>
