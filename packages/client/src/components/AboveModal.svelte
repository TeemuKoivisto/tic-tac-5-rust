<script lang="ts">
  import { fade } from 'svelte/transition'

  import { lastMove, wasOwnMove } from '../stores/game'
  import { modalActions, EModal } from '../stores/modal'

  $: hideModal = $lastMove === undefined || !$wasOwnMove

  export let isOpen = false
  const ANIMATION_DURATION = 400

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      isOpen = !isOpen
      if (!isOpen) {
        modalActions.open(EModal.GAME_OVER, {
          playerWon: true,
          startTime: Date.now(),
          turns: 0,
        })
      } else {
        modalActions.close()
      }
    }
  }
</script>

<svelte:window on:keydown|preventDefault={onKeyDown} />
<section
  class="w-full flex items-center absolute justify-center z-20"
  class:hidden={hideModal}
  class:-translate-y-full={hideModal}
>
  {#if wasOwnMove}
    <button
      transition:fade={{ duration: ANIMATION_DURATION }}
      class="dropdown-overlay"
      tabindex="-1"
    />
  {/if}
  <div
    class="dropdown-menu transform flex flex-col justify-center"
    class:-translate-y-full={hideModal}
  >
    <h2 class="text-center text-lg my-1.5 text-black">It's opponent's turn</h2>
  </div>
</section>

<style lang="scss">
  .dropdown-overlay {
    @apply fixed z-20 inset-0 h-full w-full bg-black bg-opacity-50 outline-none cursor-default;
  }
  .dropdown-menu {
    border-bottom-left-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
    @apply z-30 py-2 w-80 bg-white shadow-xl transition duration-200 ease-in-out;
  }
</style>
