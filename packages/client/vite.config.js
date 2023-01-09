import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte({})],
  server: {
    port: parseInt(process.env.PORT || 7636),
    strictPort: true,
  },
})
