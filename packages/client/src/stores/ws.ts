import {
  ClientMsgType,
  ServerMsgType,
  LobbyState,
  PlayerJoinGame,
  PlayerJoinLobby,
  PlayerMove,
  GameStart,
  GameEnd,
} from '@tt5/prototypes'
import { WS_URL } from '../config'
import { log } from '../logger'
import type { SocketEvent } from '../types/events'

let socket: WebSocket | null = null

export const socketActions = {
  connect(cb: (evt: SocketEvent) => void) {
    socket = new WebSocket(WS_URL)
    socket.binaryType = 'arraybuffer'
    socket.onopen = () => {
      console.log('Connected')
      cb({ e: 'connected' })
    }
    socket.onerror = ev => {
      console.debug('Socket error', ev)
      cb({ e: 'error', err: ev })
    }
    socket.onclose = () => {
      console.log('Disconnected')
      socket = null
      cb({ e: 'disconnected' })
    }
    socket.onmessage = e => {
      const { data } = e
      const uarray = new Uint8Array(data)
      const payload = uarray.slice(1)
      const messageType = uarray[0]
      switch (messageType) {
        case ServerMsgType.lobby_state:
          cb({ e: ServerMsgType.lobby_state, data: LobbyState.decode(payload) })
          break
        case ServerMsgType.player_msg:
        case ServerMsgType.player_join_lobby:
        case ServerMsgType.player_leave_lobby:
        case ServerMsgType.lobby_game_updated:
        case ServerMsgType.player_join:
          log.debug(`Read type ${messageType}`)
          break
        case ServerMsgType.game_start: {
          cb({ e: ServerMsgType.game_start, data: GameStart.decode(payload) })
          break
        }
        case ServerMsgType.game_end: {
          cb({ e: ServerMsgType.game_end, data: GameEnd.decode(payload) })
          break
        }
        // case ServerMsgType.tick: {
        //   cb({ e: ServerMsgType.tick, data: Tick.decode(payload) })
        //   break
        // }
        default:
          log.error(`Unknown message type: ${messageType}`)
      }
    }
  },
  emit(type: ClientMsgType, data: Uint8Array) {
    const withType = new Uint8Array(data.length + 1)
    withType.set([type], 0)
    withType.set(data, 1)
    socket?.send(withType)
  },
  emitJoinLobby(payload: PlayerJoinLobby) {
    const data = PlayerJoinLobby.encode(payload).finish()
    const poop = new Uint8Array(data.length + 1)
    poop.set([ClientMsgType.join_lobby], 0)
    poop.set(data, 1)
    socket?.send(poop)
  },
  emitJoinGame(payload: PlayerJoinGame) {
    console.log('emitJoin: ', payload)
    const data = PlayerJoinGame.encode(payload).finish()
    const poop = new Uint8Array(data.length + 1)
    poop.set([ClientMsgType.join_lobby_game], 0)
    poop.set(data, 1)
    socket?.send(poop)
  },
  emitMove(payload: PlayerMove) {
    // console.log('emitMove: ', payload)
    const data = PlayerMove.encode(payload).finish()
    const poop = new Uint8Array(data.length + 1)
    poop.set([ClientMsgType.player_move], 0)
    poop.set(data, 1)
    socket?.send(poop)
  },
}
