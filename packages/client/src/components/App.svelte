<script lang="ts">
  import { onMount } from 'svelte'
  import { PlayerAppState } from '@tt5/prototypes'

  import Loading from './Loading.svelte'
  import Lobby from './Lobby.svelte'
  import Play from './Play.svelte'
  import Waiting from './Waiting.svelte'

  import { run } from '../stores/app'
  import { appState } from '../stores/state'

  onMount(() => {
    run()
  })
</script>

<main>
  {#if $appState === PlayerAppState.initializing || $appState === PlayerAppState.disconnected}
    <Loading />
  {:else if $appState === PlayerAppState.lobby}
    <Lobby />
  {:else if $appState === PlayerAppState.waiting_game_start}
    <Waiting />
  {:else if $appState === PlayerAppState.in_game}
    <Play />
  {:else if $appState === PlayerAppState.errored}
    The app has crashed 😵‍💫
  {/if}
</main>

<style lang="scss">
</style>
