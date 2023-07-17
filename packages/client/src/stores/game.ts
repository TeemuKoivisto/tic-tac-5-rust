import { derived, get, writable } from 'svelte/store'
import {
  Cell,
  GameEnd,
  GameMove,
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

import { playerName, playerId } from './auth'
import { modalActions, EModal } from './modal'
import { socketActions } from './ws'
import { log } from '../logger'

import type { GameState, SocketEvent, Options } from '../types'

export const lobbyGames = writable<LobbyGame[]>([])
export const lobbyPlayers = writable<LobbyPlayer[]>([])

export const gameState = writable<GameState>('connecting')
export const gameEnd = writable<GameEnd | undefined>(undefined)
export const gameId = writable<string>('')
export const gameStarted = writable<number>(0)
export const gameTurns = writable<number>(0)

export const players = writable<Player[]>([])
export const cells = writable<Map<string, Cell>>(new Map())
export const gridSize = derived(cells, cells => Math.sqrt(cells.size))
export const localPlayer = writable<Player | undefined>(undefined)
export const retryConnectTimeout = writable<ReturnType<typeof setTimeout> | undefined>()
export const lastMove = writable<GameMove | undefined>(undefined)
export const wasOwnMove = derived(
  [lastMove, localPlayer],
  ([m, p]) => m?.player && m.player === p?.playerNumber
)

function handleMessages(evt: SocketEvent) {
  log.debug('Event:', evt)
  console.log('event', evt)
  switch (evt.e) {
    case 'connected':
      socketActions.emitJoinLobby({
        playerId: get(playerId),
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
      const pId = get(playerId)
      gameId.set(evt.data.gameId)
      players.set(evt.data.players)
      cells.set(new Map(evt.data.cells.map(c => [`${c.x}:${c.y}`, c])))
      localPlayer.set(evt.data.players.find(p => p.id === pId))
      gameState.set('game-running')
      gameStarted.set(Date.now())
      break
    case ServerMsgType.game_end:
      modalActions.open(EModal.GAME_OVER, {
        playerWon: evt.data.winnerId === get(playerId),
        startTime: get(gameStarted),
        turns: get(gameTurns),
      })
      gameState.set('game-ended')
      gameEnd.set(evt.data)
      lastMove.set(undefined)
      gameStarted.set(0)
      gameTurns.set(0)
      break
    case ServerMsgType.game_player_move:
      const key = `${evt.data.x}:${evt.data.y}`
      lastMove.set(evt.data)
      gameTurns.update(t => t + 1)
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
    case ServerMsgType.player_disconnected:
      modalActions.open(EModal.PLAYER_DISCONNECTED, {
        playerName: 'poop',
      })
      break
    case ServerMsgType.player_reconnected:
      modalActions.close()
      break
  }
}

export const gameActions = {
  runGame() {
    gameState.set('connecting')
    socketActions.connect(handleMessages)
  },
  joinLobby() {
    gameState.set('lobby')
    socketActions.emit(
      ClientMsgType.join_lobby,
      PlayerJoinLobby.encode({
        playerId: get(playerId),
        name: get(playerName),
      }).finish()
    )
  },
  createGame(opts: Options) {
    const payload = {
      playerId: get(playerId),
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
      playerId: get(playerId),
      name: get(playerName),
      color: 'poop',
      options: opts,
    }
    socketActions.emit(ClientMsgType.join_lobby_game, PlayerJoinGame.encode(payload).finish())
    gameState.set('waiting-game-start')
    // Set lobbyGames empty so that old games won't show up when returning to front page
    lobbyGames.set([])
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
