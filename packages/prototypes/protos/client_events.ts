/* eslint-disable */
import * as _m0 from 'protobufjs/minimal'

export enum ClientMsgType {
  /** join_lobby - lobby */
  join_lobby = 0,
  lobby_msg = 1,
  create_lobby_game = 2,
  join_lobby_game = 3,
  leave_lobby_game = 4,
  /** player_select_cell - game */
  player_select_cell = 5,
  leave_game = 6,
  UNRECOGNIZED = -1,
}

export function clientMsgTypeFromJSON(object: any): ClientMsgType {
  switch (object) {
    case 0:
    case 'join_lobby':
      return ClientMsgType.join_lobby
    case 1:
    case 'lobby_msg':
      return ClientMsgType.lobby_msg
    case 2:
    case 'create_lobby_game':
      return ClientMsgType.create_lobby_game
    case 3:
    case 'join_lobby_game':
      return ClientMsgType.join_lobby_game
    case 4:
    case 'leave_lobby_game':
      return ClientMsgType.leave_lobby_game
    case 5:
    case 'player_select_cell':
      return ClientMsgType.player_select_cell
    case 6:
    case 'leave_game':
      return ClientMsgType.leave_game
    case -1:
    case 'UNRECOGNIZED':
    default:
      return ClientMsgType.UNRECOGNIZED
  }
}

export function clientMsgTypeToJSON(object: ClientMsgType): string {
  switch (object) {
    case ClientMsgType.join_lobby:
      return 'join_lobby'
    case ClientMsgType.lobby_msg:
      return 'lobby_msg'
    case ClientMsgType.create_lobby_game:
      return 'create_lobby_game'
    case ClientMsgType.join_lobby_game:
      return 'join_lobby_game'
    case ClientMsgType.leave_lobby_game:
      return 'leave_lobby_game'
    case ClientMsgType.player_select_cell:
      return 'player_select_cell'
    case ClientMsgType.leave_game:
      return 'leave_game'
    case ClientMsgType.UNRECOGNIZED:
    default:
      return 'UNRECOGNIZED'
  }
}

/** Values are zero if not specified. */
export interface GameOptions {
  size: number
  players: number
}

export interface PlayerJoinLobby {
  playerId: number
  name: string
}

export interface PlayerCreateGame {
  playerId: number
  name: string
  preferredSymbol: string
  options: GameOptions | undefined
}

export interface PlayerJoinGame {
  gameId?: string | undefined
  playerId: number
  name: string
  options: GameOptions | undefined
}

export interface PlayerSelectCell {
  gameId: string
  playerNumber: number
  x: number
  y: number
}

export interface PlayerLeaveGame {
  gameId: string
  playerId: number
}

function createBaseGameOptions(): GameOptions {
  return { size: 0, players: 0 }
}

export const GameOptions = {
  encode(message: GameOptions, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.size !== 0) {
      writer.uint32(8).uint32(message.size)
    }
    if (message.players !== 0) {
      writer.uint32(16).uint32(message.players)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameOptions {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameOptions()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.size = reader.uint32()
          break
        case 2:
          message.players = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): GameOptions {
    return {
      size: isSet(object.size) ? Number(object.size) : 0,
      players: isSet(object.players) ? Number(object.players) : 0,
    }
  },

  toJSON(message: GameOptions): unknown {
    const obj: any = {}
    message.size !== undefined && (obj.size = Math.round(message.size))
    message.players !== undefined && (obj.players = Math.round(message.players))
    return obj
  },

  create<I extends Exact<DeepPartial<GameOptions>, I>>(base?: I): GameOptions {
    return GameOptions.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GameOptions>, I>>(object: I): GameOptions {
    const message = createBaseGameOptions()
    message.size = object.size ?? 0
    message.players = object.players ?? 0
    return message
  },
}

function createBasePlayerJoinLobby(): PlayerJoinLobby {
  return { playerId: 0, name: '' }
}

export const PlayerJoinLobby = {
  encode(message: PlayerJoinLobby, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerId !== 0) {
      writer.uint32(8).uint32(message.playerId)
    }
    if (message.name !== '') {
      writer.uint32(18).string(message.name)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerJoinLobby {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerJoinLobby()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.playerId = reader.uint32()
          break
        case 2:
          message.name = reader.string()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): PlayerJoinLobby {
    return {
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : '',
    }
  },

  toJSON(message: PlayerJoinLobby): unknown {
    const obj: any = {}
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId))
    message.name !== undefined && (obj.name = message.name)
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerJoinLobby>, I>>(base?: I): PlayerJoinLobby {
    return PlayerJoinLobby.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerJoinLobby>, I>>(object: I): PlayerJoinLobby {
    const message = createBasePlayerJoinLobby()
    message.playerId = object.playerId ?? 0
    message.name = object.name ?? ''
    return message
  },
}

function createBasePlayerCreateGame(): PlayerCreateGame {
  return { playerId: 0, name: '', preferredSymbol: '', options: undefined }
}

export const PlayerCreateGame = {
  encode(message: PlayerCreateGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerId !== 0) {
      writer.uint32(8).uint32(message.playerId)
    }
    if (message.name !== '') {
      writer.uint32(18).string(message.name)
    }
    if (message.preferredSymbol !== '') {
      writer.uint32(26).string(message.preferredSymbol)
    }
    if (message.options !== undefined) {
      GameOptions.encode(message.options, writer.uint32(34).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerCreateGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerCreateGame()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.playerId = reader.uint32()
          break
        case 2:
          message.name = reader.string()
          break
        case 3:
          message.preferredSymbol = reader.string()
          break
        case 4:
          message.options = GameOptions.decode(reader, reader.uint32())
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): PlayerCreateGame {
    return {
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : '',
      preferredSymbol: isSet(object.preferredSymbol) ? String(object.preferredSymbol) : '',
      options: isSet(object.options) ? GameOptions.fromJSON(object.options) : undefined,
    }
  },

  toJSON(message: PlayerCreateGame): unknown {
    const obj: any = {}
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId))
    message.name !== undefined && (obj.name = message.name)
    message.preferredSymbol !== undefined && (obj.preferredSymbol = message.preferredSymbol)
    message.options !== undefined &&
      (obj.options = message.options ? GameOptions.toJSON(message.options) : undefined)
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerCreateGame>, I>>(base?: I): PlayerCreateGame {
    return PlayerCreateGame.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerCreateGame>, I>>(object: I): PlayerCreateGame {
    const message = createBasePlayerCreateGame()
    message.playerId = object.playerId ?? 0
    message.name = object.name ?? ''
    message.preferredSymbol = object.preferredSymbol ?? ''
    message.options =
      object.options !== undefined && object.options !== null
        ? GameOptions.fromPartial(object.options)
        : undefined
    return message
  },
}

function createBasePlayerJoinGame(): PlayerJoinGame {
  return { gameId: undefined, playerId: 0, name: '', options: undefined }
}

export const PlayerJoinGame = {
  encode(message: PlayerJoinGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== undefined) {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    if (message.name !== '') {
      writer.uint32(26).string(message.name)
    }
    if (message.options !== undefined) {
      GameOptions.encode(message.options, writer.uint32(34).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerJoinGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerJoinGame()
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
          message.name = reader.string()
          break
        case 4:
          message.options = GameOptions.decode(reader, reader.uint32())
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): PlayerJoinGame {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : undefined,
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : '',
      options: isSet(object.options) ? GameOptions.fromJSON(object.options) : undefined,
    }
  },

  toJSON(message: PlayerJoinGame): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId))
    message.name !== undefined && (obj.name = message.name)
    message.options !== undefined &&
      (obj.options = message.options ? GameOptions.toJSON(message.options) : undefined)
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerJoinGame>, I>>(base?: I): PlayerJoinGame {
    return PlayerJoinGame.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerJoinGame>, I>>(object: I): PlayerJoinGame {
    const message = createBasePlayerJoinGame()
    message.gameId = object.gameId ?? undefined
    message.playerId = object.playerId ?? 0
    message.name = object.name ?? ''
    message.options =
      object.options !== undefined && object.options !== null
        ? GameOptions.fromPartial(object.options)
        : undefined
    return message
  },
}

function createBasePlayerSelectCell(): PlayerSelectCell {
  return { gameId: '', playerNumber: 0, x: 0, y: 0 }
}

export const PlayerSelectCell = {
  encode(message: PlayerSelectCell, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerNumber !== 0) {
      writer.uint32(16).uint32(message.playerNumber)
    }
    if (message.x !== 0) {
      writer.uint32(24).uint32(message.x)
    }
    if (message.y !== 0) {
      writer.uint32(32).uint32(message.y)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerSelectCell {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerSelectCell()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string()
          break
        case 2:
          message.playerNumber = reader.uint32()
          break
        case 3:
          message.x = reader.uint32()
          break
        case 4:
          message.y = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): PlayerSelectCell {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      playerNumber: isSet(object.playerNumber) ? Number(object.playerNumber) : 0,
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
    }
  },

  toJSON(message: PlayerSelectCell): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    message.playerNumber !== undefined && (obj.playerNumber = Math.round(message.playerNumber))
    message.x !== undefined && (obj.x = Math.round(message.x))
    message.y !== undefined && (obj.y = Math.round(message.y))
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerSelectCell>, I>>(base?: I): PlayerSelectCell {
    return PlayerSelectCell.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerSelectCell>, I>>(object: I): PlayerSelectCell {
    const message = createBasePlayerSelectCell()
    message.gameId = object.gameId ?? ''
    message.playerNumber = object.playerNumber ?? 0
    message.x = object.x ?? 0
    message.y = object.y ?? 0
    return message
  },
}

function createBasePlayerLeaveGame(): PlayerLeaveGame {
  return { gameId: '', playerId: 0 }
}

export const PlayerLeaveGame = {
  encode(message: PlayerLeaveGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerLeaveGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerLeaveGame()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string()
          break
        case 2:
          message.playerId = reader.uint32()
          break
        default:
          reader.skipType(tag & 7)
          break
      }
    }
    return message
  },

  fromJSON(object: any): PlayerLeaveGame {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
    }
  },

  toJSON(message: PlayerLeaveGame): unknown {
    const obj: any = {}
    message.gameId !== undefined && (obj.gameId = message.gameId)
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId))
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerLeaveGame>, I>>(base?: I): PlayerLeaveGame {
    return PlayerLeaveGame.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerLeaveGame>, I>>(object: I): PlayerLeaveGame {
    const message = createBasePlayerLeaveGame()
    message.gameId = object.gameId ?? ''
    message.playerId = object.playerId ?? 0
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
