<script lang="ts">
  import Board from './Board.svelte'

  import { gameStatus, gameActions } from '../game'
</script>

<main>
  <div class="items-center flex flex-col">
    <h1 class="my-12 text-5xl font-bold flex items-center">TicTacToe</h1>
    <p class="text-2xl">{$gameStatus}</p>
    <div class="w-full max-w-3xl">
      <div class="mt-12 flex justify-center">
        <button
          class="btn focus:outline-none hover:bg-blue-700"
          on:click={() => gameActions.play('x')}>Play as X</button
        >
        <button
          class="ml-4 btn focus:outline-none hover:bg-blue-700"
          on:click={() => gameActions.play('o')}>Play as O</button
        >
      </div>
    </div>
  </div>
  <Board />
</main>

<style lang="scss">
</style>
