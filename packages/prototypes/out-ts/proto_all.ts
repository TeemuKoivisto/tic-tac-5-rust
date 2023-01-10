/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export enum ClientMsgType {
  /** join_lobby - lobby */
  join_lobby = 0,
  lobby_msg = 1,
  create_lobby_game = 2,
  join_lobby_game = 3,
  leave_lobby_game = 4,
  /** player_move - game */
  player_move = 5,
  leave_game = 6,
  UNRECOGNIZED = -1,
}

export function clientMsgTypeFromJSON(object: any): ClientMsgType {
  switch (object) {
    case 0:
    case "join_lobby":
      return ClientMsgType.join_lobby;
    case 1:
    case "lobby_msg":
      return ClientMsgType.lobby_msg;
    case 2:
    case "create_lobby_game":
      return ClientMsgType.create_lobby_game;
    case 3:
    case "join_lobby_game":
      return ClientMsgType.join_lobby_game;
    case 4:
    case "leave_lobby_game":
      return ClientMsgType.leave_lobby_game;
    case 5:
    case "player_move":
      return ClientMsgType.player_move;
    case 6:
    case "leave_game":
      return ClientMsgType.leave_game;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ClientMsgType.UNRECOGNIZED;
  }
}

export function clientMsgTypeToJSON(object: ClientMsgType): string {
  switch (object) {
    case ClientMsgType.join_lobby:
      return "join_lobby";
    case ClientMsgType.lobby_msg:
      return "lobby_msg";
    case ClientMsgType.create_lobby_game:
      return "create_lobby_game";
    case ClientMsgType.join_lobby_game:
      return "join_lobby_game";
    case ClientMsgType.leave_lobby_game:
      return "leave_lobby_game";
    case ClientMsgType.player_move:
      return "player_move";
    case ClientMsgType.leave_game:
      return "leave_game";
    case ClientMsgType.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum ServerMsgType {
  /** lobby_state - lobby */
  lobby_state = 0,
  player_msg = 1,
  player_join_lobby = 2,
  player_leave_lobby = 3,
  lobby_game_updated = 4,
  /** player_join - game */
  player_join = 5,
  game_start = 6,
  game_end = 7,
  tick = 8,
  player_left = 9,
  UNRECOGNIZED = -1,
}

export function serverMsgTypeFromJSON(object: any): ServerMsgType {
  switch (object) {
    case 0:
    case "lobby_state":
      return ServerMsgType.lobby_state;
    case 1:
    case "player_msg":
      return ServerMsgType.player_msg;
    case 2:
    case "player_join_lobby":
      return ServerMsgType.player_join_lobby;
    case 3:
    case "player_leave_lobby":
      return ServerMsgType.player_leave_lobby;
    case 4:
    case "lobby_game_updated":
      return ServerMsgType.lobby_game_updated;
    case 5:
    case "player_join":
      return ServerMsgType.player_join;
    case 6:
    case "game_start":
      return ServerMsgType.game_start;
    case 7:
    case "game_end":
      return ServerMsgType.game_end;
    case 8:
    case "tick":
      return ServerMsgType.tick;
    case 9:
    case "player_left":
      return ServerMsgType.player_left;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ServerMsgType.UNRECOGNIZED;
  }
}

export function serverMsgTypeToJSON(object: ServerMsgType): string {
  switch (object) {
    case ServerMsgType.lobby_state:
      return "lobby_state";
    case ServerMsgType.player_msg:
      return "player_msg";
    case ServerMsgType.player_join_lobby:
      return "player_join_lobby";
    case ServerMsgType.player_leave_lobby:
      return "player_leave_lobby";
    case ServerMsgType.lobby_game_updated:
      return "lobby_game_updated";
    case ServerMsgType.player_join:
      return "player_join";
    case ServerMsgType.game_start:
      return "game_start";
    case ServerMsgType.game_end:
      return "game_end";
    case ServerMsgType.tick:
      return "tick";
    case ServerMsgType.player_left:
      return "player_left";
    case ServerMsgType.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
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
    case "WAITING":
      return GameStatus.WAITING;
    case 1:
    case "X_TURN":
      return GameStatus.X_TURN;
    case 2:
    case "O_TURN":
      return GameStatus.O_TURN;
    case 3:
    case "X_WON":
      return GameStatus.X_WON;
    case 4:
    case "O_WON":
      return GameStatus.O_WON;
    case 5:
    case "TIE":
      return GameStatus.TIE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return GameStatus.UNRECOGNIZED;
  }
}

export function gameStatusToJSON(object: GameStatus): string {
  switch (object) {
    case GameStatus.WAITING:
      return "WAITING";
    case GameStatus.X_TURN:
      return "X_TURN";
    case GameStatus.O_TURN:
      return "O_TURN";
    case GameStatus.X_WON:
      return "X_WON";
    case GameStatus.O_WON:
      return "O_WON";
    case GameStatus.TIE:
      return "TIE";
    case GameStatus.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface Player {
  id: number;
  socketId: number;
  playerNumber: number;
  symbol: string;
  name: string;
  dead: boolean;
}

export interface Cursor {
  player: number;
  x: number;
  y: number;
}

export interface BallOwner {
  player: number;
  percentage: number;
}

export interface Ball {
  player: number;
  color: number;
  x: number;
  y: number;
  radius: number;
  owners: BallOwner[];
}

export interface Rect {
  /** uint32 color = 1; */
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
}

export interface LobbyGame {
  gameId: string;
  players: number;
  maxPlayers: number;
}

export interface LobbyPlayer {
  playerId: number;
  name: string;
}

export interface LobbyState {
  games: LobbyGame[];
  players: LobbyPlayer[];
}

export interface GameStart {
  gameId: string;
  players: Player[];
  balls: Ball[];
  rects: Rect[];
}

export interface GameEnd {
  gameId: string;
  result: GameStatus;
  winner?: Player | undefined;
}

export interface Move {
  x: number;
  y: number;
  targetX: number;
  targetY: number;
}

export interface Tick {
  gameId: string;
  cursors: Cursor[];
  balls: Ball[];
}

/** Values are zero if not specified. */
export interface GameOptions {
  size: number;
  players: number;
}

export interface PlayerJoinLobby {
  playerId: number;
  name: string;
}

export interface PlayerCreateGame {
  playerId: number;
  name: string;
  color: string;
  options: GameOptions | undefined;
}

export interface PlayerJoinGame {
  gameId?: string | undefined;
  playerId: number;
  name: string;
  color: string;
  options: GameOptions | undefined;
}

export interface PlayerMove {
  gameId: string;
  playerNumber: number;
  x: number;
  y: number;
}

export interface PlayerLeave {
  gameId: string;
  playerId: number;
}

function createBasePlayer(): Player {
  return { id: 0, socketId: 0, playerNumber: 0, symbol: "", name: "", dead: false };
}

export const Player = {
  encode(message: Player, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== 0) {
      writer.uint32(8).uint32(message.id);
    }
    if (message.socketId !== 0) {
      writer.uint32(16).uint32(message.socketId);
    }
    if (message.playerNumber !== 0) {
      writer.uint32(24).uint32(message.playerNumber);
    }
    if (message.symbol !== "") {
      writer.uint32(34).string(message.symbol);
    }
    if (message.name !== "") {
      writer.uint32(42).string(message.name);
    }
    if (message.dead === true) {
      writer.uint32(48).bool(message.dead);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Player {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayer();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.uint32();
          break;
        case 2:
          message.socketId = reader.uint32();
          break;
        case 3:
          message.playerNumber = reader.uint32();
          break;
        case 4:
          message.symbol = reader.string();
          break;
        case 5:
          message.name = reader.string();
          break;
        case 6:
          message.dead = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Player {
    return {
      id: isSet(object.id) ? Number(object.id) : 0,
      socketId: isSet(object.socketId) ? Number(object.socketId) : 0,
      playerNumber: isSet(object.playerNumber) ? Number(object.playerNumber) : 0,
      symbol: isSet(object.symbol) ? String(object.symbol) : "",
      name: isSet(object.name) ? String(object.name) : "",
      dead: isSet(object.dead) ? Boolean(object.dead) : false,
    };
  },

  toJSON(message: Player): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = Math.round(message.id));
    message.socketId !== undefined && (obj.socketId = Math.round(message.socketId));
    message.playerNumber !== undefined && (obj.playerNumber = Math.round(message.playerNumber));
    message.symbol !== undefined && (obj.symbol = message.symbol);
    message.name !== undefined && (obj.name = message.name);
    message.dead !== undefined && (obj.dead = message.dead);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Player>, I>>(object: I): Player {
    const message = createBasePlayer();
    message.id = object.id ?? 0;
    message.socketId = object.socketId ?? 0;
    message.playerNumber = object.playerNumber ?? 0;
    message.symbol = object.symbol ?? "";
    message.name = object.name ?? "";
    message.dead = object.dead ?? false;
    return message;
  },
};

function createBaseCursor(): Cursor {
  return { player: 0, x: 0, y: 0 };
}

export const Cursor = {
  encode(message: Cursor, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.player !== 0) {
      writer.uint32(8).uint32(message.player);
    }
    if (message.x !== 0) {
      writer.uint32(16).uint32(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(24).uint32(message.y);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Cursor {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCursor();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.player = reader.uint32();
          break;
        case 2:
          message.x = reader.uint32();
          break;
        case 3:
          message.y = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Cursor {
    return {
      player: isSet(object.player) ? Number(object.player) : 0,
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
    };
  },

  toJSON(message: Cursor): unknown {
    const obj: any = {};
    message.player !== undefined && (obj.player = Math.round(message.player));
    message.x !== undefined && (obj.x = Math.round(message.x));
    message.y !== undefined && (obj.y = Math.round(message.y));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Cursor>, I>>(object: I): Cursor {
    const message = createBaseCursor();
    message.player = object.player ?? 0;
    message.x = object.x ?? 0;
    message.y = object.y ?? 0;
    return message;
  },
};

function createBaseBallOwner(): BallOwner {
  return { player: 0, percentage: 0 };
}

export const BallOwner = {
  encode(message: BallOwner, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.player !== 0) {
      writer.uint32(8).uint32(message.player);
    }
    if (message.percentage !== 0) {
      writer.uint32(21).float(message.percentage);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BallOwner {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBallOwner();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.player = reader.uint32();
          break;
        case 2:
          message.percentage = reader.float();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BallOwner {
    return {
      player: isSet(object.player) ? Number(object.player) : 0,
      percentage: isSet(object.percentage) ? Number(object.percentage) : 0,
    };
  },

  toJSON(message: BallOwner): unknown {
    const obj: any = {};
    message.player !== undefined && (obj.player = Math.round(message.player));
    message.percentage !== undefined && (obj.percentage = message.percentage);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<BallOwner>, I>>(object: I): BallOwner {
    const message = createBaseBallOwner();
    message.player = object.player ?? 0;
    message.percentage = object.percentage ?? 0;
    return message;
  },
};

function createBaseBall(): Ball {
  return { player: 0, color: 0, x: 0, y: 0, radius: 0, owners: [] };
}

export const Ball = {
  encode(message: Ball, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.player !== 0) {
      writer.uint32(8).uint32(message.player);
    }
    if (message.color !== 0) {
      writer.uint32(16).uint32(message.color);
    }
    if (message.x !== 0) {
      writer.uint32(29).float(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(37).float(message.y);
    }
    if (message.radius !== 0) {
      writer.uint32(45).float(message.radius);
    }
    for (const v of message.owners) {
      BallOwner.encode(v!, writer.uint32(50).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Ball {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBall();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.player = reader.uint32();
          break;
        case 2:
          message.color = reader.uint32();
          break;
        case 3:
          message.x = reader.float();
          break;
        case 4:
          message.y = reader.float();
          break;
        case 5:
          message.radius = reader.float();
          break;
        case 6:
          message.owners.push(BallOwner.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Ball {
    return {
      player: isSet(object.player) ? Number(object.player) : 0,
      color: isSet(object.color) ? Number(object.color) : 0,
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
      radius: isSet(object.radius) ? Number(object.radius) : 0,
      owners: Array.isArray(object?.owners) ? object.owners.map((e: any) => BallOwner.fromJSON(e)) : [],
    };
  },

  toJSON(message: Ball): unknown {
    const obj: any = {};
    message.player !== undefined && (obj.player = Math.round(message.player));
    message.color !== undefined && (obj.color = Math.round(message.color));
    message.x !== undefined && (obj.x = message.x);
    message.y !== undefined && (obj.y = message.y);
    message.radius !== undefined && (obj.radius = message.radius);
    if (message.owners) {
      obj.owners = message.owners.map((e) => e ? BallOwner.toJSON(e) : undefined);
    } else {
      obj.owners = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Ball>, I>>(object: I): Ball {
    const message = createBaseBall();
    message.player = object.player ?? 0;
    message.color = object.color ?? 0;
    message.x = object.x ?? 0;
    message.y = object.y ?? 0;
    message.radius = object.radius ?? 0;
    message.owners = object.owners?.map((e) => BallOwner.fromPartial(e)) || [];
    return message;
  },
};

function createBaseRect(): Rect {
  return { x: 0, y: 0, width: 0, height: 0, rotation: 0 };
}

export const Rect = {
  encode(message: Rect, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.x !== 0) {
      writer.uint32(21).float(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(29).float(message.y);
    }
    if (message.width !== 0) {
      writer.uint32(37).float(message.width);
    }
    if (message.height !== 0) {
      writer.uint32(45).float(message.height);
    }
    if (message.rotation !== 0) {
      writer.uint32(53).float(message.rotation);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Rect {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRect();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.x = reader.float();
          break;
        case 3:
          message.y = reader.float();
          break;
        case 4:
          message.width = reader.float();
          break;
        case 5:
          message.height = reader.float();
          break;
        case 6:
          message.rotation = reader.float();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Rect {
    return {
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
      width: isSet(object.width) ? Number(object.width) : 0,
      height: isSet(object.height) ? Number(object.height) : 0,
      rotation: isSet(object.rotation) ? Number(object.rotation) : 0,
    };
  },

  toJSON(message: Rect): unknown {
    const obj: any = {};
    message.x !== undefined && (obj.x = message.x);
    message.y !== undefined && (obj.y = message.y);
    message.width !== undefined && (obj.width = message.width);
    message.height !== undefined && (obj.height = message.height);
    message.rotation !== undefined && (obj.rotation = message.rotation);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Rect>, I>>(object: I): Rect {
    const message = createBaseRect();
    message.x = object.x ?? 0;
    message.y = object.y ?? 0;
    message.width = object.width ?? 0;
    message.height = object.height ?? 0;
    message.rotation = object.rotation ?? 0;
    return message;
  },
};

function createBaseLobbyGame(): LobbyGame {
  return { gameId: "", players: 0, maxPlayers: 0 };
}

export const LobbyGame = {
  encode(message: LobbyGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== "") {
      writer.uint32(10).string(message.gameId);
    }
    if (message.players !== 0) {
      writer.uint32(16).uint32(message.players);
    }
    if (message.maxPlayers !== 0) {
      writer.uint32(24).uint32(message.maxPlayers);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LobbyGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLobbyGame();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.players = reader.uint32();
          break;
        case 3:
          message.maxPlayers = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LobbyGame {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : "",
      players: isSet(object.players) ? Number(object.players) : 0,
      maxPlayers: isSet(object.maxPlayers) ? Number(object.maxPlayers) : 0,
    };
  },

  toJSON(message: LobbyGame): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    message.players !== undefined && (obj.players = Math.round(message.players));
    message.maxPlayers !== undefined && (obj.maxPlayers = Math.round(message.maxPlayers));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LobbyGame>, I>>(object: I): LobbyGame {
    const message = createBaseLobbyGame();
    message.gameId = object.gameId ?? "";
    message.players = object.players ?? 0;
    message.maxPlayers = object.maxPlayers ?? 0;
    return message;
  },
};

function createBaseLobbyPlayer(): LobbyPlayer {
  return { playerId: 0, name: "" };
}

export const LobbyPlayer = {
  encode(message: LobbyPlayer, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId);
    }
    if (message.name !== "") {
      writer.uint32(26).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LobbyPlayer {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLobbyPlayer();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.playerId = reader.uint32();
          break;
        case 3:
          message.name = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LobbyPlayer {
    return {
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : "",
    };
  },

  toJSON(message: LobbyPlayer): unknown {
    const obj: any = {};
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId));
    message.name !== undefined && (obj.name = message.name);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LobbyPlayer>, I>>(object: I): LobbyPlayer {
    const message = createBaseLobbyPlayer();
    message.playerId = object.playerId ?? 0;
    message.name = object.name ?? "";
    return message;
  },
};

function createBaseLobbyState(): LobbyState {
  return { games: [], players: [] };
}

export const LobbyState = {
  encode(message: LobbyState, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.games) {
      LobbyGame.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.players) {
      LobbyPlayer.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LobbyState {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLobbyState();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.games.push(LobbyGame.decode(reader, reader.uint32()));
          break;
        case 2:
          message.players.push(LobbyPlayer.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LobbyState {
    return {
      games: Array.isArray(object?.games) ? object.games.map((e: any) => LobbyGame.fromJSON(e)) : [],
      players: Array.isArray(object?.players) ? object.players.map((e: any) => LobbyPlayer.fromJSON(e)) : [],
    };
  },

  toJSON(message: LobbyState): unknown {
    const obj: any = {};
    if (message.games) {
      obj.games = message.games.map((e) => e ? LobbyGame.toJSON(e) : undefined);
    } else {
      obj.games = [];
    }
    if (message.players) {
      obj.players = message.players.map((e) => e ? LobbyPlayer.toJSON(e) : undefined);
    } else {
      obj.players = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LobbyState>, I>>(object: I): LobbyState {
    const message = createBaseLobbyState();
    message.games = object.games?.map((e) => LobbyGame.fromPartial(e)) || [];
    message.players = object.players?.map((e) => LobbyPlayer.fromPartial(e)) || [];
    return message;
  },
};

function createBaseGameStart(): GameStart {
  return { gameId: "", players: [], balls: [], rects: [] };
}

export const GameStart = {
  encode(message: GameStart, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== "") {
      writer.uint32(10).string(message.gameId);
    }
    for (const v of message.players) {
      Player.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    for (const v of message.balls) {
      Ball.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    for (const v of message.rects) {
      Rect.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameStart {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameStart();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.players.push(Player.decode(reader, reader.uint32()));
          break;
        case 3:
          message.balls.push(Ball.decode(reader, reader.uint32()));
          break;
        case 4:
          message.rects.push(Rect.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GameStart {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : "",
      players: Array.isArray(object?.players) ? object.players.map((e: any) => Player.fromJSON(e)) : [],
      balls: Array.isArray(object?.balls) ? object.balls.map((e: any) => Ball.fromJSON(e)) : [],
      rects: Array.isArray(object?.rects) ? object.rects.map((e: any) => Rect.fromJSON(e)) : [],
    };
  },

  toJSON(message: GameStart): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    if (message.players) {
      obj.players = message.players.map((e) => e ? Player.toJSON(e) : undefined);
    } else {
      obj.players = [];
    }
    if (message.balls) {
      obj.balls = message.balls.map((e) => e ? Ball.toJSON(e) : undefined);
    } else {
      obj.balls = [];
    }
    if (message.rects) {
      obj.rects = message.rects.map((e) => e ? Rect.toJSON(e) : undefined);
    } else {
      obj.rects = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GameStart>, I>>(object: I): GameStart {
    const message = createBaseGameStart();
    message.gameId = object.gameId ?? "";
    message.players = object.players?.map((e) => Player.fromPartial(e)) || [];
    message.balls = object.balls?.map((e) => Ball.fromPartial(e)) || [];
    message.rects = object.rects?.map((e) => Rect.fromPartial(e)) || [];
    return message;
  },
};

function createBaseGameEnd(): GameEnd {
  return { gameId: "", result: 0, winner: undefined };
}

export const GameEnd = {
  encode(message: GameEnd, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== "") {
      writer.uint32(10).string(message.gameId);
    }
    if (message.result !== 0) {
      writer.uint32(16).int32(message.result);
    }
    if (message.winner !== undefined) {
      Player.encode(message.winner, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameEnd {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameEnd();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.result = reader.int32() as any;
          break;
        case 3:
          message.winner = Player.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GameEnd {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : "",
      result: isSet(object.result) ? gameStatusFromJSON(object.result) : 0,
      winner: isSet(object.winner) ? Player.fromJSON(object.winner) : undefined,
    };
  },

  toJSON(message: GameEnd): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    message.result !== undefined && (obj.result = gameStatusToJSON(message.result));
    message.winner !== undefined && (obj.winner = message.winner ? Player.toJSON(message.winner) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GameEnd>, I>>(object: I): GameEnd {
    const message = createBaseGameEnd();
    message.gameId = object.gameId ?? "";
    message.result = object.result ?? 0;
    message.winner = (object.winner !== undefined && object.winner !== null)
      ? Player.fromPartial(object.winner)
      : undefined;
    return message;
  },
};

function createBaseMove(): Move {
  return { x: 0, y: 0, targetX: 0, targetY: 0 };
}

export const Move = {
  encode(message: Move, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.x !== 0) {
      writer.uint32(8).uint32(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(16).uint32(message.y);
    }
    if (message.targetX !== 0) {
      writer.uint32(24).uint32(message.targetX);
    }
    if (message.targetY !== 0) {
      writer.uint32(32).uint32(message.targetY);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Move {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMove();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.x = reader.uint32();
          break;
        case 2:
          message.y = reader.uint32();
          break;
        case 3:
          message.targetX = reader.uint32();
          break;
        case 4:
          message.targetY = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Move {
    return {
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
      targetX: isSet(object.targetX) ? Number(object.targetX) : 0,
      targetY: isSet(object.targetY) ? Number(object.targetY) : 0,
    };
  },

  toJSON(message: Move): unknown {
    const obj: any = {};
    message.x !== undefined && (obj.x = Math.round(message.x));
    message.y !== undefined && (obj.y = Math.round(message.y));
    message.targetX !== undefined && (obj.targetX = Math.round(message.targetX));
    message.targetY !== undefined && (obj.targetY = Math.round(message.targetY));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Move>, I>>(object: I): Move {
    const message = createBaseMove();
    message.x = object.x ?? 0;
    message.y = object.y ?? 0;
    message.targetX = object.targetX ?? 0;
    message.targetY = object.targetY ?? 0;
    return message;
  },
};

function createBaseTick(): Tick {
  return { gameId: "", cursors: [], balls: [] };
}

export const Tick = {
  encode(message: Tick, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== "") {
      writer.uint32(10).string(message.gameId);
    }
    for (const v of message.cursors) {
      Cursor.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    for (const v of message.balls) {
      Ball.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Tick {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTick();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.cursors.push(Cursor.decode(reader, reader.uint32()));
          break;
        case 3:
          message.balls.push(Ball.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Tick {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : "",
      cursors: Array.isArray(object?.cursors) ? object.cursors.map((e: any) => Cursor.fromJSON(e)) : [],
      balls: Array.isArray(object?.balls) ? object.balls.map((e: any) => Ball.fromJSON(e)) : [],
    };
  },

  toJSON(message: Tick): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    if (message.cursors) {
      obj.cursors = message.cursors.map((e) => e ? Cursor.toJSON(e) : undefined);
    } else {
      obj.cursors = [];
    }
    if (message.balls) {
      obj.balls = message.balls.map((e) => e ? Ball.toJSON(e) : undefined);
    } else {
      obj.balls = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Tick>, I>>(object: I): Tick {
    const message = createBaseTick();
    message.gameId = object.gameId ?? "";
    message.cursors = object.cursors?.map((e) => Cursor.fromPartial(e)) || [];
    message.balls = object.balls?.map((e) => Ball.fromPartial(e)) || [];
    return message;
  },
};

function createBaseGameOptions(): GameOptions {
  return { size: 0, players: 0 };
}

export const GameOptions = {
  encode(message: GameOptions, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.size !== 0) {
      writer.uint32(8).uint32(message.size);
    }
    if (message.players !== 0) {
      writer.uint32(16).uint32(message.players);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameOptions {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameOptions();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.size = reader.uint32();
          break;
        case 2:
          message.players = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GameOptions {
    return {
      size: isSet(object.size) ? Number(object.size) : 0,
      players: isSet(object.players) ? Number(object.players) : 0,
    };
  },

  toJSON(message: GameOptions): unknown {
    const obj: any = {};
    message.size !== undefined && (obj.size = Math.round(message.size));
    message.players !== undefined && (obj.players = Math.round(message.players));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GameOptions>, I>>(object: I): GameOptions {
    const message = createBaseGameOptions();
    message.size = object.size ?? 0;
    message.players = object.players ?? 0;
    return message;
  },
};

function createBasePlayerJoinLobby(): PlayerJoinLobby {
  return { playerId: 0, name: "" };
}

export const PlayerJoinLobby = {
  encode(message: PlayerJoinLobby, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerId !== 0) {
      writer.uint32(8).uint32(message.playerId);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerJoinLobby {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerJoinLobby();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.playerId = reader.uint32();
          break;
        case 2:
          message.name = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PlayerJoinLobby {
    return {
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : "",
    };
  },

  toJSON(message: PlayerJoinLobby): unknown {
    const obj: any = {};
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId));
    message.name !== undefined && (obj.name = message.name);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PlayerJoinLobby>, I>>(object: I): PlayerJoinLobby {
    const message = createBasePlayerJoinLobby();
    message.playerId = object.playerId ?? 0;
    message.name = object.name ?? "";
    return message;
  },
};

function createBasePlayerCreateGame(): PlayerCreateGame {
  return { playerId: 0, name: "", color: "", options: undefined };
}

export const PlayerCreateGame = {
  encode(message: PlayerCreateGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.playerId !== 0) {
      writer.uint32(8).uint32(message.playerId);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.color !== "") {
      writer.uint32(26).string(message.color);
    }
    if (message.options !== undefined) {
      GameOptions.encode(message.options, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerCreateGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerCreateGame();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.playerId = reader.uint32();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.color = reader.string();
          break;
        case 4:
          message.options = GameOptions.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PlayerCreateGame {
    return {
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : "",
      color: isSet(object.color) ? String(object.color) : "",
      options: isSet(object.options) ? GameOptions.fromJSON(object.options) : undefined,
    };
  },

  toJSON(message: PlayerCreateGame): unknown {
    const obj: any = {};
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId));
    message.name !== undefined && (obj.name = message.name);
    message.color !== undefined && (obj.color = message.color);
    message.options !== undefined && (obj.options = message.options ? GameOptions.toJSON(message.options) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PlayerCreateGame>, I>>(object: I): PlayerCreateGame {
    const message = createBasePlayerCreateGame();
    message.playerId = object.playerId ?? 0;
    message.name = object.name ?? "";
    message.color = object.color ?? "";
    message.options = (object.options !== undefined && object.options !== null)
      ? GameOptions.fromPartial(object.options)
      : undefined;
    return message;
  },
};

function createBasePlayerJoinGame(): PlayerJoinGame {
  return { gameId: undefined, playerId: 0, name: "", color: "", options: undefined };
}

export const PlayerJoinGame = {
  encode(message: PlayerJoinGame, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== undefined) {
      writer.uint32(10).string(message.gameId);
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId);
    }
    if (message.name !== "") {
      writer.uint32(26).string(message.name);
    }
    if (message.color !== "") {
      writer.uint32(34).string(message.color);
    }
    if (message.options !== undefined) {
      GameOptions.encode(message.options, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerJoinGame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerJoinGame();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.playerId = reader.uint32();
          break;
        case 3:
          message.name = reader.string();
          break;
        case 4:
          message.color = reader.string();
          break;
        case 5:
          message.options = GameOptions.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PlayerJoinGame {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : undefined,
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
      name: isSet(object.name) ? String(object.name) : "",
      color: isSet(object.color) ? String(object.color) : "",
      options: isSet(object.options) ? GameOptions.fromJSON(object.options) : undefined,
    };
  },

  toJSON(message: PlayerJoinGame): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId));
    message.name !== undefined && (obj.name = message.name);
    message.color !== undefined && (obj.color = message.color);
    message.options !== undefined && (obj.options = message.options ? GameOptions.toJSON(message.options) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PlayerJoinGame>, I>>(object: I): PlayerJoinGame {
    const message = createBasePlayerJoinGame();
    message.gameId = object.gameId ?? undefined;
    message.playerId = object.playerId ?? 0;
    message.name = object.name ?? "";
    message.color = object.color ?? "";
    message.options = (object.options !== undefined && object.options !== null)
      ? GameOptions.fromPartial(object.options)
      : undefined;
    return message;
  },
};

function createBasePlayerMove(): PlayerMove {
  return { gameId: "", playerNumber: 0, x: 0, y: 0 };
}

export const PlayerMove = {
  encode(message: PlayerMove, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== "") {
      writer.uint32(10).string(message.gameId);
    }
    if (message.playerNumber !== 0) {
      writer.uint32(16).uint32(message.playerNumber);
    }
    if (message.x !== 0) {
      writer.uint32(24).uint32(message.x);
    }
    if (message.y !== 0) {
      writer.uint32(32).uint32(message.y);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerMove {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerMove();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.playerNumber = reader.uint32();
          break;
        case 3:
          message.x = reader.uint32();
          break;
        case 4:
          message.y = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PlayerMove {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : "",
      playerNumber: isSet(object.playerNumber) ? Number(object.playerNumber) : 0,
      x: isSet(object.x) ? Number(object.x) : 0,
      y: isSet(object.y) ? Number(object.y) : 0,
    };
  },

  toJSON(message: PlayerMove): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    message.playerNumber !== undefined && (obj.playerNumber = Math.round(message.playerNumber));
    message.x !== undefined && (obj.x = Math.round(message.x));
    message.y !== undefined && (obj.y = Math.round(message.y));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PlayerMove>, I>>(object: I): PlayerMove {
    const message = createBasePlayerMove();
    message.gameId = object.gameId ?? "";
    message.playerNumber = object.playerNumber ?? 0;
    message.x = object.x ?? 0;
    message.y = object.y ?? 0;
    return message;
  },
};

function createBasePlayerLeave(): PlayerLeave {
  return { gameId: "", playerId: 0 };
}

export const PlayerLeave = {
  encode(message: PlayerLeave, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameId !== "") {
      writer.uint32(10).string(message.gameId);
    }
    if (message.playerId !== 0) {
      writer.uint32(16).uint32(message.playerId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerLeave {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerLeave();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.gameId = reader.string();
          break;
        case 2:
          message.playerId = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PlayerLeave {
    return {
      gameId: isSet(object.gameId) ? String(object.gameId) : "",
      playerId: isSet(object.playerId) ? Number(object.playerId) : 0,
    };
  },

  toJSON(message: PlayerLeave): unknown {
    const obj: any = {};
    message.gameId !== undefined && (obj.gameId = message.gameId);
    message.playerId !== undefined && (obj.playerId = Math.round(message.playerId));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PlayerLeave>, I>>(object: I): PlayerLeave {
    const message = createBasePlayerLeave();
    message.gameId = object.gameId ?? "";
    message.playerId = object.playerId ?? 0;
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

type DeepPartial<T> = T extends Builtin ? T
  : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
