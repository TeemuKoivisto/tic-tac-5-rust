import typescript from 'rollup-plugin-typescript2'

import pkg from './package.json'

export default {
  input: 'out-ts/index.ts',
  output: [
    {
      file: pkg.main,
      format: 'cjs',
    },
    {
      file: pkg.module,
      format: 'es',
    },
  ],
  external: [...Object.keys(pkg.dependencies || {}), ...Object.keys(pkg.devDependencies || {})],
  plugins: [typescript()],
}
