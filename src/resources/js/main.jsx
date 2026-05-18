import React from 'react';
import { createRoot } from 'react-dom/client';
import { createInertiaApp } from '@inertiajs/react';
import axios from 'axios';

// Konfigurasi Axios secara global
window.axios = axios;
window.axios.defaults.headers.common['X-Requested-With'] = 'XMLHttpRequest';

// Mengambil CSRF token dari meta tag dan menyematkannya ke header default Axios
const csrfToken = document.querySelector('meta[name="csrf-token"]')?.getAttribute('content');
if (csrfToken) {
  window.axios.defaults.headers.common['X-CSRF-TOKEN'] = csrfToken;
} else {
  console.warn('CSRF token meta tag tidak ditemukan! Pastikan tag <meta name="csrf-token"> tersedia.');
}


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
