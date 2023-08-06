import { get, writable } from 'svelte/store'
import { Cell, Board } from './board'

export const board = writable<Board>(new Board(3))
export const gridSize = writable(3)
export const player = writable<'x' | 'o'>('x')
export const searchDepth = writable(6)
export const gameStatus = writable<'running' | 'x-won' | 'o-won' | 'tie'>('running')

let iterations = 0
let moves = 0
const debug = false

interface Options {
  maxDepth: number
  aiPlayer: number
}

function minimax(
  selectedCell: Cell,
  initial: Board,
  depth: number,
  player: number,
  isMaximizing: boolean,
  opts: Options
) {
  iterations += 1
  const board = new Board(initial.size, initial)
  board.update_cell_owner(selectedCell.x, selectedCell.y, player)
  if (board.check_win_at(selectedCell.x, selectedCell.y)) {
    return player === 1 ? -100 - depth : 100 + depth
  } else if (board.check_is_full() || depth === 0) {
    return player === 1 ? -10 - depth : 10 + depth
  }
  let value: number
  if (isMaximizing) {
    value = Number.NEGATIVE_INFINITY
    board.get_available_moves().forEach(c => {
      value = Math.max(value, minimax(c, board, depth - 1, player === 1 ? 2 : 1, false, opts))
    })
  } else {
    value = Number.POSITIVE_INFINITY
    board.get_available_moves().forEach(c => {
      value = Math.min(
        value,
        Math.min(value, minimax(c, board, depth - 1, player === 1 ? 2 : 1, true, opts))
      )
    })
  }
  return value
}

export const gameActions = {
  play(symbol: 'x' | 'o', gridsize = 3, depth = 6) {
    board.set(new Board(gridsize))
    player.set(symbol)
    searchDepth.set(depth)
    if (symbol === 'o') {
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
    console.log('select ', playerNumber)
    b.update_cell_owner(cell.x, cell.y, playerNumber)
    let ended = true
    if (b.check_win_at(cell.x, cell.y)) {
      gameStatus.set('x-won')
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
    const opts = {
      maxDepth: 10,
      aiPlayer: 2,
    }
    iterations = 0
    const aiNumber = get(player) === 'x' ? 2 : 1
    b.get_available_moves().forEach(c => {
      const value = minimax(c, b, 9, aiNumber, false, opts)
      if (value > bestValue) {
        aiMove = c
        bestValue = value
      }
    })
    if (!aiMove) {
      throw Error('no ai move found')
    }
    console.log(`best: ${aiMove.x} ${aiMove.y} ${bestValue} at iterations ${iterations} \n`)
    b.update_cell_owner(aiMove.x, aiMove.y, aiNumber)
    if (b.check_win_at(aiMove.x, aiMove.y)) {
      gameStatus.set('o-won')
    } else if (b.check_is_full()) {
      gameStatus.set('tie')
    }
    board.set(b)
  },
}
