import {
  GameStart,
  GameEnd,
  LobbyState,
  GameMove,
  ServerMsgType,
  PlayerStatus,
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
      data: PlayerStatus
    }
  | {
      e: ServerMsgType.game_start
      data: GameStart
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
