/* eslint-disable */
import * as _m0 from 'protobufjs/minimal'
import { Cell, GameStatus, gameStatusFromJSON, gameStatusToJSON, LobbyGame, Player } from './game'

export enum ServerMsgType {
  /** lobby_state - lobby */
  lobby_state = 0,
  player_status = 1,
  player_msg = 2,
  player_join_lobby = 3,
  player_leave_lobby = 4,
  lobby_game_updated = 5,
  /** player_join - game */
  player_join = 6,
  player_left = 7,
  player_disconnected = 8,
  player_reconnected = 9,
  game_start = 10,
  game_end = 11,
  game_player_move = 12,
  UNRECOGNIZED = -1,
}

export function serverMsgTypeFromJSON(object: any): ServerMsgType {
  switch (object) {
    case 0:
    case 'lobby_state':
      return ServerMsgType.lobby_state
    case 1:
    case 'player_status':
      return ServerMsgType.player_status
    case 2:
    case 'player_msg':
      return ServerMsgType.player_msg
    case 3:
    case 'player_join_lobby':
      return ServerMsgType.player_join_lobby
    case 4:
    case 'player_leave_lobby':
      return ServerMsgType.player_leave_lobby
    case 5:
    case 'lobby_game_updated':
      return ServerMsgType.lobby_game_updated
    case 6:
    case 'player_join':
      return ServerMsgType.player_join
    case 7:
    case 'player_left':
      return ServerMsgType.player_left
    case 8:
    case 'player_disconnected':
      return ServerMsgType.player_disconnected
    case 9:
    case 'player_reconnected':
      return ServerMsgType.player_reconnected
    case 10:
    case 'game_start':
      return ServerMsgType.game_start
    case 11:
    case 'game_end':
      return ServerMsgType.game_end
    case 12:
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
    case ServerMsgType.player_status:
      return 'player_status'
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

export interface PlayerStatus {
  waitingGames: string[]
  endedGames: GameEnd[]
}

export interface GameStart {
  gameId: string
  /** uint64 start_time = 2; */
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

export interface GamePlayerDisconnected {
  gameId: string
  playerId: number
  symbol: string
  name: string
}

export interface GamePlayerReconnected {
  gameId: string
  playerId: number
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
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseLobbyPlayer()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 2:
          if (tag !== 16) {
            break
          }

          message.playerId = reader.uint32()
          continue
        case 3:
          if (tag !== 26) {
            break
          }

          message.name = reader.string()
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
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
    if (message.playerId !== 0) {
      obj.playerId = Math.round(message.playerId)
    }
    if (message.name !== '') {
      obj.name = message.name
    }
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
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseLobbyState()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break
          }

          message.games.push(LobbyGame.decode(reader, reader.uint32()))
          continue
        case 2:
          if (tag !== 18) {
            break
          }

          message.players.push(LobbyPlayer.decode(reader, reader.uint32()))
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
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
    if (message.games?.length) {
      obj.games = message.games.map(e => LobbyGame.toJSON(e))
    }
    if (message.players?.length) {
      obj.players = message.players.map(e => LobbyPlayer.toJSON(e))
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

function createBasePlayerStatus(): PlayerStatus {
  return { waitingGames: [], endedGames: [] }
}

export const PlayerStatus = {
  encode(message: PlayerStatus, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.waitingGames) {
      writer.uint32(26).string(v!)
    }
    for (const v of message.endedGames) {
      GameEnd.encode(v!, writer.uint32(34).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerStatus {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerStatus()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 3:
          if (tag !== 26) {
            break
          }

          message.waitingGames.push(reader.string())
          continue
        case 4:
          if (tag !== 34) {
            break
          }

          message.endedGames.push(GameEnd.decode(reader, reader.uint32()))
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
    }
    return message
  },

  fromJSON(object: any): PlayerStatus {
    return {
      waitingGames: Array.isArray(object?.waitingGames)
        ? object.waitingGames.map((e: any) => String(e))
        : [],
      endedGames: Array.isArray(object?.endedGames)
        ? object.endedGames.map((e: any) => GameEnd.fromJSON(e))
        : [],
    }
  },

  toJSON(message: PlayerStatus): unknown {
    const obj: any = {}
    if (message.waitingGames?.length) {
      obj.waitingGames = message.waitingGames
    }
    if (message.endedGames?.length) {
      obj.endedGames = message.endedGames.map(e => GameEnd.toJSON(e))
    }
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerStatus>, I>>(base?: I): PlayerStatus {
    return PlayerStatus.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerStatus>, I>>(object: I): PlayerStatus {
    const message = createBasePlayerStatus()
    message.waitingGames = object.waitingGames?.map(e => e) || []
    message.endedGames = object.endedGames?.map(e => GameEnd.fromPartial(e)) || []
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
      Player.encode(v!, writer.uint32(26).fork()).ldelim()
    }
    for (const v of message.cells) {
      Cell.encode(v!, writer.uint32(34).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameStart {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameStart()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break
          }

          message.gameId = reader.string()
          continue
        case 3:
          if (tag !== 26) {
            break
          }

          message.players.push(Player.decode(reader, reader.uint32()))
          continue
        case 4:
          if (tag !== 34) {
            break
          }

          message.cells.push(Cell.decode(reader, reader.uint32()))
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
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
    if (message.gameId !== '') {
      obj.gameId = message.gameId
    }
    if (message.players?.length) {
      obj.players = message.players.map(e => Player.toJSON(e))
    }
    if (message.cells?.length) {
      obj.cells = message.cells.map(e => Cell.toJSON(e))
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
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameEnd()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break
          }

          message.gameId = reader.string()
          continue
        case 2:
          if (tag !== 16) {
            break
          }

          message.result = reader.int32() as any
          continue
        case 3:
          if (tag !== 24) {
            break
          }

          message.winnerId = reader.uint32()
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
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
    if (message.gameId !== '') {
      obj.gameId = message.gameId
    }
    if (message.result !== 0) {
      obj.result = gameStatusToJSON(message.result)
    }
    if (message.winnerId !== undefined) {
      obj.winnerId = Math.round(message.winnerId)
    }
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
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGameMove()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break
          }

          message.player = reader.uint32()
          continue
        case 2:
          if (tag !== 16) {
            break
          }

          message.x = reader.uint32()
          continue
        case 3:
          if (tag !== 24) {
            break
          }

          message.y = reader.uint32()
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
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
    if (message.player !== 0) {
      obj.player = Math.round(message.player)
    }
    if (message.x !== 0) {
      obj.x = Math.round(message.x)
    }
    if (message.y !== 0) {
      obj.y = Math.round(message.y)
    }
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

function createBaseGamePlayerDisconnected(): GamePlayerDisconnected {
  return { gameId: '', playerId: 0, symbol: '', name: '' }
}

export const GamePlayerDisconnected = {
  encode(message: GamePlayerDisconnected, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    if (message.symbol !== '') {
      writer.uint32(34).string(message.symbol)
    }
    if (message.name !== '') {
      writer.uint32(42).string(message.name)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GamePlayerDisconnected {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGamePlayerDisconnected()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break
          }

          message.gameId = reader.string()
          continue
        case 2:
          if (tag !== 16) {
            break
          }

          message.playerId = reader.uint32()
          continue
        case 4:
          if (tag !== 34) {
            break
          }

          message.symbol = reader.string()
          continue
        case 5:
          if (tag !== 42) {
            break
          }

          message.name = reader.string()
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
    }
    return message
  },

  fromJSON(object: any): GamePlayerDisconnected {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      symbol: isSet(object.symbol) ? String(object.symbol) : '',
      name: isSet(object.name) ? String(object.name) : '',
    }
  },

  toJSON(message: GamePlayerDisconnected): unknown {
    const obj: any = {}
    if (message.gameId !== '') {
      obj.gameId = message.gameId
    }
    if (message.playerId !== 0) {
      obj.playerId = Math.round(message.playerId)
    }
    if (message.symbol !== '') {
      obj.symbol = message.symbol
    }
    if (message.name !== '') {
      obj.name = message.name
    }
    return obj
  },

  create<I extends Exact<DeepPartial<GamePlayerDisconnected>, I>>(
    base?: I
  ): GamePlayerDisconnected {
    return GamePlayerDisconnected.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GamePlayerDisconnected>, I>>(
    object: I
  ): GamePlayerDisconnected {
    const message = createBaseGamePlayerDisconnected()
    message.gameId = object.gameId ?? ''
    message.playerId = object.playerId ?? 0
    message.symbol = object.symbol ?? ''
    message.name = object.name ?? ''
    return message
  },
}

function createBaseGamePlayerReconnected(): GamePlayerReconnected {
  return { gameId: '', playerId: 0 }
}

export const GamePlayerReconnected = {
  encode(message: GamePlayerReconnected, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GamePlayerReconnected {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseGamePlayerReconnected()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break
          }

          message.gameId = reader.string()
          continue
        case 2:
          if (tag !== 16) {
            break
          }

          message.playerId = reader.uint32()
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
    }
    return message
  },

  fromJSON(object: any): GamePlayerReconnected {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
    }
  },

  toJSON(message: GamePlayerReconnected): unknown {
    const obj: any = {}
    if (message.gameId !== '') {
      obj.gameId = message.gameId
    }
    if (message.playerId !== 0) {
      obj.playerId = Math.round(message.playerId)
    }
    return obj
  },

  create<I extends Exact<DeepPartial<GamePlayerReconnected>, I>>(base?: I): GamePlayerReconnected {
    return GamePlayerReconnected.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GamePlayerReconnected>, I>>(
    object: I
  ): GamePlayerReconnected {
    const message = createBaseGamePlayerReconnected()
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
