<script lang="ts">
  import { fade } from 'svelte/transition'

  import { board, selectedCell } from '../store'

  export let y: number, x: number, handleCellClick: (x: number, y: number) => void

  $: cell = $board.get_cell_at(x, y)
  let isOdd = y % 2 === 0 ? x % 2 === 0 : x % 2 === 1
  $: sign = cell.piece === 'knight' ? 'N' : cell.piece.charAt(0).toUpperCase()
  $: image = `${cell.owner === 1 ? 'b' : 'w'}${sign}.png`
  $: selected = x === $selectedCell[0] && y === $selectedCell[1]
</script>

<div
  class={`${$$props.class || ''} cell h-full w-full bg-gray-400 row-end-auto col-end-auto relative`}
  class:bg-gray-500={isOdd}
  class:bg-gray-600={selected}
>
  <button
    class="h-full w-full text-gray-800 absolute inset-0"
    on:click={() => handleCellClick(x, y)}
  >
    {#if cell.owner !== 0}
      <div
        transition:fade={{ duration: 400 }}
        class="h-full w-full flex items-center text-lg justify-center p-0.5 text-black text-9xl"
      >
        {#if cell.owner !== 0}
          <img src={image} alt="Piece icon" />
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
