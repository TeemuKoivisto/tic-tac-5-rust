export type GameState =
  | 'connecting'
  | 'lobby'
  | 'waiting-game-start'
  | 'game-running'
  | 'game-ended'

export interface Options {
  size: number
  players: number
}

export interface Cursor {
  player: number
  x: number
  y: number
}

export interface Move {
  x: number
  y: number
  targetX: number
  targetY: number
}
