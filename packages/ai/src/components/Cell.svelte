<script lang="ts">
  import { fade } from 'svelte/transition'
  import { BoardCell } from 'wasm'

  import { board } from '../store'

  export let y: number, x: number, handleCellClick: (x: number, y: number) => void

  let cell: BoardCell = $board.get_cell_at(x, y)

  board.subscribe(b => {
    const found = b.get_cell_at(x, y)
    if (found) cell = found
  })
</script>

<div class={`${$$props.class || ''} cell h-full w-full row-end-auto col-end-auto relative`}>
  <button
    class="h-full w-full text-gray-800 absolute inset-0"
    on:click={() => handleCellClick(x, y)}
  >
    {#if cell.owner !== 0}
      <div
        transition:fade={{ duration: 400 }}
        class="h-full w-full flex items-center text-lg justify-center p-0.5 text-black text-9xl"
      >
        {#if cell.owner === 1}
          X
        {:else if cell.owner === 2}
          O
        {/if}
      </div>
    {/if}
  </button>
</div>

<style>
  .selected {
    background: #eff5ff;
  }
</style>
