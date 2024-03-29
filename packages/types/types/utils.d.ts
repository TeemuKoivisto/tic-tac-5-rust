export type Ok<T> = {
  data: T
}
export type Error = {
  err: string
  code: number
}
export type Result<T> = Ok<T> | Error
