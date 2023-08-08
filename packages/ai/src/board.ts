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

export function cell_from_uint(value: number): Cell {
  const h = (value) & 0b111;
  const v = (value >> 3) & 0b111;
  const l = (value >> 6) & 0b111;
  const r = (value >> 9) & 0b111;
  const o = (value >> 12) & 0b1111;
  const x = (value >> 16) & 0b1111_1111;
  const y = (value >> 24) & 0b1111_1111;
  return {
    x,y,
owner:o,
    adjacency:
      {
        [Adjacency.Horizontal]:h,
        [Adjacency.Vertical]:v,
        [Adjacency.LeftToRightDiagonal]:l,
        [Adjacency.RightToLeftDiagonal]:r

      }
  }
}

export function cell_to_uint(cell: Cell):number {
  // const bits_per_adjacency =
  // const h = (value >> 0) & 0b111;
  // const v = (value >> 3) & 0b111;
  // const l = (value >> 6) & 0b111;
  // const r = (value >> 9) & 0b111;
  // const o = (value >> 12) & 0b1111;
  // const x = (value >> 16) & 0b1111_1111_1111_1111;
  // const y = (value >> 16) & 0b1111_1111_1111_1111;
  return ((cell.adjacency[Adjacency.Horizontal] & 0b111) )
       | ((cell.adjacency[Adjacency.Vertical] & 0b111) << 3)
    | ((cell.adjacency[Adjacency.LeftToRightDiagonal] & 0b111) << 6)
    | ((cell.adjacency[Adjacency.RightToLeftDiagonal] & 0b111) << 9)
    | ((cell.owner & 0b1111) << 12)
    | ((cell.x & 0b1111_1111) << 16)
    | ((cell.y & 0b1111_1111) << 24)
}


// @ts-ignore
function deepEqual(x, y) {
  return (x && y && typeof x === 'object' && typeof y === 'object') ?
    (Object.keys(x).length === Object.keys(y).length) &&
    Object.keys(x).reduce(function(isEqual, key) {
      return isEqual && deepEqual(x[key], y[key]);
    }, true) : (x === y);
}
export class Board {
  size = 3
  inRow = 3
  cells_array: Uint32Array = undefined!
  available = 3 * 3

  constructor(opts?: BoardOptions, previous?: Board) {
    this.size = previous?.size ?? opts?.gridSize ?? this.size
    this.inRow = previous?.inRow ?? opts?.inRow ?? this.inRow
    this.available = previous?.available ?? this.size * this.size

    const test_cell: Cell={
      x: 5,
      y: 0,
      owner: 2,
      adjacency: {
        [Adjacency.Horizontal]: 0,
        [Adjacency.Vertical]: 1,
        [Adjacency.LeftToRightDiagonal]: 4,
        [Adjacency.RightToLeftDiagonal]: 5,
      },
    }
    console.assert( deepEqual( cell_from_uint( cell_to_uint(test_cell)), test_cell))

    if (previous) {
      // TODO
      debugger;
    } else {
      this.cells_array = new Uint32Array({length:this.size*this.size});
      for (let y = 0; y < this.size; y += 1) {
        for (let x = 0; x < this.size; x += 1) {
          this.cells_array[this.index(x,y)] = cell_to_uint({
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
  }

  is_within_board(x: number, y: number) {
    return x >= 0 && y >= 0 && x < this.size && y < this.size
  }

  index(x: number, y: number) {
    return x + y * this.size
  }

  get_cell_at(x: number, y: number) {
    return cell_from_uint( this.cells_array[this.index(x,y)])
  }
  set_cell(cell: Cell) {
    return this.cells_array[this.index(cell.x,cell.y)] = cell_to_uint(cell)
  }

  set_cell_owner(x: number, y: number, player: number) {
    const cell = this.get_cell_at(x,y)
    cell.owner = player
    this.set_cell(cell)
    if (player !== 0) {
      this.available -= 1
    } else {
      this.available += 1
    }
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
      if (iters > 5) {
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
    let bestInRow = 0
    for (let i = 0; i < 4; i += 1) {
      const dir = i as Adjacency
      const cells = this.get_adjacent_cells(x, y, player, dir)
      const adjacent_count = cells.length + 1
      for (const c of cells) {
        c.adjacency[dir] = adjacent_count
        this.set_cell(c)
      }
      const cell = this.get_cell_at(x,y)
      cell.adjacency[dir] = adjacent_count
      this.set_cell(cell)
      if (adjacent_count > bestInRow) {
        bestInRow = adjacent_count
      }
    }
    return bestInRow === this.inRow
  }

  is_full() {
    return this.available === 0
  }

  get_available_moves(): Cell[] {
    return [...this.cells_array].map(c => cell_from_uint(c)).filter(c => c.owner === 0)
  }
}
