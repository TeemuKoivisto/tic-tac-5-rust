import { get, writable } from 'svelte/store'
import { Board } from './Board'

export interface PlayOptions {
  symbol?: 'x' | 'o'
  size?: number
  maxDepth?: number
}

export const board = writable<Board>(new Board())
export const gridSize = writable(8)
export const player = writable<'x' | 'o'>('x')
export const searchDepth = writable(6)
export const gameStatus = writable<'running' | 'x-won' | 'o-won' | 'tie'>('running')

export const gameActions = {
  play(opts: PlayOptions) {},
  playerSelectCell(x: number, y: number) {},
  evaluateAiMove() {},
}
