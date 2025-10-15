// @ts-check
import { defineConfig } from 'astro/config';

import react from '@astrojs/react';

const BACKEND_BASE_URL = (import.meta.env.MODE === 'production')
  ? 'http://backend:8000'
  : 'http://localhost:8000';

// https://astro.build/config
export default defineConfig({
  integrations: [react()],
  server: {
    port: 3000,
    host: true,
  },
  vite: {
    server: {
      proxy: {
        '/api': {
          target: BACKEND_BASE_URL,
          changeOrigin: true,
          rewrite: path => path.replace(/^\/api/, ''),
        },
      }
    }
  }
});