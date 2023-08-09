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
  available_moves: [number, number][] = []

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
    this.available_moves = cells.filter(c => getOwner(c) === 0).map(c => getCellCoords(c))
  }

  is_within_board(x: number, y: number) {
    return x >= 0 && y >= 0 && x < this.size && y < this.size
  }

  get_cell_at(x: number, y: number) {
    return this.cells[x + y * this.size]
  }

  get_cell_value_at(x: number, y: number) {
    return getCellValue(this.cells[x + y * this.size])
  }

  // set_cell_owner(cell: number, player: number) {
  //   this.cells[x + y * this.size] = setOwner(this.cells[x + y * this.size], player)
  //   if (player !== 0) {
  //     this.available -= 1
  //   } else {
  //     this.available += 1
  //   }
  // }

  set_cell_owner(x: number, y: number, player: number) {
    this.cells[x + y * this.size] = setOwner(this.cells[x + y * this.size], player)
    if (player !== 0) {
      this.available -= 1
      this.available_moves = this.available_moves.filter(([xx, yy]) => xx !== x || yy !== y)
    } else {
      this.available += 1
      this.available_moves.push(getCellCoords(this.cells[x + y * this.size]))
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

  get_adjacent_cells(x: number, y: number, player: number, dir: Adjacency) {
    const adjacent: [number, number, number][] = []
    let topside = true
    let now_x = x
    let now_y = y
    let iters = 0
    let cell: number | undefined
    let c: [number, number, number] | undefined
    while (true) {
      cell = this.get_adjacent_in_direction(now_x, now_y, dir, topside)
      if (iters > 20) {
        throw Error('infinite loop')
      }
      c = cell !== undefined ? getCellCoordsOwner(cell) : undefined
      if (c && c[2] === player) {
        adjacent.push(c)
        now_x = c[0]
        now_y = c[1]
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
    let bestInRow = 0
    for (let i = 0; i < 4; i += 1) {
      const dir = i as Adjacency
      const cells = this.get_adjacent_cells(x, y, player, dir)
      const adjacent_count = cells.length + 1
      for (const c of cells) {
        this.cells[c[0] + c[1] * this.size] = setDirAdjacency(
          this.cells[c[0] + c[1] * this.size],
          dir,
          adjacent_count
        )
      }
      this.cells[x + y * this.size] = setDirAdjacency(
        this.cells[x + y * this.size],
        dir,
        adjacent_count
      )
      if (adjacent_count > bestInRow) {
        bestInRow = adjacent_count
      }
    }
    return bestInRow === this.inRow
  }

  is_full() {
    return this.available === 0
  }

  // check_win_at(x: number, y: number) {
  //   const cell = this.get_cell_at(x, y)
  //   return Object.values(cell.adjacency).some(v => v === this.inRow)
  // }

  // check_win() {
  //   return this.cells.find(c => Object.values(c.adjacency).some(v => v === this.inRow))
  // }

  // get_available_moves(): Cell[] {
  //   return this.cells.filter(c => getOwner(c) === 0).map(c => getCellValue(c))
  // }
}
