import { get, writable } from 'svelte/store'
import { Cell } from './cell'
// import { Board } from './board'
import { Board } from './board2'

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
  coords: [number, number],
  board: Board,
  depth: number,
  isMaximizing: boolean,
  alpha: number,
  beta: number,
  player: number,
  opts: Options
) {
  iterations += 1
  // console.log('cell', selectedCell)
  if (board.update_cell_owner(coords[0], coords[1], player)) {
    return opts.humanPlayer === player ? -1000 - depth : 1000 + depth
  } else if (board.is_full()) {
    return opts.humanPlayer === player ? -100 - depth : 100 + depth
  } else if (depth === 0) {
    return 0
  }
  let value: number
  if (isMaximizing) {
    value = Number.NEGATIVE_INFINITY
    board.available_moves.some(c => {
      value = Math.max(
        value,
        minimax(c, board, depth - 1, false, alpha, beta, player === 1 ? 2 : 1, opts)
      )
      alpha = Math.max(alpha, value)
      board.update_cell_owner(c[0], c[1], 0)
      return beta <= alpha
    })
  } else {
    value = Number.POSITIVE_INFINITY
    board.available_moves.some(c => {
      value = Math.min(
        value,
        Math.min(value, minimax(c, board, depth - 1, true, alpha, beta, player === 1 ? 2 : 1, opts))
      )
      beta = Math.min(beta, value)
      board.update_cell_owner(c[0], c[1], 0)
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
    const cell = b.get_cell_value_at(x, y)
    if (cell.owner !== 0) {
      return
    }
    const playerNumber = get(player) === 'x' ? 1 : 2
    let ended = true
    if (b.update_cell_owner(cell.x, cell.y, playerNumber)) {
      gameStatus.set(playerNumber === 1 ? 'x-won' : 'o-won')
    } else if (b.is_full()) {
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
    let aiMove: [number, number] | undefined
    let bestValue = Number.NEGATIVE_INFINITY
    const aiNumber = get(player) === 'x' ? 2 : 1
    const opts = {
      maxDepth: 10,
      humanPlayer: aiNumber === 1 ? 2 : 1,
      aiPlayer: aiNumber,
    }
    iterations = 0
    const t0 = performance.now()
    b.available_moves.forEach(c => {
      const value = minimax(c, b, get(searchDepth), false, -Infinity, Infinity, aiNumber, opts)
      b.update_cell_owner(c[0], c[1], 0)
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
    console.log(`best: ${aiMove[0]} ${aiMove[1]} ${bestValue} at iterations ${iterations} \n`)
    if (b.update_cell_owner(aiMove[0], aiMove[1], aiNumber)) {
      gameStatus.set(aiNumber === 2 ? 'o-won' : 'x-won')
    } else if (b.is_full()) {
      gameStatus.set('tie')
    }
    console.log('board', b)
    board.set(b)
  },
}
