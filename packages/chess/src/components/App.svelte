<script lang="ts">
  import Board from './Board.svelte'

  import { gameStatus, gameActions } from '../store'
</script>

<main>
  <div class="items-center flex flex-col">
    <h1 class="my-12 text-5xl font-bold flex items-center">Chess</h1>
    <p class="text-2xl">{$gameStatus}</p>
    <div class="w-full max-w-3xl">
      <div class="mt-12 flex justify-center">
        <button
          class="btn focus:outline-none hover:bg-blue-700"
          on:click={() => gameActions.play({ symbol: 'x' })}>Play as X</button
        >
        <button
          class="ml-4 btn focus:outline-none hover:bg-blue-700"
          on:click={() => gameActions.play({ symbol: 'o' })}>Play as O</button
        >
        <button
          class="ml-4 btn focus:outline-none hover:bg-blue-700"
          on:click={() => gameActions.play({ size: 3 })}>Size 3</button
        >
        <button
          class="ml-4 btn focus:outline-none hover:bg-blue-700"
          on:click={() => gameActions.play({ size: 5 })}>Size 5</button
        >
      </div>
    </div>
  </div>
  <Board />
</main>

<style lang="scss">
</style>
