import { get, writable } from 'svelte/store'

export enum EModal {
  GAME_OVER = 'GAME_OVER_MODAL',
  PLAYER_DISCONNECTED = 'PLAYER_DISCONNECTED',
}
export interface GameOverParams {
  playerWon: boolean
  startTime: number
  turns: number
}
export interface PlayerDisconnectParams {
  playerName: string
}

export type ModalParams = {
  [EModal.GAME_OVER]: GameOverParams
  [EModal.PLAYER_DISCONNECTED]: PlayerDisconnectParams
}

export const modals = writable<ModalParams>({
  [EModal.GAME_OVER]: {
    playerWon: true,
    startTime: 0,
    turns: 0,
  },
  [EModal.PLAYER_DISCONNECTED]: {
    playerName: '',
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
