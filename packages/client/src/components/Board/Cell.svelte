<script lang="ts">
  import { Cell, CellType } from '@tt5/prototypes'

  import { cells, gridSize } from '../../stores/game'

  export let y: number, x: number, handleCellClick: (x: number, y: number) => void

  let cell: Cell = $cells.get(`${x}:${y}`) as Cell

  cells.subscribe(cells => {
    let newVal = cells.get(`${x}:${y}`) as Cell
    if (newVal.cellType !== cell.cellType) {
      cell = newVal
    }
  })

  let cellY = y + 1
  let cellX = x + 1
  $: borderRight = cellX !== $gridSize && cellX % 3 !== 0
  $: borderRightBold = cellX !== $gridSize && cellX % 3 === 0
  $: borderBottom = cellY !== $gridSize && cellY % 3 !== 0
  $: borderBottomBold = cellY !== $gridSize && cellY % 3 === 0
</script>

<div class="cell border-r border-b relative">
  <button
    class="h-full w-full text-gray-800 absolute inset-0"
    data-cell={`${x}:${y}`}
    data-owner={cell}
    on:click={() => handleCellClick(x, y)}
  >
    <div class="h-full w-full flex items-center text-base justify-center p-0.5 text-black">
      {#if cell?.cellType === CellType.EMPTY}
        &nbsp;
      {:else if cell?.player === 0}
        X
        <!-- <svg
          stroke="currentColor"
          fill="none"
          stroke-width="2"
          viewBox="0 0 24 24"
          stroke-linecap="round"
          stroke-linejoin="round"
          xmlns="http://www.w3.org/2000/svg"
          ><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg
        > -->
      {:else if cell?.player === 1}
        O
        <!-- <svg
          stroke="currentColor"
          fill="none"
          stroke-width="2"
          viewBox="0 0 24 24"
          stroke-linecap="round"
          stroke-linejoin="round"
          xmlns="http://www.w3.org/2000/svg"><circle cx="12" cy="12" r="10" /></svg
        > -->
      {/if}
    </div>
  </button>
</div>

<style>
  .cell {
    @apply h-full w-full row-end-auto col-end-auto;
  }

  .cell-inner {
    @apply relative h-full w-full text-gray-800;
  }

  .cell-btn {
    @apply absolute inset-0 h-full w-full;
  }

  .cell-btn:focus {
    @apply outline-none;
  }

  .selected {
    @apply bg-primary text-white;
  }
</style>
