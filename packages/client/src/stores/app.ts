import { authActions } from './auth'
import { gameActions, handleMessages } from './game'
import { AppState, stateActions } from './state'
import { socketActions } from './ws'

let retryTimeout: ReturnType<typeof setTimeout> | undefined

export async function run() {
  const result = await authActions.login()
  if ('err' in result) {
    stateActions.transitApp(AppState.errored)
    retryTimeout = setTimeout(() => {
      run()
    }, 5000)
  } else {
    stateActions.transitApp(AppState.connecting)
    socketActions.connect(handleMessages)
  }
}
