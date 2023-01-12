import { writable } from 'svelte/store'

export enum EModal {
  GAME_OVER = 'GAME_OVER_MODAL',
}
export interface GameOverParams {
  playerWon: boolean
  startTime: number
  turns: number
}

export type ModalParams = {
  [EModal.GAME_OVER]: GameOverParams
}

export const modals = writable<ModalParams>({
  [EModal.GAME_OVER]: {
    playerWon: true,
    startTime: 0,
    turns: 0,
  },
})
export const openModal = writable<EModal | null>(null)

export const modalActions = {
  open<K extends keyof ModalParams>(name: K, params: ModalParams[K]) {
    openModal.set(name)
    modals.update(m => {
      m[name] = params
      return m
    })
  },
  close() {
    openModal.set(null)
  },
}
