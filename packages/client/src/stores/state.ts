import { PlayerAppState, PlayerInGameState } from '@tt5/prototypes'
import { derived, get, writable } from 'svelte/store'
import { log } from '../logger'

import type { Result } from '@tt5/types'

// export enum PlayerAppState {
//   'unauthenticated',
//   'connecting',
//   'lobby',
//   'waiting_game_start',
//   'in_game',
//   'errored',
// }

// export enum PlayerInGameState {
//   'disconnected',
//   'your_turn',
//   'opponent_turn',
//   'waiting_opponent',
//   'paused',
//   'ended',
// }

export const appState = writable<PlayerAppState>(PlayerAppState.initializing)
export const prevAppState = writable<PlayerAppState>(PlayerAppState.initializing)
export const gameState = writable<PlayerInGameState>(PlayerInGameState.paused)

const appTransitions: { [key in PlayerAppState]: PlayerAppState[] } = {
  [PlayerAppState.initializing]: [PlayerAppState.disconnected, PlayerAppState.errored],
  [PlayerAppState.disconnected]: [PlayerAppState.lobby, PlayerAppState.errored],
  [PlayerAppState.lobby]: [
    PlayerAppState.disconnected,
    PlayerAppState.waiting_game_start,
    PlayerAppState.in_game,
    PlayerAppState.errored,
  ],
  [PlayerAppState.waiting_game_start]: [PlayerAppState.disconnected, PlayerAppState.in_game],
  [PlayerAppState.in_game]: [PlayerAppState.lobby],
  [PlayerAppState.errored]: [PlayerAppState.disconnected],
  [PlayerAppState.UNRECOGNIZED]: [],
}

const gameTransitions: { [key in PlayerInGameState]: PlayerInGameState[] } = {
  [PlayerInGameState.not_started]: [
    PlayerInGameState.x_turn,
    PlayerInGameState.o_turn,
    PlayerInGameState.ended,
  ],
  [PlayerInGameState.x_turn]: [
    PlayerInGameState.waiting_player,
    PlayerInGameState.o_turn,
    PlayerInGameState.paused,
    PlayerInGameState.ended,
  ],
  [PlayerInGameState.o_turn]: [
    PlayerInGameState.waiting_player,
    PlayerInGameState.x_turn,
    PlayerInGameState.paused,
    PlayerInGameState.ended,
  ],
  [PlayerInGameState.waiting_player]: [
    PlayerInGameState.not_started,
    PlayerInGameState.x_turn,
    PlayerInGameState.o_turn,
    PlayerInGameState.paused,
    PlayerInGameState.ended,
  ],
  [PlayerInGameState.paused]: [
    PlayerInGameState.waiting_player,
    PlayerInGameState.not_started,
    PlayerInGameState.x_turn,
    PlayerInGameState.o_turn,
    PlayerInGameState.ended,
  ],
  [PlayerInGameState.ended]: [],
  [PlayerAppState.UNRECOGNIZED]: [],
}

export const stateActions = {
  transitApp(to: PlayerAppState): Result<undefined> {
    const currentApp = get(appState)
    const available = appTransitions[currentApp]
    log.debug(`app transition from ${PlayerAppState[currentApp]} to ${PlayerAppState[to]}`)
    if (!available.includes(to)) {
      console.error(`Not a valid app state transition! from ${currentApp} to ${to}`)
      return { err: `Not a valid app state transition! from ${currentApp} to ${to}`, code: 500 }
    }
    prevAppState.set(currentApp)
    appState.set(to)
    if (to === PlayerAppState.in_game) {
      gameState.set(PlayerInGameState.paused)
    }
    return { data: undefined }
  },
  transitGame(to: PlayerInGameState): Result<undefined> {
    const currentApp = get(appState)
    const currentGame = get(gameState)
    log.debug(`game transition from ${PlayerInGameState[currentGame]} to ${PlayerInGameState[to]}`)
    if (currentApp !== PlayerAppState.in_game) {
      console.error(`App was not in in_game state! ${currentApp}`)
      return { err: `App was not in in_game state! ${currentApp}`, code: 500 }
    }
    const available = gameTransitions[currentGame]
    if (!available.includes(to)) {
      console.error(`Not a valid game state transition! from ${currentGame} to ${to}`)
      return { err: `Not a valid game state transition! from ${currentGame} to ${to}`, code: 500 }
    }
    gameState.set(to)
    return { data: undefined }
  },
}
