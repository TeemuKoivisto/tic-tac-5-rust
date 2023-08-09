<script lang="ts">
  import Cell from './Cell.svelte'
  import { gridSize, gameActions } from '../store'

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

<div class="h-screen flex justify-center items-center">
  <div class="w-full relative z-10">
    <div class="max-w-3xl relative">
      <div class="w-full" style="padding-top: 100%" />
    </div>
    <div class="absolute inset-0 flex justify-center">
      <div class="bg-white shadow-2xl rounded-xl w-full h-full max-w-3xl max-h-3xl flex flex-col">
        {#each Array($gridSize) as _, y}
          <div class="row flex">
            {#each Array($gridSize) as _, x}
              <Cell class={`${y < $gridSize - 1 ? 'border-b' : ''}`} {y} {x} {handleCellClick} />
            {/each}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  :root {
    --grid-size: 12;
  }
  .board-padding {
    @apply px-4 pb-4;
  }
  .row {
    height: calc(100% / var(--grid-size));
  }
  :global(.cell) + :global(.cell) {
    @apply border-l;
  }
</style>
