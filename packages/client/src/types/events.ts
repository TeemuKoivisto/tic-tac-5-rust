import {
  GameStart,
  GameEnd,
  LobbyState,
  GameMove,
  ServerMsgType,
  GamePlayerConnection,
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
      data: GamePlayerConnection
    }
  | {
      e: ServerMsgType.player_reconnected
      data: GamePlayerConnection
    }
  | {
      e: 'error'
      err: Event
    }
  | {
      e: 'finally'
    }
