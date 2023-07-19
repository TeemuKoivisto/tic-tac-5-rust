<script lang="ts">
  import { onMount } from 'svelte'

  import Loading from './Loading.svelte'
  import Lobby from './Lobby.svelte'
  import Play from './Play.svelte'
  import Waiting from './Waiting.svelte'

  import { run } from '../stores/app'
  import { appState, AppState } from '../stores/state'

  onMount(() => {
    run()
  })
</script>

<main>
  {#if $appState === AppState.unauthenticated || $appState === AppState.connecting}
    <Loading />
  {:else if $appState === AppState.lobby}
    <Lobby />
  {:else if $appState === AppState.waiting_game_start}
    <Waiting />
  {:else if $appState === AppState.in_game}
    <Play />
  {:else if $appState === AppState.errored}
    The app has crashed 😵‍💫
  {/if}
</main>

<style lang="scss">
</style>
