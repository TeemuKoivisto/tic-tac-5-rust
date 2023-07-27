export * from './protos/client_events'
export * from './protos/game'
export * from './protos/server_events'

import {
  ClientMsgType,
  PlayerCreateGame,
  PlayerJoinGame,
  PlayerJoinLobby,
  PlayerSelectCell,
} from './protos/client_events'

export type ClientEvent<T, P> = [T, P]

export interface ClientEventMap {
  [ClientMsgType.join_lobby]: ClientEvent<ClientMsgType.join_lobby, PlayerJoinLobby>
  [ClientMsgType.create_lobby_game]: ClientEvent<ClientMsgType.create_lobby_game, PlayerCreateGame>
  [ClientMsgType.join_lobby_game]: ClientEvent<ClientMsgType.join_lobby_game, PlayerJoinGame>
  [ClientMsgType.player_select_cell]: ClientEvent<
    ClientMsgType.player_select_cell,
    PlayerSelectCell
  >
}
