import { derived, get, writable } from 'svelte/store'
import { log } from '../logger'

import type { Result } from '@tt5/types'

export enum AppState {
  'unauthenticated',
  'connecting',
  'lobby',
  'waiting_game_start',
  'in_game',
  'errored',
}

export enum GameState {
  'disconnected',
  'your_turn',
  'opponent_turn',
  'waiting_opponent',
  'paused',
  'ended',
}

export const appState = writable<AppState>(AppState.unauthenticated)
export const prevAppState = writable<AppState>(AppState.unauthenticated)
export const gameState = writable<GameState>(GameState.paused)

const appTransitions: { [key in AppState]: AppState[] } = {
  [AppState.unauthenticated]: [AppState.connecting, AppState.errored],
  [AppState.connecting]: [AppState.lobby, AppState.errored],
  [AppState.lobby]: [
    AppState.connecting,
    AppState.waiting_game_start,
    AppState.in_game,
    AppState.errored,
  ],
  [AppState.waiting_game_start]: [AppState.connecting, AppState.in_game],
  [AppState.in_game]: [AppState.lobby],
  [AppState.errored]: [AppState.connecting],
}

const gameTransitions: { [key in GameState]: GameState[] } = {
  [GameState.disconnected]: [GameState.your_turn, GameState.opponent_turn, GameState.ended],
  [GameState.your_turn]: [
    GameState.disconnected,
    GameState.opponent_turn,
    GameState.ended,
    GameState.paused,
  ],
  [GameState.opponent_turn]: [
    GameState.disconnected,
    GameState.your_turn,
    GameState.ended,
    GameState.paused,
  ],
  [GameState.waiting_opponent]: [
    GameState.disconnected,
    GameState.your_turn,
    GameState.opponent_turn,
    GameState.ended,
    GameState.paused,
  ],
  [GameState.paused]: [
    GameState.disconnected,
    GameState.your_turn,
    GameState.opponent_turn,
    GameState.ended,
  ],
  [GameState.ended]: [GameState.disconnected],
}

export const stateActions = {
  transitApp(to: AppState): Result<undefined> {
    const currentApp = get(appState)
    const available = appTransitions[currentApp]
    if (!available.includes(to)) {
      return { err: `Not a valid app state transition! from ${currentApp} to ${to}`, code: 500 }
    }
    prevAppState.set(currentApp)
    appState.set(to)
    if (to === AppState.in_game) {
      gameState.set(GameState.paused)
    }
    return { data: undefined }
  },
  transitGame(to: GameState): Result<undefined> {
    const currentApp = get(appState)
    const currentGame = get(gameState)
    log.debug(`transition from ${currentGame} to ${to}`)
    if (currentApp !== AppState.in_game) {
      return { err: `App was not in in_game state! ${currentApp}`, code: 500 }
    }
    const available = gameTransitions[currentGame]
    if (!available.includes(to)) {
      return { err: `Not a valid game state transition! from ${currentGame} to ${to}`, code: 500 }
    }
    gameState.set(to)
    return { data: undefined }
  },
}
