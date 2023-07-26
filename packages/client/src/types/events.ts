import {
  BoardState,
  GameEnd,
  LobbyState,
  GameMove,
  ServerMsgType,
  PlayerState,
  GamePlayerDisconnected,
  GamePlayerReconnected,
  PlayerJoinedGame,
} from '@tt5/prototypes'

export type SocketEvent =
  | {
      e: 'connected'
    }
  | {
      e: 'disconnected'
    }
  | {
      e: ServerMsgType.player_state
      data: PlayerState
    }
  | {
      e: ServerMsgType.player_disconnected
      data: GamePlayerDisconnected
    }
  | {
      e: ServerMsgType.player_reconnected
      data: GamePlayerReconnected
    }
  | {
      e: ServerMsgType.lobby_state
      data: LobbyState
    }
  | {
      e: ServerMsgType.player_joined_game
      data: PlayerJoinedGame
    }
  | {
      e: ServerMsgType.game_start
      data: BoardState
    }
  | {
      e: ServerMsgType.game_player_move
      data: GameMove
    }
  | {
      e: ServerMsgType.game_end
      data: GameEnd
    }
  | {
      e: 'error'
      err: Event
    }
  | {
      e: 'finally'
    }
