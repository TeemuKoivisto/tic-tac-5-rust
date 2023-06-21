export interface LoginResponse {
  player_id: number
  token: string
  expires: number
}

export interface Player {
  player_id: number
  name: string
}
export interface Jwt {
  token: string
  expires: number
}
