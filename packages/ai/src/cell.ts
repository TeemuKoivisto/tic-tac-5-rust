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

export function createCell(x: number, y: number) {
  return x | (y << 6)
}

export function setOwner(c: number, owner: number) {
  return ((c << 4) >> 4) | (owner << 28)
}

export function getOwner(c: number) {
  return c >> 28
}

export function setDirAdjacency(c: number, dir: Adjacency, val: number) {
  let shift
  switch (dir) {
    case Adjacency.Horizontal:
      shift = 12
      break
    case Adjacency.Vertical:
      shift = 16
      break
    case Adjacency.LeftToRightDiagonal:
      shift = 20
      break
    case Adjacency.RightToLeftDiagonal:
      shift = 24
      break
    default:
      throw Error(`Unknown direction: ${dir}`)
  }
  return (((0xffffffff >>> 1) ^ (0xf << shift)) & c) | ((0xf & val) << shift)
}

export function setAdjacency(c: number, hor: number, ver: number, ldiag: number, rdiag: number) {
  return (
    ((c << 16) >> 16) |
    (((0xf & hor) << 12) | ((0xf & ver) << 16) | ((0xf & ldiag) << 20) | ((0xf & rdiag) << 24))
  )
}

export function getCellCoords(c: number): [number, number] {
  return [0b111111 & c, 0b111111 & (c >> 6)]
}

export function getCellCoordsOwner(c: number): [number, number, number] {
  return [0b111111 & c, 0b111111 & (c >> 6), 0xf & (c >> 28)]
}

export function getCellValue(c: number): Cell {
  return {
    x: 0b111111 & c,
    y: 0b111111 & (c >> 6),
    owner: 0xf & (c >> 28),
    adjacency: {
      [Adjacency.Horizontal]: 0xf & (c >> 12),
      [Adjacency.Vertical]: 0xf & (c >> 16),
      [Adjacency.LeftToRightDiagonal]: 0xf & (c >> 20),
      [Adjacency.RightToLeftDiagonal]: 0xf & (c >> 24),
    },
  }
}
