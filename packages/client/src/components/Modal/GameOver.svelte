<script lang="ts">
  import type { GameOverParams } from '../../stores/modal'
  import { gameActions } from '../../stores/game'
  import { GameStatus } from '@tt5/prototypes'

  export let params: GameOverParams
  export const hideModal: () => void = () => undefined

  const { player, result, startTime, turns } = params
  const elapsed = Date.now() / 1000 - startTime
  const min = Math.floor(elapsed / 60) % 60
  const sec = Math.floor(elapsed) % 60
  const timer = `${min < 10 ? '0' : ''}${min}:${sec < 10 ? '0' : ''}${sec}`
  const elapsedTurns = turns
  let titleText: string
  if (
    (result === GameStatus.X_WON && player.symbol === 'X') ||
    (result == GameStatus.O_WON && player.symbol === 'O')
  ) {
    titleText = 'You won!'
  } else if (result === GameStatus.X_WON || result == GameStatus.O_WON) {
    titleText = 'You lost'
  } else if (result === GameStatus.TIE) {
    titleText = 'Draw'
  } else if (
    (result === GameStatus.X_TURN && player.symbol === 'X') ||
    (result == GameStatus.O_TURN && player.symbol === 'O')
  ) {
    titleText = 'It is your turn'
  } else if (result === GameStatus.X_TURN || result == GameStatus.O_TURN) {
    titleText = "It is opponent's turn"
  } else {
    titleText = 'Game still in progress'
  }

  function handleRematch() {}
  function handleNewGame() {
    gameActions.createGame({
      size: 0,
      players: 2,
    })
  }
  function handleOpenLobby() {
    gameActions.joinLobby()
  }
</script>

<div class="flex flex-col text-center items-center">
  <h1 class="text-4xl font-semibold text-black">{titleText}</h1>
</div>

<div class="mt-6 mb-10 w-full text-xl space-y-2">
  <div class="flex text-black">
    <div class="flex-auto mr-3">
      <svg
        class="w-8 h-8 inline-block"
        xmlns="http://www.w3.org/2000/svg"
        fill="#222"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
    </div>

    <div class="flex-grow w-full flex flex-wrap border-b-2 border-gray-300 pb-1">
      <div class="w-1/2 text-left text-black">Time:</div>
      <div class="w-1/2 text-right text-black">
        {timer}
      </div>
    </div>
  </div>

  <div class="flex">
    <div class="flex-auto mr-3">
      <svg
        class="w-8 h-8 inline-block"
        xmlns="http://www.w3.org/2000/svg"
        fill="#222"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
        />
      </svg>
    </div>

    <div class="flex-grow w-full flex flex-wrap border-b-2 border-gray-300 pb-1">
      <div class="w-1/2 text-left text-black">Turns:</div>
      <div class="w-1/2 text-right text-black">
        {elapsedTurns}
      </div>
    </div>
  </div>
</div>

<button class="btn w-full focus:outline-none hover:bg-blue-700" on:click={handleRematch}
  >Request rematch</button
>
<button class="btn mt-4 w-full focus:outline-none hover:bg-blue-700" on:click={handleNewGame}
  >New game</button
>
<button class="btn mt-4 w-full focus:outline-none hover:bg-blue-700" on:click={handleOpenLobby}
  >Lobby</button
>

<style>
</style>
