import { derived, get, writable } from 'svelte/store'
import {
  Cell,
  GameEnd,
  Player,
  ServerMsgType,
  ClientMsgType,
  LobbyGame,
  LobbyPlayer,
  LobbyState,
  PlayerCreateGame,
  PlayerJoinGame,
  PlayerJoinLobby,
  PlayerSelectCell,
  CellType,
} from '@tt5/prototypes'

import { socketActions } from './ws'
import { log } from '../logger'

import type { GameState, SocketEvent, Options } from '../types'

export const lobbyGames = writable<LobbyGame[]>([])
export const lobbyPlayers = writable<LobbyPlayer[]>([])

export const playerName = writable('unknown')

export const gameState = writable<GameState>('connecting')
export const gameEnd = writable<GameEnd | undefined>(undefined)
export const gameId = writable<string>('')
export const players = writable<Player[]>([])
export const cells = writable<Map<string, Cell>>(new Map())
export const gridSize = derived(cells, cells => Math.sqrt(cells.size))
export const playerId = Math.ceil(Math.random() * 100000) // TODO: remove client-side id generation
export const localPlayer = writable<Player | undefined>(undefined)
export const retryConnectTimeout = writable<ReturnType<typeof setTimeout> | undefined>()

function handleMessages(evt: SocketEvent) {
  log.debug('Event:', evt)
  console.log('event', evt)
  switch (evt.e) {
    case 'connected':
      socketActions.emitJoinLobby({
        playerId,
        name: get(playerName),
      })
      gameState.set('lobby')
      break
    case 'disconnected':
      gameState.set('connecting')
      retryConnectTimeout.set(
        setTimeout(() => {
          socketActions.connect(handleMessages)
        }, 1000)
      )
      break
    case ServerMsgType.lobby_state:
      lobbyGames.set(evt.data.games)
      lobbyPlayers.set(evt.data.players)
      break
    case ServerMsgType.game_start:
      gameId.set(evt.data.gameId)
      players.set(evt.data.players)
      cells.set(new Map(evt.data.cells.map(c => [`${c.x}:${c.y}`, c])))
      localPlayer.set(evt.data.players.find(p => p.id === playerId))
      gameState.set('game-running')
      break
    case ServerMsgType.game_end:
      gameEnd.set(evt.data)
      gameState.set('game-ended')
      break
    case ServerMsgType.game_player_move:
      const key = `${evt.data.x}:${evt.data.y}`
      cells.update(cells => {
        const old = cells.get(key)
        if (!old) {
          throw Error(`Unable to find cell at coords: ${key}`)
        }
        cells.set(key, {
          cellType: CellType.PLAYER_CELL,
          player: evt.data.player,
          x: old.x,
          y: old.y,
        })
        return cells
      })
      break
  }
}

export const gameActions = {
  setPlayerName(name: string) {
    playerName.set(name)
  },
  runGame() {
    gameState.set('connecting')
    socketActions.connect(handleMessages)
  },
  joinLobby() {
    gameState.set('lobby')
    socketActions.emit(
      ClientMsgType.join_lobby,
      PlayerJoinLobby.encode({
        playerId,
        name: get(playerName),
      }).finish()
    )
  },
  createGame(opts: Options) {
    const payload = {
      playerId,
      name: get(playerName),
      preferredSymbol: 'X',
      options: opts,
    }
    socketActions.emit(ClientMsgType.create_lobby_game, PlayerCreateGame.encode(payload).finish())
    gameState.set('waiting-game-start')
  },
  joinGame(game: LobbyGame, opts: Options) {
    const payload = {
      gameId: game.gameId,
      playerId,
      name: get(playerName),
      color: 'poop',
      options: opts,
    }
    socketActions.emit(ClientMsgType.join_lobby_game, PlayerJoinGame.encode(payload).finish())
    gameState.set('waiting-game-start')
  },
  playerSelectCell(x: number, y: number) {
    const payload = {
      gameId: get(gameId),
      playerNumber: get(localPlayer)?.playerNumber || 0,
      x,
      y,
    }
    socketActions.emit(ClientMsgType.player_select_cell, PlayerSelectCell.encode(payload).finish())
  },
}
