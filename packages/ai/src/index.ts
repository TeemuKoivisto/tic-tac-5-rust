import App from './components/App.svelte'
import './index.css'

import { createCell, getCellValue, setAdjacency, setDirAdjacency, getOwner, setOwner } from './cell'

const c = createCell(45, 63)
console.log('c', c)
console.log(getCellValue(c))
const c2 = setAdjacency(c, 2, 4, 6, 1)
console.log('c2', c2)
console.log(getCellValue(c2))
const c3 = setOwner(c2, 2)
console.log('c3', c3)
console.log(getCellValue(c3))
console.log(getCellValue(setDirAdjacency(c3, 1, 3)))
console.log(getCellValue(setDirAdjacency(c3, 0, 11)))
console.log(getCellValue(setDirAdjacency(c3, 3, 11)))

// @ts-ignore
window.tt = {
  createCell,
  getCellValue,
  setAdjacency,
  getOwner,
  setOwner,
}

const el = document.querySelector('#app')
if (el) {
  new App({ target: el })
}
