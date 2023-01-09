#!/usr/bin/env ts-node

import fg from 'fast-glob'
import { promisify } from 'util'
import { exec as rawExec } from 'child_process'
import { promises as fs } from 'fs'
import path from 'path'

const exec = promisify(rawExec)

const arg = process.argv[2]

const OUT_DIR = 'out-ts'

async function readJson(path: string) {
  const pkg = await fs.readFile(path, 'utf-8').catch(err => console.error(err))
  if (!pkg) return undefined
  return JSON.parse(pkg)
}

async function cleanCurrentExports() {
  const pkg = await readJson('./package.json')
  pkg.exports = {}
  await fs.writeFile('./package.json', JSON.stringify(pkg, null, 2))
}

// async function writePackageExports() {
//   const oldPkg = await readJson('./package.json')
//   const newPkg = await readJson('./package/package.json')
//   oldPkg.exports = Object.keys(newPkg.exports).reduce((acc, key) => {
//     const newPath = path.join('./package', newPkg.exports[key])
//     acc[key] = `./${newPath}`
//     return acc
//   }, {})
//   await fs.writeFile('./package.json', JSON.stringify(oldPkg, null, 2))
// }

async function build() {
  await exec(`rimraf ${OUT_DIR} && mkdir ${OUT_DIR}`)
  const files = await fg(path.resolve('./protos/*.proto'), {
    absolute: false,
    stats: true,
  })
  const protos = await Promise.all(
    files.map(f =>
      exec(
        `protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_opt=exportCommonSymbols=false --ts_proto_out=./${OUT_DIR} ${path.relative(
          '.',
          f.path
        )}`
      )
    )
  )
  await exec(`mv ./${OUT_DIR}/protos/*.ts ./${OUT_DIR}/ && rimraf ./${OUT_DIR}/protos`)
  await fs.writeFile(
    `./${OUT_DIR}/index.ts`,
    `${files.map(f => `export * from './${f.name.split('.proto')[0]}'`).join('\n')}`,
    'utf-8'
  )
  // await cleanCurrentExports()
  // await Promise.all([
  //   exec('yarn build:pkg'),
  //   exec('yarn build:dist'),
  // ])
  // await writePackageExports()
  // await exec('rm ./package/.npmignore && rm ./package/package.json')
  // await checkFilesBuilt()
}

async function checkFilesBuilt() {
  await Promise.all([
    fs.access('./dist/index.js'),
    fs.access('./dist/index.es.js'),
    fs.access('./dist/index.css'),
    fs.access('./dist/types.d.ts'),
    fs.access('./package/index.js'),
  ])
}

if (!arg) {
  build()
} else {
  throw Error(`Unknown command '${arg}' for build.ts, available commands: <none> | check`)
}
