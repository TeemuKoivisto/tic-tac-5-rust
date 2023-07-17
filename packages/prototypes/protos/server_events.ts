/* eslint-disable */
import * as _m0 from 'protobufjs/minimal'
import { Cell, GameStatus, gameStatusFromJSON, gameStatusToJSON, LobbyGame, Player } from './game'

export enum ServerMsgType {
  /** lobby_state - lobby */
  lobby_state = 0,
  player_msg = 1,
  player_join_lobby = 2,
  player_leave_lobby = 3,
  lobby_game_updated = 4,
  /** player_join - game */
  player_join = 5,
  player_left = 6,
  player_disconnected = 7,
  player_reconnected = 8,
  game_start = 9,
  game_end = 10,
  game_player_move = 11,
  UNRECOGNIZED = -1,
}

export function serverMsgTypeFromJSON(object: any): ServerMsgType {
  switch (object) {
    case 0:
    case 'lobby_state':
      return ServerMsgType.lobby_state
    case 1:
    case 'player_msg':
      return ServerMsgType.player_msg
    case 2:
    case 'player_join_lobby':
      return ServerMsgType.player_join_lobby
    case 3:
    case 'player_leave_lobby':
      return ServerMsgType.player_leave_lobby
    case 4:
    case 'lobby_game_updated':
      return ServerMsgType.lobby_game_updated
    case 5:
    case 'player_join':
      return ServerMsgType.player_join
    case 6:
    case 'player_left':
      return ServerMsgType.player_left
    case 7:
    case 'player_disconnected':
      return ServerMsgType.player_disconnected
    case 8:
    case 'player_reconnected':
      return ServerMsgType.player_reconnected
    case 9:
    case 'game_start':
      return ServerMsgType.game_start
    case 10:
    case 'game_end':
      return ServerMsgType.game_end
    case 11:
    case 'game_player_move':
      return ServerMsgType.game_player_move
    case -1:
    case 'UNRECOGNIZED':
    default:
      return ServerMsgType.UNRECOGNIZED
  }
}

export function serverMsgTypeToJSON(object: ServerMsgType): string {
  switch (object) {
    case ServerMsgType.lobby_state:
      return 'lobby_state'
    case ServerMsgType.player_msg:
      return 'player_msg'
    case ServerMsgType.player_join_lobby:
      return 'player_join_lobby'
    case ServerMsgType.player_leave_lobby:
      return 'player_leave_lobby'
    case ServerMsgType.lobby_game_updated:
      return 'lobby_game_updated'
    case ServerMsgType.player_join:
      return 'player_join'
    case ServerMsgType.player_left:
      return 'player_left'
    case ServerMsgType.player_disconnected:
      return 'player_disconnected'
    case ServerMsgType.player_reconnected:
      return 'player_reconnected'
    case ServerMsgType.game_start:
      return 'game_start'
    case ServerMsgType.game_end:
      return 'game_end'
    case ServerMsgType.game_player_move:
      return 'game_player_move'
    case ServerMsgType.UNRECOGNIZED:
    default:
      return 'UNRECOGNIZED'
  }
}

export interface LobbyPlayer {
  playerId: number
  name: string
}

export interface LobbyState {
  games: LobbyGame[]
  players: LobbyPlayer[]
}

export interface GameStart {
  gameId: string
  players: Player[]
  cells: Cell[]
}

export interface GameEnd {
  gameId: string
  result: GameStatus
  winnerId?: number | undefined
}

export interface GameMove {
  player: number
  x: number
  /** string symbol = 4; */
  y: number
}

export interface GamePlayerConnection {
  gameId: string
  playerId: number
  connected: boolean
}

function createBaseLobbyPlayer(): LobbyPlayer {
  return { playerId: 0, name: '' }
}

export const LobbyPlayer = {
  encode(message: LobbyPlayer, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    if (message.name !== '') {
      writer.uint32(26).string(message.name)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LobbyPlayer {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseLobbyPlayer()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 2:
          message.playerId = reader.uint32()
          break
        case 3:
          message.name = reader.string()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): LobbyPlayer {
    return {
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : '',
    }
  },

  toJSON(message: LobbyPlayer): unknown {
    const obj: any = {}
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId))
    message.name !== undefined && (obj.name = message.name)
    return obj
  },

  create<I extends Exact<DeepPartial<LobbyPlayer>, I>>(base?: I): LobbyPlayer {
    return LobbyPlayer.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<LobbyPlayer>, I>>(object: I): LobbyPlayer {
    const message = createBaseLobbyPlayer()
    message.playerId = object.playerId ?? 0
    message.name = object.name ?? ''
    return message
  },
}

function createBaseLobbyState(): LobbyState {
  return { games: [], players: [] }
}

export const LobbyState = {
  encode(message: LobbyState, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.games) {
      LobbyGame.encode(v!, writer.uint32(10).fork()).ldelim()
    }
    for (const v of message.players) {
      LobbyPlayer.encode(v!, writer.uint32(18).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LobbyState {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseLobbyState()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.games.push(LobbyGame.decode(reader, reader.uint32()))
          break
        case 2:
          message.players.push(LobbyPlayer.decode(reader, reader.uint32()))
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): LobbyState {
    return {
      games: Array.isArray(object?.games)
        ? object.games.map((e: any) => LobbyGame.fromJSON(e))
        : [],
      players: Array.isArray(object?.players)
        ? object.players.map((e: any) => LobbyPlayer.fromJSON(e))
        : [],
    }
  },

  toJSON(message: LobbyState): unknown {
    const obj: any = {}
    if (message.games) {
      obj.games = message.games.map(e => (e ? LobbyGame.toJSON(e) : undefined))
    } else {
      obj.games = []
    }
    if (message.players) {
      obj.players = message.players.map(e => (e ? LobbyPlayer.toJSON(e) : undefined))
    } else {
      obj.players = []
    }
    return obj
  },

  create<I extends Exact<DeepPartial<LobbyState>, I>>(base?: I): LobbyState {
    return LobbyState.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<LobbyState>, I>>(object: I): LobbyState {
    const message = createBaseLobbyState()
    message.games = object.games?.map(e => LobbyGame.fromPartial(e)) || []
    message.players = object.players?.map(e => LobbyPlayer.fromPartial(e)) || []
    return message
  },
}

function createBaseGameStart(): GameStart {
  return { gameId: '', players: [], cells: [] }
}

export const GameStart = {
  encode(message: GameStart, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    for (const v of message.players) {
      Player.encode(v!, writer.uint32(18).fork()).ldelim()
    }
    for (const v of message.cells) {
      Cell.encode(v!, writer.uint32(26).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameStart {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameStart()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string()
          break
        case 2:
          message.players.push(Player.decode(reader, reader.uint32()))
          break
        case 3:
          message.cells.push(Cell.decode(reader, reader.uint32()))
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): GameStart {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      players: Array.isArray(object?.players)
        ? object.players.map((e: any) => Player.fromJSON(e))
        : [],
      cells: Array.isArray(object?.cells) ? object.cells.map((e: any) => Cell.fromJSON(e)) : [],
    }
  },

  toJSON(message: GameStart): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    if (message.players) {
      obj.players = message.players.map(e => (e ? Player.toJSON(e) : undefined))
    } else {
      obj.players = []
    }
    if (message.cells) {
      obj.cells = message.cells.map(e => (e ? Cell.toJSON(e) : undefined))
    } else {
      obj.cells = []
    }
    return obj
  },

  create<I extends Exact<DeepPartial<GameStart>, I>>(base?: I): GameStart {
    return GameStart.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GameStart>, I>>(object: I): GameStart {
    const message = createBaseGameStart()
    message.gameId = object.gameId ?? ''
    message.players = object.players?.map(e => Player.fromPartial(e)) || []
    message.cells = object.cells?.map(e => Cell.fromPartial(e)) || []
    return message
  },
}

function createBaseGameEnd(): GameEnd {
  return { gameId: '', result: 0, winnerId: undefined }
}

export const GameEnd = {
  encode(message: GameEnd, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.result !== 0) {
      writer.uint32(16).int32(message.result)
    }
    if (message.winnerId !== undefined) {
      writer.uint32(24).uint32(message.winnerId)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameEnd {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameEnd()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string()
          break
        case 2:
          message.result = reader.int32() as any
          break
        case 3:
          message.winnerId = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): GameEnd {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      result: isSet(object.result) ? gameStatusFromJSON(object.result) : 0,
      winnerId: isSet(object.winnerId) ? Number(object.winnerId) : undefined,
    }
  },

  toJSON(message: GameEnd): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    message.result !== undefined && (obj.result = gameStatusToJSON(message.result))
    message.winnerId !== undefined && (obj.winnerId = Math.round(message.winnerId))
    return obj
  },

  create<I extends Exact<DeepPartial<GameEnd>, I>>(base?: I): GameEnd {
    return GameEnd.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GameEnd>, I>>(object: I): GameEnd {
    const message = createBaseGameEnd()
    message.gameId = object.gameId ?? ''
    message.result = object.result ?? 0
    message.winnerId = object.winnerId ?? undefined
    return message
  },
}

function createBaseGameMove(): GameMove {
  return { player: 0, x: 0, y: 0 }
}

export const GameMove = {
  encode(message: GameMove, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.player !== 0) {
      writer.uint32(8).uint32(message.player)
    }
    if (message.x !== 0) {
      writer.uint32(16).uint32(message.x)
    }
    if (message.y !== 0) {
      writer.uint32(24).uint32(message.y)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameMove {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameMove()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.player = reader.uint32()
          break
        case 2:
          message.x = reader.uint32()
          break
        case 3:
          message.y = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): GameMove {
    return {
      player: isSet(object.player) ? Number(object.player) : 0,
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
    }
  },

  toJSON(message: GameMove): unknown {
    const obj: any = {}
    message.player !== undefined && (obj.player = Math.round(message.player))
    message.x !== undefined && (obj.x = Math.round(message.x))
    message.y !== undefined && (obj.y = Math.round(message.y))
    return obj
  },

  create<I extends Exact<DeepPartial<GameMove>, I>>(base?: I): GameMove {
    return GameMove.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GameMove>, I>>(object: I): GameMove {
    const message = createBaseGameMove()
    message.player = object.player ?? 0
    message.x = object.x ?? 0
    message.y = object.y ?? 0
    return message
  },
}

function createBaseGamePlayerConnection(): GamePlayerConnection {
  return { gameId: '', playerId: 0, connected: false }
}

export const GamePlayerConnection = {
  encode(message: GamePlayerConnection, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    if (message.connected === true) {
      writer.uint32(24).bool(message.connected)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GamePlayerConnection {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGamePlayerConnection()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string()
          break
        case 2:
          message.playerId = reader.uint32()
          break
        case 3:
          message.connected = reader.bool()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): GamePlayerConnection {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      connected: isSet(object.connected) ? Boolean(object.connected) : false,
    }
  },

  toJSON(message: GamePlayerConnection): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId))
    message.connected !== undefined && (obj.connected = message.connected)
    return obj
  },

  create<I extends Exact<DeepPartial<GamePlayerConnection>, I>>(base?: I): GamePlayerConnection {
    return GamePlayerConnection.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GamePlayerConnection>, I>>(
    object: I
  ): GamePlayerConnection {
    const message = createBaseGamePlayerConnection()
    message.gameId = object.gameId ?? ''
    message.playerId = object.playerId ?? 0
    message.connected = object.connected ?? false
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
