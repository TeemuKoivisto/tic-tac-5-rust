import {
  Adjacency,
  Adjancies,
  Cell,
  createCell,
  getCellValue,
  getCellCoords,
  getCellCoordsOwner,
  setAdjacency,
  getOwner,
  setOwner,
  setDirAdjacency,
} from './cell'

export type Option<T> = { data: T } | undefined

interface BoardOptions {
  gridSize?: number
  inRow?: number
}

export class Board {
  size = 3
  inRow = 3
  cells: number[] = []
  available = 3 * 3

  constructor(opts?: BoardOptions, previous?: Board) {
    this.size = previous?.size ?? opts?.gridSize ?? this.size
    this.inRow = previous?.inRow ?? opts?.inRow ?? this.inRow
    this.available = previous?.available ?? this.size * this.size
    let cells: number[] = []
    if (previous) {
      cells = previous.cells.map(c => c)
    } else {
      for (let y = 0; y < this.size; y += 1) {
        for (let x = 0; x < this.size; x += 1) {
          cells.push(createCell(x, y))
        }
      }
    }
    this.cells = cells
  }

  is_within_board(x: number, y: number) {
    return x >= 0 && y >= 0 && x < this.size && y < this.size
  }

  get_cell_idx(c: number) {
    return 0b111111 & (c + (0b111111 & ((c >> 6) * this.size)))
  }

  get_cell_at(x: number, y: number) {
    return this.cells[x + y * this.size]
  }

  get_cell_value_at(x: number, y: number) {
    return getCellValue(this.cells[x + y * this.size])
  }

  set_cell_owner(c: number, player: number) {
    const idx = this.get_cell_idx(c)
    this.cells[idx] = setOwner(this.cells[idx], player)
    if (player !== 0) {
      this.available -= 1
    } else {
      this.available += 1
    }
  }

  get_next_empty_cell(): number | undefined {
    if (this.available === 0) {
      return undefined
    }
    let idx = 0
    let cell = this.cells[idx]
    while (getOwner(cell) !== 0 || this.cells.length === idx) {
      idx += 1
      cell = this.cells[idx]
    }
    return cell
  }

  get_adjacent_in_direction(
    x: number,
    y: number,
    dir: Adjacency,
    topside: boolean
  ): number | undefined {
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
      return this.cells[xx + yy * this.size]
    }
    return undefined
  }

  get_adjacent_cells(c: number, player: number, dir: Adjacency) {
    const adjacent: number[] = []
    let topside = true
    let now_x = 0b111111 & c
    let now_y = 0b111111 & (c >> 6)
    let iters = 0
    let cell: number | undefined
    while (true) {
      cell = this.get_adjacent_in_direction(now_x, now_y, dir, topside)
      if (cell !== undefined && getOwner(cell) === player) {
        adjacent.push(c)
        now_x = 0b111111 & cell
        now_y = 0b111111 & (cell >> 6)
      } else if (topside) {
        topside = false
        now_x = 0b111111 & c
        now_y = 0b111111 & (c >> 6)
      } else {
        break
      }
      if (iters > 20) {
        throw Error('infinite loop')
      }
      iters += 1
    }
    return adjacent
  }

  update_cell_owner(c: number, player: number) {
    this.set_cell_owner(c, player)
    let bestInRow = 0
    let idx = 0
    for (let i = 0; i < 4; i += 1) {
      const dir = i as Adjacency
      const cells = this.get_adjacent_cells(c, player, dir)
      const adjacent_count = cells.length + 1
      for (const c of cells) {
        idx = this.get_cell_idx(c)
        this.cells[idx] = setDirAdjacency(this.cells[idx], dir, adjacent_count)
      }
      idx = this.get_cell_idx(c)
      this.cells[idx] = setDirAdjacency(this.cells[idx], dir, adjacent_count)
      if (adjacent_count > bestInRow) {
        bestInRow = adjacent_count
      }
    }
    return bestInRow === this.inRow
  }

  is_full() {
    return this.available === 0
  }

  get_available_moves(): number[] {
    return this.cells.filter(c => getOwner(c) === 0)
  }
}
