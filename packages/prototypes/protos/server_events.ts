/* eslint-disable */
import Long from 'long'
import _m0 from 'protobufjs/minimal'
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

export enum PlayerAppState {
  initializing = 0,
  disconnected = 1,
  lobby = 2,
  waiting_game_start = 3,
  in_game = 4,
  errored = 5,
  UNRECOGNIZED = -1,
}

export function playerAppStateFromJSON(object: any): PlayerAppState {
  switch (object) {
    case 0:
    case 'initializing':
      return PlayerAppState.initializing
    case 1:
    case 'disconnected':
      return PlayerAppState.disconnected
    case 2:
    case 'lobby':
      return PlayerAppState.lobby
    case 3:
    case 'waiting_game_start':
      return PlayerAppState.waiting_game_start
    case 4:
    case 'in_game':
      return PlayerAppState.in_game
    case 5:
    case 'errored':
      return PlayerAppState.errored
    case -1:
    case 'UNRECOGNIZED':
    default:
      return PlayerAppState.UNRECOGNIZED
  }
}

export function playerAppStateToJSON(object: PlayerAppState): string {
  switch (object) {
    case PlayerAppState.initializing:
      return 'initializing'
    case PlayerAppState.disconnected:
      return 'disconnected'
    case PlayerAppState.lobby:
      return 'lobby'
    case PlayerAppState.waiting_game_start:
      return 'waiting_game_start'
    case PlayerAppState.in_game:
      return 'in_game'
    case PlayerAppState.errored:
      return 'errored'
    case PlayerAppState.UNRECOGNIZED:
    default:
      return 'UNRECOGNIZED'
  }
}

export enum PlayerInGameState {
  not_started = 0,
  x_turn = 1,
  o_turn = 2,
  waiting_player = 3,
  paused = 4,
  ended = 5,
  UNRECOGNIZED = -1,
}

export function playerInGameStateFromJSON(object: any): PlayerInGameState {
  switch (object) {
    case 0:
    case 'not_started':
      return PlayerInGameState.not_started
    case 1:
    case 'x_turn':
      return PlayerInGameState.x_turn
    case 2:
    case 'o_turn':
      return PlayerInGameState.o_turn
    case 3:
    case 'waiting_player':
      return PlayerInGameState.waiting_player
    case 4:
    case 'paused':
      return PlayerInGameState.paused
    case 5:
    case 'ended':
      return PlayerInGameState.ended
    case -1:
    case 'UNRECOGNIZED':
    default:
      return PlayerInGameState.UNRECOGNIZED
  }
}

export function playerInGameStateToJSON(object: PlayerInGameState): string {
  switch (object) {
    case PlayerInGameState.not_started:
      return 'not_started'
    case PlayerInGameState.x_turn:
      return 'x_turn'
    case PlayerInGameState.o_turn:
      return 'o_turn'
    case PlayerInGameState.waiting_player:
      return 'waiting_player'
    case PlayerInGameState.paused:
      return 'paused'
    case PlayerInGameState.ended:
      return 'ended'
    case PlayerInGameState.UNRECOGNIZED:
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

export interface PlayerState {
  appState: PlayerAppState
  gameState: PlayerInGameState
  waitingGames: string[]
  endedGames: GameEnd[]
}

export interface BoardState {
  gameId: string
  playerInTurn: number
  startTime: number
  players: Player[]
  cells: Cell[]
  state: PlayerInGameState
}

export interface GameEnd {
  gameId: string
  result: GameStatus
  winnerId?: number | undefined
  state: PlayerInGameState
}

export interface GameMove {
  playerNumber: number
  nextInTurn: number
  x: number
  y: number
  /** string symbol = 4; */
  state: PlayerInGameState
}

export interface GamePlayerDisconnected {
  gameId: string
  playerId: number
  symbol: string
  name: string
  state: PlayerInGameState
}

export interface GamePlayerReconnected {
  gameId: string
  playerId: number
  state: PlayerInGameState
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

function createBasePlayerState(): PlayerState {
  return { appState: 0, gameState: 0, waitingGames: [], endedGames: [] }
}

export const PlayerState = {
  encode(message: PlayerState, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.appState !== 0) {
      writer.uint32(8).int32(message.appState)
    }
    if (message.gameState !== 0) {
      writer.uint32(16).int32(message.gameState)
    }
    for (const v of message.waitingGames) {
      writer.uint32(26).string(v!)
    }
    for (const v of message.endedGames) {
      GameEnd.encode(v!, writer.uint32(34).fork()).ldelim()
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerState {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBasePlayerState()
    while (reader.pos < end) {
      const tag = reader.uint32()
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break
          }

          message.appState = reader.int32() as any
          continue
        case 2:
          if (tag !== 16) {
            break
          }

          message.gameState = reader.int32() as any
          continue
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

  fromJSON(object: any): PlayerState {
    return {
      appState: isSet(object.appState) ? playerAppStateFromJSON(object.appState) : 0,
      gameState: isSet(object.gameState) ? playerInGameStateFromJSON(object.gameState) : 0,
      waitingGames: Array.isArray(object?.waitingGames)
        ? object.waitingGames.map((e: any) => String(e))
        : [],
      endedGames: Array.isArray(object?.endedGames)
        ? object.endedGames.map((e: any) => GameEnd.fromJSON(e))
        : [],
    }
  },

  toJSON(message: PlayerState): unknown {
    const obj: any = {}
    if (message.appState !== 0) {
      obj.appState = playerAppStateToJSON(message.appState)
    }
    if (message.gameState !== 0) {
      obj.gameState = playerInGameStateToJSON(message.gameState)
    }
    if (message.waitingGames?.length) {
      obj.waitingGames = message.waitingGames
    }
    if (message.endedGames?.length) {
      obj.endedGames = message.endedGames.map(e => GameEnd.toJSON(e))
    }
    return obj
  },

  create<I extends Exact<DeepPartial<PlayerState>, I>>(base?: I): PlayerState {
    return PlayerState.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<PlayerState>, I>>(object: I): PlayerState {
    const message = createBasePlayerState()
    message.appState = object.appState ?? 0
    message.gameState = object.gameState ?? 0
    message.waitingGames = object.waitingGames?.map(e => e) || []
    message.endedGames = object.endedGames?.map(e => GameEnd.fromPartial(e)) || []
    return message
  },
}

function createBaseBoardState(): BoardState {
  return { gameId: '', playerInTurn: 0, startTime: 0, players: [], cells: [], state: 0 }
}

export const BoardState = {
  encode(message: BoardState, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerInTurn !== 0) {
      writer.uint32(16).uint32(message.playerInTurn)
    }
    if (message.startTime !== 0) {
      writer.uint32(24).uint64(message.startTime)
    }
    for (const v of message.players) {
      Player.encode(v!, writer.uint32(34).fork()).ldelim()
    }
    for (const v of message.cells) {
      Cell.encode(v!, writer.uint32(42).fork()).ldelim()
    }
    if (message.state !== 0) {
      writer.uint32(48).int32(message.state)
    }
    return writer
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BoardState {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input)
    let end = length === undefined ? reader.len : reader.pos + length
    const message = createBaseBoardState()
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

          message.playerInTurn = reader.uint32()
          continue
        case 3:
          if (tag !== 24) {
            break
          }

          message.startTime = longToNumber(reader.uint64() as Long)
          continue
        case 4:
          if (tag !== 34) {
            break
          }

          message.players.push(Player.decode(reader, reader.uint32()))
          continue
        case 5:
          if (tag !== 42) {
            break
          }

          message.cells.push(Cell.decode(reader, reader.uint32()))
          continue
        case 6:
          if (tag !== 48) {
            break
          }

          message.state = reader.int32() as any
          continue
      }
      if ((tag & 7) === 4 || tag === 0) {
        break
      }
      reader.skipType(tag & 7)
    }
    return message
  },

  fromJSON(object: any): BoardState {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : '',
      playerInTurn: isSet(object.playerInTurn) ? Number(object.playerInTurn) : 0,
      startTime: isSet(object.startTime) ? Number(object.startTime) : 0,
      players: Array.isArray(object?.players)
        ? object.players.map((e: any) => Player.fromJSON(e))
        : [],
      cells: Array.isArray(object?.cells) ? object.cells.map((e: any) => Cell.fromJSON(e)) : [],
      state: isSet(object.state) ? playerInGameStateFromJSON(object.state) : 0,
    }
  },

  toJSON(message: BoardState): unknown {
    const obj: any = {}
    if (message.gameId !== '') {
      obj.gameId = message.gameId
    }
    if (message.playerInTurn !== 0) {
      obj.playerInTurn = Math.round(message.playerInTurn)
    }
    if (message.startTime !== 0) {
      obj.startTime = Math.round(message.startTime)
    }
    if (message.players?.length) {
      obj.players = message.players.map(e => Player.toJSON(e))
    }
    if (message.cells?.length) {
      obj.cells = message.cells.map(e => Cell.toJSON(e))
    }
    if (message.state !== 0) {
      obj.state = playerInGameStateToJSON(message.state)
    }
    return obj
  },

  create<I extends Exact<DeepPartial<BoardState>, I>>(base?: I): BoardState {
    return BoardState.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<BoardState>, I>>(object: I): BoardState {
    const message = createBaseBoardState()
    message.gameId = object.gameId ?? ''
    message.playerInTurn = object.playerInTurn ?? 0
    message.startTime = object.startTime ?? 0
    message.players = object.players?.map(e => Player.fromPartial(e)) || []
    message.cells = object.cells?.map(e => Cell.fromPartial(e)) || []
    message.state = object.state ?? 0
    return message
  },
}

function createBaseGameEnd(): GameEnd {
  return { gameId: '', result: 0, winnerId: undefined, state: 0 }
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
    if (message.state !== 0) {
      writer.uint32(32).int32(message.state)
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
        case 4:
          if (tag !== 32) {
            break
          }

          message.state = reader.int32() as any
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
      state: isSet(object.state) ? playerInGameStateFromJSON(object.state) : 0,
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
    if (message.state !== 0) {
      obj.state = playerInGameStateToJSON(message.state)
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
    message.state = object.state ?? 0
    return message
  },
}

function createBaseGameMove(): GameMove {
  return { playerNumber: 0, nextInTurn: 0, x: 0, y: 0, state: 0 }
}

export const GameMove = {
  encode(message: GameMove, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerNumber !== 0) {
      writer.uint32(8).uint32(message.playerNumber)
    }
    if (message.nextInTurn !== 0) {
      writer.uint32(16).uint32(message.nextInTurn)
    }
    if (message.x !== 0) {
      writer.uint32(24).uint32(message.x)
    }
    if (message.y !== 0) {
      writer.uint32(32).uint32(message.y)
    }
    if (message.state !== 0) {
      writer.uint32(40).int32(message.state)
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

          message.playerNumber = reader.uint32()
          continue
        case 2:
          if (tag !== 16) {
            break
          }

          message.nextInTurn = reader.uint32()
          continue
        case 3:
          if (tag !== 24) {
            break
          }

          message.x = reader.uint32()
          continue
        case 4:
          if (tag !== 32) {
            break
          }

          message.y = reader.uint32()
          continue
        case 5:
          if (tag !== 40) {
            break
          }

          message.state = reader.int32() as any
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
      playerNumber: isSet(object.playerNumber) ? Number(object.playerNumber) : 0,
      nextInTurn: isSet(object.nextInTurn) ? Number(object.nextInTurn) : 0,
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
      state: isSet(object.state) ? playerInGameStateFromJSON(object.state) : 0,
    }
  },

  toJSON(message: GameMove): unknown {
    const obj: any = {}
    if (message.playerNumber !== 0) {
      obj.playerNumber = Math.round(message.playerNumber)
    }
    if (message.nextInTurn !== 0) {
      obj.nextInTurn = Math.round(message.nextInTurn)
    }
    if (message.x !== 0) {
      obj.x = Math.round(message.x)
    }
    if (message.y !== 0) {
      obj.y = Math.round(message.y)
    }
    if (message.state !== 0) {
      obj.state = playerInGameStateToJSON(message.state)
    }
    return obj
  },

  create<I extends Exact<DeepPartial<GameMove>, I>>(base?: I): GameMove {
    return GameMove.fromPartial(base ?? {})
  },

  fromPartial<I extends Exact<DeepPartial<GameMove>, I>>(object: I): GameMove {
    const message = createBaseGameMove()
    message.playerNumber = object.playerNumber ?? 0
    message.nextInTurn = object.nextInTurn ?? 0
    message.x = object.x ?? 0
    message.y = object.y ?? 0
    message.state = object.state ?? 0
    return message
  },
}

function createBaseGamePlayerDisconnected(): GamePlayerDisconnected {
  return { gameId: '', playerId: 0, symbol: '', name: '', state: 0 }
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
    if (message.state !== 0) {
      writer.uint32(48).int32(message.state)
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
        case 6:
          if (tag !== 48) {
            break
          }

          message.state = reader.int32() as any
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
      state: isSet(object.state) ? playerInGameStateFromJSON(object.state) : 0,
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
    if (message.state !== 0) {
      obj.state = playerInGameStateToJSON(message.state)
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
    message.state = object.state ?? 0
    return message
  },
}

function createBaseGamePlayerReconnected(): GamePlayerReconnected {
  return { gameId: '', playerId: 0, state: 0 }
}

export const GamePlayerReconnected = {
  encode(message: GamePlayerReconnected, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== '') {
      writer.uint32(10).string(message.gameId)
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId)
    }
    if (message.state !== 0) {
      writer.uint32(24).int32(message.state)
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
        case 3:
          if (tag !== 24) {
            break
          }

          message.state = reader.int32() as any
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
      state: isSet(object.state) ? playerInGameStateFromJSON(object.state) : 0,
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
    if (message.state !== 0) {
      obj.state = playerInGameStateToJSON(message.state)
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
    message.state = object.state ?? 0
    return message
  },
}

declare const self: any | undefined
declare const window: any | undefined
declare const global: any | undefined
const tsProtoGlobalThis: any = (() => {
  if (typeof globalThis !== 'undefined') {
    return globalThis
  }
  if (typeof self !== 'undefined') {
    return self
  }
  if (typeof window !== 'undefined') {
    return window
  }
  if (typeof global !== 'undefined') {
    return global
  }
  throw 'Unable to locate global object'
})()

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

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new tsProtoGlobalThis.Error('Value is larger than Number.MAX_SAFE_INTEGER')
  }
  return long.toNumber()
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any
  _m0.configure()
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined
}
