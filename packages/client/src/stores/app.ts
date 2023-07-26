import { PlayerAppState } from '@tt5/prototypes'
import { authActions } from './auth'
import { gameActions, handleMessages } from './game'
import { stateActions } from './state'
import { socketActions } from './ws'

let retryTimeout: ReturnType<typeof setTimeout> | undefined

export async function run() {
  const result = await authActions.login()
  if ('err' in result) {
    stateActions.transitApp(PlayerAppState.errored)
    retryTimeout = setTimeout(() => {
      run()
    }, 5000)
  } else {
    stateActions.transitApp(PlayerAppState.disconnected)
    socketActions.connect(handleMessages)
  }
}
