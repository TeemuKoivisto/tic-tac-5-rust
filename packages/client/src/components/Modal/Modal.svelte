<script>
  import { fade, scale } from 'svelte/transition'
  import { modalActions, openModal, modals, EModal } from '../../stores/modal'

  import GameOverModal from './GameOver.svelte'

  const MODAL_DURATION = 400

  const components = {
    [EModal.GAME_OVER]: GameOverModal,
  }

  function handleOverlayClick() {
    modalActions.close()
  }
</script>

{#if $openModal}
  <div class="fixed z-40 w-full h-full top-0 left-0 flex items-center justify-center">
    <button
      transition:fade={{ duration: MODAL_DURATION }}
      class="fixed z-40 inset-0 h-full w-full bg-black bg-opacity-50 outline-none cursor-default"
      on:click={handleOverlayClick}
      tabindex="-1"
    />

    <div
      transition:scale={{ duration: MODAL_DURATION }}
      class="z-50 bg-gray-custom w-11/12 mx-auto rounded-xl shadow-lg overflow-y-auto md:max-w-md"
    >
      <div class="flex flex-col p-6 text-left">
        <svelte:component
          this={components[$openModal]}
          params={$modals[$openModal]}
          hideModal={() => modalActions.close()}
        />
      </div>
    </div>
  </div>
{/if}

<style>
</style>
