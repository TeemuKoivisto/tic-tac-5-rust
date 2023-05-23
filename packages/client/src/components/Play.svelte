<script lang="ts">
  import { onMount } from 'svelte'
  import { modalActions, EModal } from '../stores/modal'

  import AboveModal from './AboveModal.svelte'
  import Board from './Board/Board.svelte'
  import Modal from './Modal/Modal.svelte'

  function handleKeydown(event: KeyboardEvent & { currentTarget: EventTarget & Window; }) {
    if (event.key === "Escape") {
      modalActions.toggle(EModal.GAME_OVER)
    }
	}
  
  onMount(() => {
    console.log('mount play')
    return () => {
      console.log('destroy play')
    }
  })
</script>

<!-- <svelte:window on:keydown={handleKeydown}/> -->
<AboveModal />
<Board />
<Modal />

<style lang="scss">
</style>
