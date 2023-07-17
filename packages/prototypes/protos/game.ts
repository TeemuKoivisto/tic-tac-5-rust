/* eslint-disable */
import * as _m0 from 'protobufjs/minimal'

export enum CellType {
  EMPTY = 0,
  PLAYER_CELL = 1,
  UNRECOGNIZED = -1,
}

export function cellTypeFromJSON(object: any): CellType {
  switch (object) {
    case 0:
    case 'EMPTY':
      return CellType.EMPTY
    case 1:
    case 'PLAYER_CELL':
      return CellType.PLAYER_CELL
    case -1:
    case 'UNRECOGNIZED':
    default:
      return CellType.UNRECOGNIZED
  }
}

export function cellTypeToJSON(object: CellType): string {
  switch (object) {
    case CellType.EMPTY:
      return 'EMPTY'
    case CellType.PLAYER_CELL:
      return 'PLAYER_CELL'
    case CellType.UNRECOGNIZED:
    default:
      return 'UNRECOGNIZED'
  }
}

export enum GameStatus {
  WAITING = 0,
  X_TURN = 1,
  O_TURN = 2,
  X_WON = 3,
  O_WON = 4,
  TIE = 5,
  UNRECOGNIZED = -1,
}

export function gameStatusFromJSON(object: any): GameStatus {
  switch (object) {
    case 0:
    case 'WAITING':
      return GameStatus.WAITING
    case 1:
    case 'X_TURN':
      return GameStatus.X_TURN
    case 2:
    case 'O_TURN':
      return GameStatus.O_TURN
    case 3:
    case 'X_WON':
      return GameStatus.X_WON
    case 4:
    case 'O_WON':
      return GameStatus.O_WON
    case 5:
    case 'TIE':
      return GameStatus.TIE
    case -1:
    case 'UNRECOGNIZED':
    default:
      return GameStatus.UNRECOGNIZED
  }
}

export function gameStatusToJSON(object: GameStatus): string {
  switch (object) {
    case GameStatus.WAITING:
      return 'WAITING'
    case GameStatus.X_TURN:
      return 'X_TURN'
    case GameStatus.O_TURN:
      return 'O_TURN'
    case GameStatus.X_WON:
      return 'X_WON'
    case GameStatus.O_WON:
      return 'O_WON'
    case GameStatus.TIE:
      return 'TIE'
    case GameStatus.UNRECOGNIZED:
    default:
      return 'UNRECOGNIZED'
  }
}

export interface Player {
  id: number
  socketId: number
  playerNumber: number
  symbol: string
  name: string
  dead: boolean
  ai: boolean
}

export interface Cell {
  x: number
  y: number
  cellType: CellType
  player: number
}

export interface LobbyGame {
  gameId: string
  players: number
  maxPlayers: number
}

function createBasePlayer(): Player {
  return { id: 0, socketId: 0, playerNumber: 0, symbol: '', name: '', dead: false, ai: false }
}

export const Player = {
  encode(message: Player, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).uint32(message.id)
    }
    if (message.socketId !== 0) {
      writer.uint32(16).uint32(message.socketId)
    }
    if (message.playerNumber !== 0) {
      writer.uint32(24).uint32(message.playerNumber)
    }
    if (message.symbol !== '') {
      writer.uint32(34).string(message.symbol)
    }
    if (message.name !== '') {
      writer.uint32(42).string(message.name)
    }
    if (message.dead === true) {
      writer.uint32(48).bool(message.dead)
    }
    if (message.ai === true) {
      writer.uint32(64).bool(message.ai)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Player {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayer()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.id = reader.uint32()
          break
        case 2:
          message.socketId = reader.uint32()
          break
        case 3:
          message.playerNumber = reader.uint32()
          break
        case 4:
          message.symbol = reader.string()
          break
        case 5:
          message.name = reader.string()
          break
        case 6:
          message.dead = reader.bool()
          break
        case 8:
          message.ai = reader.bool()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): Player {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      socketId: isSet(object.socketId) ? Number(object.socketId) : 0,
      playerNumber: isSet(object.playerNumber) ? Number(object.playerNumber) : 0,
      symbol: isSet(object.symbol) ? String(object.symbol) : '',
      name: isSet(object.name) ? String(object.name) : '',
      dead: isSet(object.dead) ? Boolean(object.dead) : false,
      ai: isSet(object.ai) ? Boolean(object.ai) : false,
    }
  },

  toJSON(message: Player): unknown {
    const obj: any = {}
    message.id !== undefined && (obj.id = Math.round(message.id))
    message.socketId !== undefined && (obj.socketId = Math.round(message.socketId))
    message.playerNumber !== undefined && (obj.playerNumber = Math.round(message.playerNumber))
    message.symbol !== undefined && (obj.symbol = message.symbol)
    message.name !== undefined && (obj.name = message.name)
    message.dead !== undefined && (obj.dead = message.dead)
    message.ai !== undefined && (obj.ai = message.ai)
    return obj
  },

  create<I extends Exact<DeepPartial<Player>, I>>(base?: I): Player {
    return Player.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<Player>, I>>(object: I): Player {
    const message = createBasePlayer()
    message.id = object.id ?? 0
    message.socketId = object.socketId ?? 0
    message.playerNumber = object.playerNumber ?? 0
    message.symbol = object.symbol ?? ''
    message.name = object.name ?? ''
    message.dead = object.dead ?? false
    message.ai = object.ai ?? false
    return message
  },
}

function createBaseCell(): Cell {
  return { x: 0, y: 0, cellType: 0, player: 0 }
}

export const Cell = {
  encode(message: Cell, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.x !== 0) {
      writer.uint32(8).uint32(message.x)
    }
    if (message.y !== 0) {
      writer.uint32(16).uint32(message.y)
    }
    if (message.cellType !== 0) {
      writer.uint32(24).int32(message.cellType)
    }
    if (message.player !== 0) {
      writer.uint32(32).uint32(message.player)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Cell {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseCell()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.x = reader.uint32()
          break
        case 2:
          message.y = reader.uint32()
          break
        case 3:
          message.cellType = reader.int32() as any
          break
        case 4:
          message.player = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): Cell {
    return {
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
      cellType: isSet(object.cellType) ? cellTypeFromJSON(object.cellType) : 0,
      player: isSet(object.player) ? Number(object.player) : 0,
    }
  },

  toJSON(message: Cell): unknown {
    const obj: any = {}
    message.x !== undefined && (obj.x = Math.round(message.x))
    message.y !== undefined && (obj.y = Math.round(message.y))
    message.cellType !== undefined && (obj.cellType = cellTypeToJSON(message.cellType))
    message.player !== undefined && (obj.player = Math.round(message.player))
    return obj
  },

  create<I extends Exact<DeepPartial<Cell>, I>>(base?: I): Cell {
    return Cell.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<Cell>, I>>(object: I): Cell {
    const message = createBaseCell()
    message.x = object.x ?? 0
    message.y = object.y ?? 0
    message.cellType = object.cellType ?? 0
    message.player = object.player ?? 0
    return message
  },
}

function createBaseLobbyGame(): LobbyGame {
  return { gameId: '', players: 0, maxPlayers: 0 }
}

export const LobbyGame = {
  encode(message: LobbyGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.players !== 0) {
      writer.uint32(16).uint32(message.players)
    }
    if (message.maxPlayers !== 0) {
      writer.uint32(24).uint32(message.maxPlayers)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LobbyGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseLobbyGame()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string()
          break
        case 2:
          message.players = reader.uint32()
          break
        case 3:
          message.maxPlayers = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): LobbyGame {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      players: isSet(object.players) ? Number(object.players) : 0,
      maxPlayers: isSet(object.maxPlayers) ? Number(object.maxPlayers) : 0,
    }
  },

  toJSON(message: LobbyGame): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    message.players !== undefined && (obj.players = Math.round(message.players))
    message.maxPlayers !== undefined && (obj.maxPlayers = Math.round(message.maxPlayers))
    return obj
  },

  create<I extends Exact<DeepPartial<LobbyGame>, I>>(base?: I): LobbyGame {
    return LobbyGame.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<LobbyGame>, I>>(object: I): LobbyGame {
    const message = createBaseLobbyGame()
    message.gameId = object.gameId ?? ''
    message.players = object.players ?? 0
    message.maxPlayers = object.maxPlayers ?? 0
    return message
  },
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined

type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>

type KeysOfUnion<T> = T extends T ? keyof T : never
type Exact<P, I extends P> = P extends Builtin
  ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never }

function isSet(value: any): boolean {
  return value !== null && value !== undefined
}
