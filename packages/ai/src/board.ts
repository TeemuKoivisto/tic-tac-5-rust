export enum Adjacency {
  Horizontal = 0,
  Vertical = 1,
  LeftToRightDiagonal = 2,
  RightToLeftDiagonal = 3,
}

export interface Adjancies {
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

export type Option<T> = { data: T } | undefined

interface BoardOptions {
  gridSize?: number
  inRow?: number
}

export class Board {
  size = 3
  inRow = 3
  cells: Cell[] = []
  available = 3 * 3

  constructor(opts?: BoardOptions, previous?: Board) {
    this.size = previous?.size ?? opts?.gridSize ?? this.size
    this.inRow = previous?.inRow ?? opts?.inRow ?? this.inRow
    this.available = previous?.available ?? this.size * this.size
    let cells: Cell[] = []
    if (previous) {
      cells = previous.cells.map(c => Object.assign({}, c))
    } else {
      for (let y = 0; y < this.size; y += 1) {
        for (let x = 0; x < this.size; x += 1) {
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
    if (player !== 0) {
      this.available -= 1
    } else {
      this.available += 1
    }
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
    let cell: Option<Cell>
    while (true) {
      cell = this.get_adjacent_in_direction(now_x, now_y, dir, topside)
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
    for (let i = 0; i < 4; i += 1) {
      const dir = i as Adjacency
      const cells = this.get_adjacent_cells(x, y, player, dir)
      const adjacent_count = cells.length + 1
      for (const c of cells) {
        this.cells[c.x + c.y * this.size].adjacency[dir] = adjacent_count
      }
      this.cells[x + y * this.size].adjacency[dir] = adjacent_count
    }
  }

  is_full() {
    return this.available === 0
  }

  check_win_at(x: number, y: number) {
    const cell = this.get_cell_at(x, y)
    return Object.values(cell.adjacency).some(v => v === this.inRow)
  }

  check_win() {
    return this.cells.find(c => Object.values(c.adjacency).some(v => v === this.inRow))
  }

  get_available_moves(): Cell[] {
    return this.cells.filter(c => c.owner === 0)
  }
}
