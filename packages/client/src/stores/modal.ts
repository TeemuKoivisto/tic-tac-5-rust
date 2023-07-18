import { GamePlayerDisconnected, GameStatus } from '@tt5/prototypes'
import { get, writable } from 'svelte/store'

export enum EModal {
  GAME_OVER = 'GAME_OVER_MODAL',
  PLAYER_DISCONNECTED = 'PLAYER_DISCONNECTED',
}
export interface GameOverParams {
  player: {
    symbol: 'X' | 'O'
    name: string
  }
  result: GameStatus
  startTime: number
  turns: number
}

export type ModalParams = {
  [EModal.GAME_OVER]: GameOverParams
  [EModal.PLAYER_DISCONNECTED]: GamePlayerDisconnected
}

export const modals = writable<ModalParams>({
  [EModal.GAME_OVER]: {
    player: {
      symbol: 'X',
      name: '',
    },
    result: GameStatus.WAITING,
    startTime: 0,
    turns: 0,
  },
  [EModal.PLAYER_DISCONNECTED]: {
    gameId: '',
    playerId: 0,
    symbol: '',
    name: '',
  },
})
export const openModal = writable<EModal | null>(null)

export const modalActions = {
  open<K extends keyof ModalParams>(name: K, params: ModalParams[K]) {
    openModal.set(name)
    modals.update(m => ({ ...m, [name]: params }))
  },
  close() {
    openModal.set(null)
  },
  toggle(modal: EModal) {
    const opened = get(openModal)
    if (!opened || (opened && opened !== modal)) {
      openModal.set(modal)
    } else {
      openModal.set(null)
    }
  },
}
