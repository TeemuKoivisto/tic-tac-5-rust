<script lang="ts">
  import { onMount } from 'svelte'
  // import Board from './Board/Board.svelte'
  import Loading from './Loading.svelte'
  import Lobby from './Lobby.svelte'
  import { GameStatus } from '@tt5/prototypes'

  import { gameActions, gameState, gameEnd, playerId } from '../stores/game'

  let endText = ''

  $: {
    const end = $gameEnd
    if (!end) {
    } else if (end.winner?.id === playerId) {
      endText = 'You won 🏆 :DD'
    } else if (end.winner?.id !== playerId) {
      endText = 'You lost 😵'
    } else if (end.result === GameStatus.TIE) {
      endText = 'Tie 🤦‍♂️'
      // } else if (end.result === GameStatus.AI_WON) {
      //   endText = 'Computer won 🤖'
    } else {
      endText = `Something weird happened: ${GameStatus}`
    }
  }

  onMount(() => {
    gameActions.runGame()
  })

  function handlePlayAgain() {
    gameActions.joinLobby()
  }
</script>

<main>
  {#if $gameState === 'waiting-game-start'}
    <div class="loading-menu">
      <h2>Waiting for players&hellip;</h2>
    </div>
  {:else if $gameState === 'lobby'}
    <Lobby />
  {:else if $gameState === 'game-ended'}
    <div class="end-menu">
      <marquee class="title" width="50%">Game ended</marquee>
      <p class="game-winner">{endText}</p>
      <button on:click={handlePlayAgain}>Play again</button>
    </div>
  {:else if $gameState === 'game-running'}
    Should be board
    <!-- <Board /> -->
  {:else if $gameState === 'connecting'}
    <Loading />
  {/if}
</main>

<style lang="scss">
  main {
    display: flex;
    justify-content: center;
    width: 100vw;
  }
  h1 {
    text-align: center;
  }
  .loading-menu {
    align-items: center;
    flex-direction: column;
    font-size: 1.25rem;
    position: fixed;
    top: 40%;
  }

  .end-menu {
    align-items: center;
    flex-direction: column;
    display: flex;
    font-size: 1.25rem;
    position: fixed;
    top: 35%;
  }

  .end-menu .title {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0;
    width: 140%;
  }

  .end-menu .game-winner {
    font-size: 1.25rem;
    margin: 3rem 0;
  }

  .end-menu button {
    cursor: pointer;
    font-size: 1rem;
    padding: 0.5rem 1rem;
    width: 100%;
  }
</style>
