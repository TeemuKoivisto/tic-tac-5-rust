export interface Square {
  x: number
  y: number
  id: string
  owner: number
  piece: Piece | 'empty'
}

type Piece = 'rook' | 'knight' | 'bishop' | 'queen' | 'king' | 'pawn'

const RANKS = 'abcdefgh'
const PIECES: {
  [key: number]: Piece
} = {
  0: 'rook',
  1: 'knight',
  2: 'bishop',
  3: 'queen',
  4: 'king',
  5: 'bishop',
  6: 'knight',
  7: 'rook',
}

interface BoardOptions {}

export class Board {
  size = 8
  cells: Square[] = []

  constructor(opts?: BoardOptions) {
    const cells: Square[] = []
    for (let y = 0; y < this.size; y += 1) {
      for (let x = 0; x < this.size; x += 1) {
        let piece: Piece | 'empty'
        if (y === 0 || y === 7) {
          piece = PIECES[x]
        } else if (y === 1 || y === 6) {
          piece = 'pawn'
        } else {
          piece = 'empty'
        }
        cells.push({
          x,
          y,
          owner: y === 0 || y === 1 ? 1 : y === 6 || y === 7 ? 2 : 0,
          piece,
          id: `${RANKS.charAt(x)}${8 - y}`,
        })
      }
    }
    this.cells = cells
  }

  is_within_board(x: number, y: number) {
    return x >= 0 && y >= 0 && x < this.size && y < this.size
  }

  is_valid_move(x: number, y: number) {
    return this.is_within_board(x, y)
  }

  get_cell_at(x: number, y: number) {
    return this.cells[x + y * this.size]
  }

  move_piece(x: number, y: number, tx: number, ty: number) {
    const from = this.get_cell_at(x, y)
    const to = this.get_cell_at(tx, ty)
    to.owner = from.owner
    to.piece = from.piece
    from.owner = 0
    from.piece = 'empty'
    return false
  }
}
