import { derived, get, writable } from 'svelte/store'
import {
  Cell,
  GameEnd,
  GameMove,
  Player,
  PlayerAppState,
  PlayerInGameState,
  ServerMsgType,
  ClientMsgType,
  LobbyGame,
  LobbyPlayer,
  CellType,
} from '@tt5/prototypes'

import { gameState, stateActions } from './state'
import { playerName, playerId } from './auth'
import { modalActions, EModal } from './modal'
import { socketActions } from './ws'
import { log } from '../logger'

import type { SocketEvent, Options } from '../types'

export const lobbyGames = writable<LobbyGame[]>([])
export const lobbyPlayers = writable<LobbyPlayer[]>([])

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
export const playerInTurn = writable<Player | undefined>()
export const isInTurn = derived(
  [gameState, localPlayer],
  ([s, p]) =>
    (s === PlayerInGameState.x_turn && p?.symbol === 'X') ||
    (s === PlayerInGameState.o_turn && p?.symbol === 'O')
)
// export const wasOwnMove = derived(
//   [lastMove, localPlayer],
//   ([move, player]) => move?.playerId && move.playerId === player?.id
// )

export function handleMessages(evt: SocketEvent) {
  log.debug('Event:', evt)
  console.log('event', evt)
  switch (evt.e) {
    case 'connected':
      socketActions.emit(ClientMsgType.join_lobby, {
        playerId: get(playerId),
        name: get(playerName),
      })
      break
    case 'disconnected':
      retryConnectTimeout.set(
        setTimeout(() => {
          socketActions.connect(handleMessages)
        }, 1000)
      )
      stateActions.transitApp(PlayerAppState.disconnected)
      break
    case ServerMsgType.lobby_state:
      lobbyGames.set(evt.data.games)
      lobbyPlayers.set(evt.data.players)
      gameActions.reset()
      stateActions.transitApp(PlayerAppState.lobby)
      break
    case ServerMsgType.player_status:
      const waitingGameId = evt.data.waitingGames[0]
      if (waitingGameId) {
        socketActions.emit(ClientMsgType.player_rejoin, {
          gameId: waitingGameId,
        })
        stateActions.transitGame(PlayerInGameState.paused)
      }
      break
    case ServerMsgType.game_start:
      gameActions.reset()
      const pId = get(playerId)
      gameId.set(evt.data.gameId)
      players.set(evt.data.players)
      cells.set(new Map(evt.data.cells.map(c => [`${c.x}:${c.y}`, c])))
      localPlayer.set(evt.data.players.find(p => p.id === pId))
      // @TODO initialize properly to handle disconnects
      gameStarted.set(Date.now())
      playerInTurn.set(evt.data.players.find(p => p.id === evt.data.playerInTurn))
      stateActions.transitApp(PlayerAppState.in_game)
      stateActions.transitGame(evt.data.state)
      // if (evt.data.playerInTurn === pId) {
      //   stateActions.transitGame(PlayerInGameState.your_turn)
      // } else {
      //   stateActions.transitGame(PlayerInGameState.opponent_turn)
      // }
      break
    case ServerMsgType.game_end:
      const player = get(localPlayer)
      modalActions.open(EModal.GAME_OVER, {
        player: {
          symbol: player?.symbol === 'X' ? 'X' : 'O',
          name: get(playerName),
        },
        result: evt.data.result,
        startTime: get(gameStarted),
        turns: get(gameTurns),
      })
      gameEnd.set(evt.data)
      stateActions.transitGame(PlayerInGameState.ended)
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
          player: evt.data.playerNumber,
          x: old.x,
          y: old.y,
        })
        return cells
      })
      playerInTurn.set(get(players).find(p => p.playerNumber === evt.data.nextInTurn))
      stateActions.transitGame(evt.data.state)
      // if (evt.data.nextInTurn === get(localPlayer)?.playerNumber) {
      //   stateActions.transitGame(PlayerInGameState.your_turn)
      // } else {
      //   stateActions.transitGame(PlayerInGameState.opponent_turn)
      // }
      break
    case ServerMsgType.player_disconnected:
      modalActions.open(EModal.PLAYER_DISCONNECTED, evt.data)
      stateActions.transitGame(evt.data.state)
      break
    case ServerMsgType.player_reconnected:
      modalActions.close()
      stateActions.transitGame(evt.data.state)
      break
  }
}

export const gameActions = {
  joinLobby() {
    socketActions.emit(ClientMsgType.join_lobby, {
      playerId: get(playerId),
      name: get(playerName),
    })
    stateActions.transitApp(PlayerAppState.lobby)
  },
  createGame(opts: Options) {
    socketActions.emit(ClientMsgType.create_lobby_game, {
      playerId: get(playerId),
      name: get(playerName),
      preferredSymbol: 'X',
      options: opts,
    })
    stateActions.transitApp(PlayerAppState.waiting_game_start)
  },
  joinGame(game: LobbyGame, opts: Options) {
    socketActions.emit(ClientMsgType.join_lobby_game, {
      gameId: game.gameId,
      playerId: get(playerId),
      name: get(playerName),
      options: opts,
    })
    // Set lobbyGames empty so that old games won't show up when returning to front page
    lobbyGames.set([])
    stateActions.transitApp(PlayerAppState.waiting_game_start)
  },
  playerSelectCell(x: number, y: number) {
    socketActions.emit(ClientMsgType.player_select_cell, {
      gameId: get(gameId),
      x,
      y,
    })
  },
  reset() {
    gameEnd.set(undefined)
    gameId.set('')
    gameStarted.set(0)
    gameTurns.set(0)

    players.set([])
    cells.set(new Map())
    localPlayer.set(undefined)
    lastMove.set(undefined)
    playerInTurn.set(undefined)
  },
}
