import { get, writable } from 'svelte/store'

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
    startTime: Date.now(),
    turns: 10,
  },
})
export const openModal = writable<EModal | null>(null)

export const modalActions = {
  open<K extends keyof ModalParams>(name: K, params: ModalParams[K]) {
    console.log('open ' + name)
    console.log(params)
    openModal.set(name)
    modals.update(m => ({ ...m, [name]: params }))
  },
  close() {
    console.log('close')
    openModal.set(null)
  },
  toggle(modal: EModal) {
    const opened = get(openModal)
    if (!opened || (opened && opened !== modal)) {
      openModal.set(modal)
    } else {
      openModal.set(null)
    }
  }
}
