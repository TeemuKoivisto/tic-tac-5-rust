<script lang="ts">
  import { fade } from 'svelte/transition'

  import { Cell, CellType } from '@tt5/prototypes'

  import { cells, lastMove } from '../../stores/game'

  export let y: number, x: number, handleCellClick: (x: number, y: number) => void

  let cell: Cell = $cells.get(`${x}:${y}`) as Cell
  $: wasLastMove = $lastMove?.x === x && $lastMove?.y === y

  cells.subscribe(cells => {
    let newVal = cells.get(`${x}:${y}`) as Cell
    if (newVal.cellType !== cell.cellType) {
      cell = newVal
    }
  })
</script>

<div class={`${$$props.class || ''} cell h-full w-full row-end-auto col-end-auto relative`}>
  <button
    class="h-full w-full text-gray-800 absolute inset-0"
    class:selected={wasLastMove}
    class:text-black={!wasLastMove}
    on:click={() => handleCellClick(x, y)}
  >
    {#if cell?.cellType === CellType.PLAYER_CELL}
      <div
        transition:fade={{ duration: 400 }}
        class="h-full w-full flex items-center text-lg justify-center p-0.5 text-black"
      >
        {#if cell?.player === 1}
          X
        {:else if cell?.player === 2}
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
