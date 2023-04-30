<script lang="ts">
  import { lastMove, wasOwnMove } from '../stores/game'
  // import { modalActions, EModal } from '../stores/modal'

  $: hideModal = $lastMove === undefined || !$wasOwnMove

  function onKeyDown(e: KeyboardEvent) {
    // if (e.key === 'Escape') {
    //   isOpen = !isOpen
    //   if (!isOpen) {
    //     modalActions.open(EModal.GAME_OVER, {
    //       playerWon: true,
    //       startTime: Date.now(),
    //       turns: 0,
    //     })
    //   } else {
    //     modalActions.close()
    //   }
    // }
  }
</script>

<svelte:window on:keydown|preventDefault={onKeyDown} />
<section class="h-0 relative">
  <div class="w-full z-20 flex items-center relative justify-center">
    <div class="menu transform flex flex-col justify-center" class:-translate-y-full={hideModal}>
      <h2 class="text-center text-lg my-1.5 text-black">It's opponent's turn</h2>
    </div>
  </div>
</section>

<style lang="scss">
  .overlay {
    @apply fixed z-20 inset-0 h-full w-full bg-black bg-opacity-50 outline-none cursor-default;
  }
  .menu {
    border-bottom-left-radius: 0.5rem;
    border-bottom-right-radius: 0.5rem;
    @apply z-30 py-2 w-80 bg-white shadow-xl transition duration-200 ease-in-out;
  }
</style>
