<script lang="ts">
  import Cell from './Cell.svelte'
  import { gridSize, gameActions } from '../../stores/game'

  $: {
    if (typeof document !== undefined) {
      try {
        document.documentElement.style.setProperty(`--grid-size`, `${$gridSize}`)
      } catch (e) {}
    }
  }

  function handleCellClick(x: number, y: number) {
    gameActions.playerSelectCell(x, y)
  }
</script>

<div class="board-padding relative z-10 mt-8">
  <div class="max-w-3xl relative">
    <div class="w-full" style="padding-top: 100%" />
  </div>
  <div class="board-padding absolute inset-0 flex justify-center">
    <div class="bg-white shadow-2xl rounded-xl w-full h-full max-w-3xl flex flex-col">
      {#each Array($gridSize) as _, y}
        <div class="row flex">
          {#each Array($gridSize) as _, x}
            <Cell {y} {x} {handleCellClick} />
          {/each}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  :root {
    --grid-size: 12;
  }
  .board-padding {
    @apply px-4 pb-4;
  }
  .row {
    height: calc(100% / var(--grid-size));
  }
</style>
