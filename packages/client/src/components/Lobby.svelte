<script lang="ts">
  import { authActions, playerName } from '../stores/auth'
  import { lobbyGames, gameActions } from '../stores/game'

  let size: number | undefined = undefined,
    players = 2

  function handleSetName(
    e: Event & {
      currentTarget: EventTarget & HTMLInputElement
    }
  ) {
    authActions.setPlayerName(e.currentTarget.value)
  }

  function handleCreateSubmit() {
    gameActions.createGame({
      size: size || 0,
      players,
    })
  }
  function handleJoinGame(idx: number) {
    const g = $lobbyGames[idx]
    gameActions.joinGame(g, {
      size: size || 0,
      players,
    })
  }
</script>

<section
  class="my-32 p-4 h-full m-auto lg:container md:p-16 md:pt-8 xs:p-8 rounded-2xl flex flex-col items-center justify-center"
>
  <h1 class="my-3 text-5xl font-bold flex items-center">TicTac5</h1>
  <div class="mt-24 flex flex-col md:flex-row items-center justify-center">
    <form class="w-52 flex flex-col" on:submit|preventDefault={handleCreateSubmit}>
      <div class="col-field">
        <label for="name">Name</label>
        <input
          id="name"
          class="w-full py-2 px-2 bg-01 text-0B bg-zinc-600 rounded"
          value={$playerName}
          on:input={handleSetName}
        />
      </div>
      <div class="mt-6 w-full">
        <button
          class="btn w-full focus:outline-none hover:bg-blue-700 hover:no-underline hover:-translate-y-1 hover:scale-110"
        >
          Play against Human
        </button>
      </div>
    </form>
    <div class="w-52 mt-12 md:ml-16 flex justify-center">
      {#if $lobbyGames.length === 0}
        <h3>No games</h3>
      {:else}
        <ul class="w-full">
          {#each $lobbyGames as game, idx}
            <li>
              <div>
                <button
                  class="btn w-full focus:outline-none hover:bg-blue-700"
                  on:click={() => handleJoinGame(idx)}
                  >Players: {game.players}/{game.maxPlayers}</button
                >
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  </div>
</section>

<style lang="scss">
  .field {
    @apply flex items-center;
  }
  .field > label {
    @apply mr-2;
  }
  .field + .field {
    @apply mt-2;
  }
  .col-field {
    @apply flex flex-col;
  }
  .col-field + .col-field {
    @apply mt-2;
  }
  .col-field > label {
    @apply mb-2;
  }
  li + li {
    @apply mt-4;
  }
</style>
