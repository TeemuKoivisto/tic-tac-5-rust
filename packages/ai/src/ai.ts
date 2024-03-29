// import { Board } from './board'
import { Board } from './board2'
import { Cell, getCellValue } from './cell'

interface Options {
  maxDepth: number
  humanPlayer: number
  aiPlayer: number
}

let iterations = 0

const cachedComputations = new Map<string, number>()

export function computeAi(b: Board, aiNumber: number, searchDepth: number) {
  let chosenCell: number | undefined
  // let chosenCell: Cell | undefined
  let bestValue = Number.NEGATIVE_INFINITY
  const opts = {
    maxDepth: 10,
    humanPlayer: aiNumber === 1 ? 2 : 1,
    aiPlayer: aiNumber,
  }
  iterations = 0
  const t0 = performance.now()
  b.get_available_moves().forEach(c => {
    const value = minimax(c, b, searchDepth, false, -Infinity, Infinity, aiNumber, opts)
    b.set_cell_owner(c, 0)
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
  // const aiMove = chosenCell
  console.log(`best: ${aiMove.x} ${aiMove.x} ${bestValue} at iterations ${iterations} \n`)
  b.set_cell_owner(chosenCell, aiNumber)
  return b.update_cell_adjancies(chosenCell, aiNumber)
}

export function minimax(
  cell: number,
  // cell: Cell,
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

  const won = board.update_cell_adjancies(cell, player)
  let value = NaN
  if (won) {
    value = opts.humanPlayer === player ? -1000 - depth : 1000 + depth
  } else if (board.is_full()) {
    value = opts.humanPlayer === player ? -100 - depth : 100 + depth
  } else if (depth === 0) {
    value = 0
  }
  if (!isNaN(value)) {
    // cachedComputations.set(board.code, value)
    return value
  }

  if (isMaximizing) {
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
  return value
}
