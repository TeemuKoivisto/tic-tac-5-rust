import App from './App.svelte'

import './index.css'

const el = document.querySelector('#app')
if (el) {
  new App({ target: el })
}
