import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],
    // for some reason this seams to be needed when running a development server,
    // although I don't see an error in the syntastica package.json
    resolve: {
        alias: {
            syntastica: './node_modules/syntastica/pkg/index.js',
        },
    },
})
