import { get, writable } from 'svelte/store'
import {
  Ball,
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
export const balls = writable<Ball[]>([])
export const playerId = Math.ceil(Math.random() * 100000) // TODO: remove client-side id generation
export const localPlayer = writable<Player | undefined>(undefined)
export const retryConnectTimeout = writable<ReturnType<typeof setTimeout> | undefined>()

function handleMessages(evt: SocketEvent) {
  log.debug('Event:', evt)
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
      balls.set(evt.data.balls)
      localPlayer.set(evt.data.players.find(p => p.id === playerId))
      // gfxActions.initGame(evt.data.players, evt.data.balls, evt.data.rects)
      gameState.set('game-running')
      break
    case ServerMsgType.game_end:
      gameEnd.set(evt.data)
      // gfxActions.destroy()
      gameState.set('game-ended')
      break
    case ServerMsgType.tick:
      // const tick = evt.data
      // players.update(plrs => {
      //   tick.cursors.forEach((cursor, i) => {
      //     plrs[i].cursor = cursor
      //   })
      //   return plrs
      // })
      // const mouz = get(mouse)
      // socketActions.emitMove({
      //   gameId: tick.gameId,
      //   playerNumber: get(localPlayer)?.playerNumber || 0,
      //   mouseX: mouz.x,
      //   mouseY: mouz.y,
      // })
      // // const cursors = get(globalCursors)
      // // for (let i = 0; i < tick.cursors.length; i++) {
      // //   const serverCursor = tick.cursors[i]
      // //   const clientCursor = cursors.find(c => c.player === serverCursor.player)!
      // //   clientCursor.targetX = serverCursor.x
      // //   clientCursor.targetY = serverCursor.y
      // // }
      // gfxActions.updateBalls(tick.balls)
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
}
