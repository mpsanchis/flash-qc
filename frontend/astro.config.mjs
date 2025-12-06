// @ts-check
import { defineConfig } from "astro/config";

import react from "@astrojs/react";
import tailwindcss from "@tailwindcss/vite";

const BACKEND_BASE_URL =
  import.meta.env.MODE === "production"
    ? "http://backend:8000"
    : "http://localhost:8000";

// https://astro.build/config
export default defineConfig({
  integrations: [react()],
  server: {
    port: 3000,
    host: true,
  },
  vite: {
    // @ts-expect-error - Type mismatch due to duplicate Vite installations with different
    // optional dependencies (jiti, lightningcss). Both @tailwindcss/vite and Astro depend on
    // Vite 6.4.1, but pnpm installs them separately due to differing peer dependency contexts.
    // This causes TypeScript to see two incompatible Plugin types. Safe to ignore as both
    // resolve to the same Vite version at runtime.
    plugins: [tailwindcss()],
    server: {
      allowedHosts: [
        'flash-qc.com',
        'www.flash-qc.com',
        'localhost',
      ],
      proxy: {
        "/api": {
          target: BACKEND_BASE_URL,
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/api/, ""),
        },
      },
    },
  },
});
