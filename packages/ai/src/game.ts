import { get, writable } from 'svelte/store'
import { Cell, getCellValue } from './cell'
// import { Board } from './board'
import { Board } from './board2'

export const board = writable<Board>(new Board())
export const gridSize = writable(3)
export const player = writable<'x' | 'o'>('x')
export const searchDepth = writable(7)
export const gameStatus = writable<'running' | 'x-won' | 'o-won' | 'tie'>('running')

let iterations = 0
let moves = 0
const debug = false

const cachedComputations = new Map<string, number>()

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
  cell: number,
  board: Board,
  depth: number,
  isMaximizing: boolean,
  alpha: number,
  beta: number,
  player: number,
  opts: Options
) {
  iterations += 1
  board.set_cell_owner(cell, player)
  const cached = cachedComputations.get(board.code)
  if (cached !== undefined) {
    return cached
  }
  const won = board.update_cell_adjancies(cell, player)
  let value: number
  if (won) {
    value = opts.humanPlayer === player ? -1000 - depth : 1000 + depth
  } else if (board.is_full()) {
    value = opts.humanPlayer === player ? -100 - depth : 100 + depth
  } else if (depth === 0) {
    value = 0
  } else if (isMaximizing) {
    value = Number.NEGATIVE_INFINITY
    board.get_available_moves().some(c => {
      value = Math.max(
        value,
        minimax(c, board, depth - 1, false, alpha, beta, player === 1 ? 2 : 1, opts)
      )
      alpha = Math.max(alpha, value)
      board.set_cell_owner(c, 0)
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
      board.set_cell_owner(c, 0)
      return beta <= alpha
    })
  }
  cachedComputations.set(board.code, value)
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
    const c = b.get_cell_at(x, y)
    const cell = b.get_cell_value_at(x, y)
    if (cell.owner !== 0) {
      return
    }
    const playerNumber = get(player) === 'x' ? 1 : 2
    let ended = true
    b.set_cell_owner(c, playerNumber)
    if (b.update_cell_adjancies(c, playerNumber)) {
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
    let chosenCell: number | undefined
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
      const cached = cachedComputations.get(b.code)
      let value: number
      if (cached !== undefined) {
        value = cached
      } else {
        value = minimax(c, b, get(searchDepth), false, -Infinity, Infinity, aiNumber, opts)
        cachedComputations.set(b.code, value)
        b.set_cell_owner(c, 0)
      }
      if (value > bestValue) {
        chosenCell = c
        bestValue = value
      }
    })
    if (!chosenCell) {
      throw Error('no ai move found')
    }
    const t1 = performance.now()
    console.log(
      `took ${Math.round(t1 - t0)} ms ${(Math.round(t1 - t0) / iterations).toPrecision(
        6
      )} per iteration`
    )
    const aiMove = getCellValue(chosenCell)
    console.log(`best: ${aiMove.x} ${aiMove.x} ${bestValue} at iterations ${iterations} \n`)
    b.set_cell_owner(chosenCell, aiNumber)
    if (b.update_cell_adjancies(chosenCell, aiNumber)) {
      gameStatus.set(aiNumber === 2 ? 'o-won' : 'x-won')
    } else if (b.is_full()) {
      gameStatus.set('tie')
    }
    console.log('board', b)
    board.set(b)
  },
}
