import { get, writable } from 'svelte/store'

enum Adjacency {
  Horizontal = 0,
  Vertical = 1,
  LeftToRightDiagonal = 2,
  RightToLeftDiagonal = 3,
}

interface Adjancies {
  hor: number
  ver: number
  left_diag: number
  right_diag: number
}

export interface Cell {
  x: number
  y: number
  owner: number
  adjacency: Adjancies
}

type Option<T> = { data: T } | undefined

class Board {
  size = 3
  cells: Cell[] = []

  constructor(size = 3, previous?: Cell[]) {
    this.size = size
    let cells: Cell[] = []
    if (previous) {
      cells = previous.map(c => Object.assign({}, c))
    } else {
      for (let y = 0; y < size; y += 1) {
        for (let x = 0; x < size; x += 1) {
          cells.push({
            x,
            y,
            owner: 0,
            adjacency: {
              hor: 0,
              ver: 0,
              left_diag: 0,
              right_diag: 0,
            },
          })
        }
      }
    }
    this.cells = cells
  }

  is_within_board(x: number, y: number) {
    return x >= 0 && y >= 0 && x < this.size && y < this.size
  }

  get_cell_at(x: number, y: number) {
    return this.cells[x + y * this.size]
  }

  set_cell_owner(x: number, y: number, player: number) {
    this.cells[x + y * this.size].owner = player
  }

  get_next_empty_cell(): Cell | undefined {
    let idx = 0
    let cell = this.cells[idx]
    while (cell.owner !== 0 || this.cells.length === idx) {
      idx += 1
      cell = this.cells[idx]
    }
    return cell
  }

  get_adjacent_in_direction(x: number, y: number, dir: Adjacency, topside: boolean): Option<Cell> {
    let xx = x
    let yy = y
    switch (dir) {
      case Adjacency.Horizontal:
        if (topside) {
          xx += 1
        } else {
          xx -= 1
        }
        break
      case Adjacency.Vertical:
        if (topside) {
          yy += 1
        } else {
          yy -= 1
        }
        break
      case Adjacency.LeftToRightDiagonal:
        if (topside) {
          xx -= 1
          yy += 1
        } else {
          xx += 1
          yy -= 1
        }
        break
      case Adjacency.RightToLeftDiagonal:
        if (topside) {
          xx += 1
          yy += 1
        } else {
          xx -= 1
          yy -= 1
        }
        break
    }
    if (this.is_within_board(xx, yy)) {
      return { data: this.get_cell_at(xx, yy) }
    }
    return undefined
  }

  get_adjacent_cells(placed: Cell, player: number, dir: Adjacency) {
    const adjacent = []
    let topside = true
    let now_x = placed.x
    let now_y = placed.y
    let iters = 0
    while (true) {
      const cell = this.get_adjacent_in_direction(now_x, now_y, dir, topside)
      if (iters > 20) {
        throw Error('infinite loop')
      }
      if (cell && cell.data.owner === player) {
        const c = cell.data
        adjacent.push(c)
        now_x = c.x
        now_y = c.y
      } else if (topside) {
        topside = false
        now_x = placed.x
        now_y = placed.y
      } else {
        break
      }
      iters += 1
    }
    return adjacent
  }

  check_is_full() {
    return this.cells.every(c => c.owner !== 0)
  }

  check_win_at(x: number, y: number) {
    const cell = this.get_cell_at(x, y)
    return Object.values(cell.adjacency).some(v => v === 5)
  }

  check_win() {
    return this.cells.find(c => Object.values(c.adjacency).some(v => v === 5))
  }

  get_available_moves(): Cell[] {
    return this.cells.filter(c => c.owner === 0)
  }

  create_board(player: number) {
    const newBoard = new Board(this.size, this.cells)
    const cell = this.get_next_empty_cell()
    if (cell) {
      newBoard.set_cell_owner(cell.x, cell.y, player)
      if (newBoard.check_win_at(cell.x, cell.y)) {
        return 10
      } else if (newBoard.check_is_full()) {
        return 0
      }
    }
    return 0
  }

  copy_board_with_cell_selected(cell: Cell, player: number) {
    const newBoard = new Board(this.size, this.cells)
    newBoard.set_cell_owner(cell.x, cell.y, player)
    return newBoard
  }
}

export const board = writable<Board>(new Board())
export const gridSize = writable(3)
export const player = writable<1 | 2>(1)
export const gameStatus = writable<'running' | 'x-won' | 'o-won' | 'tie'>('running')

interface Options {
  maxDepth: number
  aiPlayer: number
}

function minimax(
  selectedCell: Cell,
  initial: Board,
  depth: number,
  maximizingPlayer: boolean,
  opts: Options
) {
  // debugger
  const board = initial.copy_board_with_cell_selected(selectedCell, maximizingPlayer ? 1 : 2)
  const won = board.check_win_at(selectedCell.x, selectedCell.y)
  if (won || depth === 0) {
    return maximizingPlayer ? 10 : -10
  } else if (board.check_is_full()) {
    return 0
  }
  let value: number
  if (maximizingPlayer) {
    value = Number.POSITIVE_INFINITY
    initial.get_available_moves().forEach(c => {
      value = Math.max(value, minimax(c, board, depth - 1, false, opts))
    })
  } else {
    value = Number.NEGATIVE_INFINITY
    initial.get_available_moves().forEach(c => {
      value = Math.min(value, minimax(c, board, depth - 1, true, opts))
    })
  }
  return value
}

export const gameActions = {
  rematch() {},
  playerSelectCell(x: number, y: number) {
    const b = get(board)
    const cell = b.get_cell_at(x, y)
    if (cell.owner !== 0) {
      return
    }
    const playerNumber = get(player)
    b.set_cell_owner(cell.x, cell.y, playerNumber)
    let ended = true
    if (b.check_win_at(cell.x, cell.y)) {
      gameStatus.set('x-won')
    } else if (b.check_is_full()) {
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
    let aiMove: Cell | undefined
    let bestValue = Number.NEGATIVE_INFINITY
    const opts = {
      maxDepth: 10,
      aiPlayer: 2,
    }
    b.get_available_moves().forEach(c => {
      const value = minimax(c, b, 6, true, opts)
      if (bestValue < value) {
        aiMove = c
        bestValue = value
      }
    })
    if (!aiMove) {
      throw Error('no ai move found')
    }
    b.set_cell_owner(aiMove.x, aiMove.y, 2)
    if (b.check_win_at(aiMove.x, aiMove.y)) {
      gameStatus.set('o-won')
    } else if (b.check_is_full()) {
      gameStatus.set('tie')
    }
    board.set(b)
  },
}
