import { GameStart, Tick as GameTick, GameEnd, LobbyGames, ServerMsgType } from '@tt5/prototypes'

export type SocketEvent =
  | {
      e: 'connected'
    }
  | {
      e: 'disconnected'
    }
  | {
      e: ServerMsgType.lobby_state
      data: LobbyGames
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
      e: ServerMsgType.tick
      data: GameTick
    }
  | {
      e: 'error'
      err: Event
    }
  | {
      e: 'finally'
    }
