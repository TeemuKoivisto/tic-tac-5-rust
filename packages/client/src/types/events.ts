import {
  BoardState,
  GameEnd,
  LobbyState,
  GameMove,
  ServerMsgType,
  PlayerState,
  GamePlayerDisconnected,
  GamePlayerReconnected,
} from '@tt5/prototypes'

export type SocketEvent =
  | {
      e: 'connected'
    }
  | {
      e: 'disconnected'
    }
  | {
      e: ServerMsgType.lobby_state
      data: LobbyState
    }
  | {
      e: ServerMsgType.player_status
      data: PlayerState
    }
  | {
      e: ServerMsgType.game_start
      data: BoardState
    }
  | {
      e: ServerMsgType.game_end
      data: GameEnd
    }
  | {
      e: ServerMsgType.game_player_move
      data: GameMove
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
      e: 'error'
      err: Event
    }
  | {
      e: 'finally'
    }
