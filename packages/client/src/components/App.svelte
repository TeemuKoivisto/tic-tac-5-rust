<script lang="ts">
  import { onMount } from 'svelte'

  import Loading from './Loading.svelte'
  import Lobby from './Lobby.svelte'
  import Play from './Play.svelte'
  import Waiting from './Waiting.svelte'

  import { gameActions, gameState } from '../stores/game'

  onMount(() => {
    gameActions.runGame()
  })
</script>

<main>
  {#if $gameState === 'waiting-game-start'}
    <Waiting />
  {:else if $gameState === 'lobby'}
    <Lobby />
  {:else if $gameState === 'game-running' || $gameState === 'game-ended'}
    <Play />
  {:else if $gameState === 'connecting'}
    <Loading />
  {/if}
</main>

<style lang="scss">
</style>
