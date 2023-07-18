<script lang="ts">
  import type { PlayerDisconnectParams } from '../../stores/modal'
  // import { gameActions } from '../../stores/game'

  export let params: PlayerDisconnectParams
  export const hideModal: () => void = () => undefined

  const { playerName } = params
  let waited_s = 0

  setInterval(() => {
    waited_s += 1
  }, 1000)
</script>

<div class="flex flex-col text-center items-center">
  <h1 class="text-4xl font-semibold text-black">Waiting for X player {playerName}</h1>
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
      <div class="w-1/2 text-left text-black">Waited:</div>
      <div class="w-1/2 text-right text-black">
        {waited_s}
      </div>
    </div>
  </div>
</div>

<button class="btn mt-4 w-full focus:outline-none hover:bg-blue-700">Leave to lobby</button>

<style>
</style>
