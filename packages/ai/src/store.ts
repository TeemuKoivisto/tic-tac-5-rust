import { get, writable } from 'svelte/store'
import { compute_ai, Board } from 'wasm'
// import { computeAi } from './ai2'

export interface PlayOptions {
  symbol?: 'x' | 'o'
  size?: number
  maxDepth?: number
}

export const board = writable<Board>(new Board(3, 3))
export const gridSize = writable(3)
export const player = writable<'x' | 'o'>('x')
export const searchDepth = writable(6)
export const gameStatus = writable<'running' | 'x-won' | 'o-won' | 'tie'>('running')

export const gameActions = {
  play(opts: PlayOptions) {
    const { symbol, size, maxDepth } = opts
    const prevSymbol = get(player)
    const prevSize = get(gridSize)
    const newSize = size ?? prevSize
    board.set(new Board(newSize, newSize === 5 ? 4 : 3))
    if (size !== undefined) gridSize.set(size)
    if (symbol !== undefined) player.set(symbol)
    if (maxDepth !== undefined) searchDepth.set(maxDepth)
    if ((symbol || prevSymbol) === 'o') {
      this.evaluateAiMove()
    }
  },
  playerSelectCell(x: number, y: number) {
    const b = get(board)
    const cell = b.get_cell_at(x, y)
    if (cell.owner !== 0) {
      return
    }
    const playerNumber = get(player) === 'x' ? 1 : 2
    let ended = true
    if (b.select_cell(cell.x, cell.y, playerNumber)) {
      gameStatus.set(playerNumber === 1 ? 'x-won' : 'o-won')
    } else if (b.is_full()) {
      gameStatus.set('tie')
    } else {
      ended = false
    }
    board.set(b)
    if (!ended) {
      this.evaluateAiMove()
    }
  },
  evaluateAiMove() {
    const b = get(board)
    const aiNumber = get(player) === 'x' ? 2 : 1
    const aiWon = compute_ai(b, aiNumber, get(searchDepth))
    if (aiWon) {
      gameStatus.set(aiNumber === 2 ? 'o-won' : 'x-won')
    } else if (b.is_full()) {
      gameStatus.set('tie')
    }
    console.log('board', b)
    board.set(b)
  },
}
