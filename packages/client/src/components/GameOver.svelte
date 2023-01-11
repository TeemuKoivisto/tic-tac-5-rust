<script lang="ts">
  import { GameEnd, GameStatus } from '@tt5/prototypes'

  import { gameActions } from '../stores/game'

  export let gameEnd: GameEnd | undefined, playerId: number

  let endText = ''

  $: {
    if (!gameEnd) {
    } else if (gameEnd.winner?.id === playerId) {
      endText = 'You won 🏆 :DD'
    } else if (gameEnd.winner?.id !== playerId) {
      endText = 'You lost 😵'
    } else if (gameEnd.result === GameStatus.TIE) {
      endText = 'Tie 🤦‍♂️'
      // } else if (end.result === GameStatus.AI_WON) {
      //   endText = 'Computer won 🤖'
    } else {
      endText = `Something weird happened: ${JSON.stringify(gameEnd)}`
    }
  }

  function handlePlayAgain() {
    gameActions.joinLobby()
  }
</script>

<section
  class="my-32 p-4 h-full m-auto lg:container md:p-16 md:pt-8 xs:p-8 rounded-2xl flex flex-col items-center justify-center"
>
  <h1 class="my-3 text-5xl font-bold flex items-center">TicTac5</h1>
  <div class="mt-24 flex flex-col items-center justify-center">
    <marquee class="text-3xl my-8" width="100%">Game ended</marquee>
    <p class="text-lg my-8">{endText}</p>
    <button
      class="btn w-full focus:outline-none hover:bg-blue-700 hover:no-underline hover:-translate-y-1 hover:scale-110"
      on:click={handlePlayAgain}>Play again</button
    >
  </div>
</section>

<style lang="scss">
</style>
