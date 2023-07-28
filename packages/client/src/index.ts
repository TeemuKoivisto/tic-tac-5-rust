import App from './components/App.svelte'
import './index.css'

import { enableDebug } from './logger'

enableDebug(true)

const el = document.querySelector('#app')
if (el) {
  new App({ target: el })
}
