<script lang="ts">
  import { onMount } from 'svelte'
  import Board from './Board/Board.svelte'
  import GameOver from './GameOver.svelte'
  import Loading from './Loading.svelte'
  import Lobby from './Lobby.svelte'
  import Waiting from './Waiting.svelte'

  import { gameActions, gameState, gameEnd, playerId } from '../stores/game'

  onMount(() => {
    gameActions.runGame()
  })
</script>

<main>
  {#if $gameState === 'waiting-game-start'}
    <Waiting />
  {:else if $gameState === 'lobby'}
    <Lobby />
  {:else if $gameState === 'game-ended'}
    <GameOver gameEnd={$gameEnd} {playerId} />
  {:else if $gameState === 'game-running'}
    <Board />
  {:else if $gameState === 'connecting'}
    <Loading />
  {/if}
</main>

<style lang="scss">
</style>
