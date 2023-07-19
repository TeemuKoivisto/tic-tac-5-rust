import { Writable, derived, get, writable } from 'svelte/store'
import { wrappedFetch } from '@tt5/types'

import { API_URL } from '../config'

import type { Result } from '@tt5/types'
import type { Jwt, Player, LoginResponse } from '../types'

export const player = persist(
  writable<Player>({
    player_id: 0,
    name: 'unknown',
  }),
  'auth/player'
)
export const jwt = persist(writable<Jwt | null>(null), 'auth/jwt')
export const playerId = derived(player, player => player.player_id)
export const playerName = derived(player, player => player.name)

function persist<T>(store: Writable<T>, storageKey: string) {
  let persisted = null
  try {
    persisted = localStorage.getItem(storageKey)
  } catch (err) {}
  if (persisted && persisted.length > 0) {
    try {
      const parsed = JSON.parse(persisted)
      store.set(parsed)
    } catch (err) {}
  }
  store.subscribe(val => {
    localStorage.setItem(storageKey, JSON.stringify(val))
  })
  return store
}

export const authActions = {
  setPlayerName(name: string) {
    player.update(p => ({ ...p, name }))
  },
  async login(): Promise<Result<true | LoginResponse>> {
    const jwtToken = get(jwt)
    if (jwtToken && jwtToken.expires > Date.now() / 1000) {
      return { data: true }
    }
    const resp = await wrappedFetch<LoginResponse>(`${API_URL}/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    })
    if ('data' in resp) {
      player.update(p => ({ ...p, player_id: resp.data.player_id }))
      jwt.set({ token: resp.data.token, expires: resp.data.expires })
    }
    return resp
  },
}
