import { get, writable } from 'svelte/store'
import { Board } from './board'

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
export const selectedCell = writable([-1, -1])

export const gameActions = {
  play(opts: PlayOptions) {},
  playerSelectCell(x: number, y: number) {
    const b = get(board)
    const selected = get(selectedCell)
    if (selected[0] === -1 && selected[1] === -1) {
      selectedCell.set([x, y])
    } else if (x === selected[0] && y === selected[1]) {
      selectedCell.set([-1, -1])
    } else if (b.is_valid_move(x, y)) {
      const won = b.move_piece(selected[0], selected[1], x, y)
      console.log('mvoed', b)
      board.set(b)
      selectedCell.set([-1, -1])
      if (!won) {
        this.evaluateAiMove()
      }
    }
  },
  evaluateAiMove() {},
}
