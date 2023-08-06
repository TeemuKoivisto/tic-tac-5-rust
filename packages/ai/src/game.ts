import { get, writable } from 'svelte/store'
import { Cell, Board } from './board'

export const board = writable<Board>(new Board())
export const gridSize = writable(3)
export const player = writable<'x' | 'o'>('x')
export const searchDepth = writable(6)
export const gameStatus = writable<'running' | 'x-won' | 'o-won' | 'tie'>('running')

let iterations = 0
let moves = 0
const debug = false

interface PlayOptions {
  symbol?: 'x' | 'o'
  size?: number
  maxDepth?: number
}

interface Options {
  maxDepth: number
  humanPlayer: number
  aiPlayer: number
}

function minimax(
  selectedCell: Cell,
  board: Board,
  depth: number,
  isMaximizing: boolean,
  alpha: number,
  beta: number,
  player: number,
  opts: Options
) {
  iterations += 1
  board.update_cell_owner(selectedCell.x, selectedCell.y, player)
  if (board.check_win_at(selectedCell.x, selectedCell.y)) {
    return opts.humanPlayer === player ? -100 - depth : 100 + depth
  } else if (board.check_is_full() || depth === 0) {
    return opts.humanPlayer === player ? -10 - depth : 10 + depth
  }
  let value: number
  if (isMaximizing) {
    value = Number.NEGATIVE_INFINITY
    board.get_available_moves().some(c => {
      value = Math.max(
        value,
        minimax(c, board, depth - 1, false, alpha, beta, player === 1 ? 2 : 1, opts)
      )
      alpha = Math.max(alpha, value)
      board.set_cell_owner(c.x, c.y, 0)
      return beta <= alpha
    })
  } else {
    value = Number.POSITIVE_INFINITY
    board.get_available_moves().some(c => {
      value = Math.min(
        value,
        Math.min(value, minimax(c, board, depth - 1, true, alpha, beta, player === 1 ? 2 : 1, opts))
      )
      beta = Math.min(beta, value)
      board.set_cell_owner(c.x, c.y, 0)
      return beta <= alpha
    })
  }
  return value
}

export const gameActions = {
  play(opts: PlayOptions) {
    const { symbol, size, maxDepth } = opts
    const prevSymbol = get(player)
    const prevSize = get(gridSize)
    const newSize = size ?? prevSize
    board.set(new Board({ gridSize: newSize, inRow: newSize === 5 ? 4 : 3 }))
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
    b.update_cell_owner(cell.x, cell.y, playerNumber)
    let ended = true
    if (b.check_win_at(cell.x, cell.y)) {
      gameStatus.set(playerNumber === 1 ? 'x-won' : 'o-won')
    } else if (b.check_is_full()) {
      gameStatus.set('tie')
    } else {
      ended = false
    }
    board.set(b)
    moves += 1
    if (!ended) {
      this.evaluateAiMove()
    }
  },
  evaluateAiMove() {
    const b = get(board)
    let aiMove: Cell | undefined
    let bestValue = Number.NEGATIVE_INFINITY
    const aiNumber = get(player) === 'x' ? 2 : 1
    const opts = {
      maxDepth: 10,
      humanPlayer: aiNumber === 1 ? 2 : 1,
      aiPlayer: aiNumber,
    }
    iterations = 0
    const t0 = performance.now()
    b.get_available_moves().forEach(c => {
      const value = minimax(c, b, get(searchDepth), false, -Infinity, Infinity, aiNumber, opts)
      b.set_cell_owner(c.x, c.y, 0)
      if (value > bestValue) {
        aiMove = c
        bestValue = value
      }
    })
    if (!aiMove) {
      throw Error('no ai move found')
    }
    const t1 = performance.now()
    console.log(
      `took ${Math.round(t1 - t0)} ms ${(Math.round(t1 - t0) / iterations).toPrecision(
        6
      )} per iteration`
    )
    console.log(`best: ${aiMove.x} ${aiMove.y} ${bestValue} at iterations ${iterations} \n`)
    b.update_cell_owner(aiMove.x, aiMove.y, aiNumber)
    if (b.check_win_at(aiMove.x, aiMove.y)) {
      gameStatus.set(aiNumber === 2 ? 'o-won' : 'x-won')
    } else if (b.check_is_full()) {
      gameStatus.set('tie')
    }
    board.set(b)
  },
}
