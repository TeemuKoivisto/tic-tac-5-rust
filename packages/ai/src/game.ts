import { get, writable } from 'svelte/store'

export enum Adjacency {
  Horizontal = 0,
  Vertical = 1,
  LeftToRightDiagonal = 2,
  RightToLeftDiagonal = 3,
}

interface Adjancies {
  [Adjacency.Horizontal]: number
  [Adjacency.Vertical]: number
  [Adjacency.LeftToRightDiagonal]: number
  [Adjacency.RightToLeftDiagonal]: number
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
  available = 3 * 3

  constructor(size = 3, previous?: Board) {
    this.size = previous?.size ?? size
    this.available = previous?.available ?? size * size
    let cells: Cell[] = []
    if (previous) {
      cells = previous.cells.map(c => Object.assign({}, c))
    } else {
      for (let y = 0; y < size; y += 1) {
        for (let x = 0; x < size; x += 1) {
          cells.push({
            x,
            y,
            owner: 0,
            adjacency: {
              [Adjacency.Horizontal]: 0,
              [Adjacency.Vertical]: 0,
              [Adjacency.LeftToRightDiagonal]: 0,
              [Adjacency.RightToLeftDiagonal]: 0,
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
    this.available -= 1
  }

  get_next_empty_cell(): Cell | undefined {
    if (this.available === 0) {
      return undefined
    }
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

  get_adjacent_cells(x: number, y: number, player: number, dir: Adjacency) {
    const adjacent = []
    let topside = true
    let now_x = x
    let now_y = y
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
        now_x = x
        now_y = y
      } else {
        break
      }
      iters += 1
    }
    return adjacent
  }

  update_cell_owner(x: number, y: number, player: number) {
    this.set_cell_owner(x, y, player)
    const adjancies: Adjancies = {
      [Adjacency.Horizontal]: 0,
      [Adjacency.Vertical]: 0,
      [Adjacency.LeftToRightDiagonal]: 0,
      [Adjacency.RightToLeftDiagonal]: 0,
    }
    for (const adj in Adjacency) {
      const dir: Adjacency = Number(adj)
      if (!isNaN(dir)) {
        const cells = this.get_adjacent_cells(x, y, player, dir)
        const adjacent_count = cells.length + 1
        for (const c of cells) {
          this.cells[c.x + c.y * this.size].adjacency[dir] = adjacent_count
        }
        adjancies[dir] = adjacent_count
      }
    }
    this.cells[x + y * this.size].adjacency = adjancies
  }

  check_is_full() {
    return this.available === 0
  }

  check_win_at(x: number, y: number) {
    const cell = this.get_cell_at(x, y)
    return Object.values(cell.adjacency).some(v => v === 3)
  }

  check_win() {
    return this.cells.find(c => Object.values(c.adjacency).some(v => v === 3))
  }

  get_available_moves(): Cell[] {
    return this.cells.filter(c => c.owner === 0)
  }
}

export const board = writable<Board>(new Board(3))
export const gridSize = writable(3)
export const player = writable<1 | 2>(1)
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
  rematch() {},
  playerSelectCell(x: number, y: number) {
    const b = get(board)
    const cell = b.get_cell_at(x, y)
    if (cell.owner !== 0) {
      return
    }
    const playerNumber = get(player)
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
    b.get_available_moves().forEach(c => {
      const value = minimax(c, b, 9, 2, false, opts)
      if (value > bestValue) {
        aiMove = c
        bestValue = value
      }
    })
    if (!aiMove) {
      throw Error('no ai move found')
    }
    console.log(`best: ${aiMove.x} ${aiMove.y} ${bestValue} at iterations ${iterations} \n`)
    b.update_cell_owner(aiMove.x, aiMove.y, 2)
    if (b.check_win_at(aiMove.x, aiMove.y)) {
      gameStatus.set('o-won')
    } else if (b.check_is_full()) {
      gameStatus.set('tie')
    }
    board.set(b)
  },
}
