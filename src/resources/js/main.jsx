import React from 'react';
import { createRoot } from 'react-dom/client';
import { createInertiaApp } from '@inertiajs/react';

createInertiaApp({
  resolve: name => {
    // Membaca seluruh file React di dalam folder Pages secara dinamis menggunakan Vite glob
    const pages = import.meta.glob('./Pages/**/*.jsx', { eager: true });
    const page = pages[`./Pages/${name}.jsx`];
    if (!page) {
      throw new Error(`Halaman Pages/${name}.jsx tidak ditemukan!`);
    }
    return page;
  },
  setup({ el, App, props }) {
    // Mount aplikasi React ke elemen HTML #app
    createRoot(el).render(<App {...props} />);
  },
});
