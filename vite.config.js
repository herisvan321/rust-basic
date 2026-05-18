import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src/resources/js'),
    },
  },
  build: {
    outDir: 'public/build',
    emptyOutDir: true,
    manifest: true, // Generates manifest.json in the build folder for versioned lookups in production
    rollupOptions: {
      input: 'src/resources/js/main.jsx',
    },
  },
  server: {
    cors: true,
    origin: 'http://localhost:5173',
    hmr: {
      host: 'localhost',
    },
  },
});
