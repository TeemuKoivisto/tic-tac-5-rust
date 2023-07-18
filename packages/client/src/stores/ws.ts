import {
  ClientEventMap,
  ClientMsgType,
  ServerMsgType,
  LobbyState,
  PlayerJoinGame,
  PlayerJoinLobby,
  PlayerStatus,
  GameStart,
  GameEnd,
  GameMove,
  GamePlayerConnection,
  PlayerCreateGame,
  PlayerSelectCell,
} from '@tt5/prototypes'

import { jwt } from './auth'
import { WS_URL } from '../config'
import { log } from '../logger'
import type { SocketEvent } from '../types/events'
import { get } from 'svelte/store'

let socket: WebSocket | null = null

export const socketActions = {
  connect(cb: (evt: SocketEvent) => void) {
    socket = new WebSocket(`${WS_URL}?jwt=${get(jwt)?.token}`)
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
        case ServerMsgType.player_status:
          cb({ e: ServerMsgType.player_status, data: PlayerStatus.decode(payload) })
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
        case ServerMsgType.game_player_move: {
          cb({ e: ServerMsgType.game_player_move, data: GameMove.decode(payload) })
          break
        }
        case ServerMsgType.player_disconnected: {
          cb({ e: ServerMsgType.player_disconnected, data: GamePlayerConnection.decode(payload) })
          break
        }
        case ServerMsgType.player_reconnected: {
          cb({ e: ServerMsgType.player_reconnected, data: GamePlayerConnection.decode(payload) })
          break
        }
        default:
          log.error(`Unknown message type: ${messageType}`)
      }
    }
  },
  emit<K extends keyof ClientEventMap>(...args: ClientEventMap[K]) {
    let data: Uint8Array | undefined
    console.log('type', args[0])
    switch (args[0]) {
      case ClientMsgType.join_lobby:
        data = PlayerJoinLobby.encode(args[1]).finish()
        break
      case ClientMsgType.create_lobby_game:
        data = PlayerCreateGame.encode(args[1]).finish()
        break
      case ClientMsgType.join_lobby_game:
        data = PlayerJoinGame.encode(args[1]).finish()
        break
      case ClientMsgType.player_select_cell:
        data = PlayerSelectCell.encode(args[1]).finish()
        break
      case ClientMsgType.player_rejoin:
        data = PlayerJoinGame.encode(args[1]).finish()
        break
    }
    if (!data) {
      throw Error(`Unknown event! ${args[0]}, ${JSON.stringify(args[1])}`)
    }
    const withType = new Uint8Array(data.length + 1)
    withType.set([args[0]], 0)
    withType.set(data, 1)
    socket?.send(withType)
  },
}
