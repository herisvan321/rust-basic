import React, { useState } from 'react';
import { Link } from '@inertiajs/react';

export default function Welcome({ title, auth_installed }) {
  const [count, setCount] = useState(0);

  return (
    <div className="min-h-screen bg-[#070b13] text-gray-100 flex flex-col justify-between relative overflow-hidden" style={{ fontFamily: "'Inter', sans-serif" }}>
      
      {/* Background Neon Glowing Orbs */}
      <div 
        className="absolute top-[-10%] left-[-10%] w-[500px] h-[500px] rounded-full pointer-events-none" 
        style={{ background: "rgba(99, 102, 241, 0.08)", filter: "blur(120px)" }} 
      />
      <div 
        className="absolute bottom-[-10%] right-[-10%] w-[500px] h-[500px] rounded-full pointer-events-none" 
        style={{ background: "rgba(6, 182, 212, 0.08)", filter: "blur(120px)" }} 
      />

      {/* Header/Navbar */}
      <header className="border-b border-white/5 sticky top-0 z-50" style={{ background: "rgba(7, 11, 19, 0.6)", backdropFilter: "blur(12px)", WebkitBackdropFilter: "blur(12px)" }}>
        <div className="max-w-7xl mx-auto px-6 h-20 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-indigo-500 via-purple-500 to-cyan-500 flex items-center justify-center font-bold text-white shadow-[0_0_20px_rgba(99,102,241,0.5)]">
              RB
            </div>
            <span className="font-extrabold text-xl tracking-tight text-white">
              Rust<span className="text-indigo-400">Basic</span>
            </span>
          </div>
          
          <nav className="hidden md:flex items-center gap-8">
            <Link 
              href="/" 
              className="text-sm font-semibold text-indigo-400 hover:text-white transition-colors duration-200"
              style={{ textDecoration: 'none' }}
            >
              Beranda
            </Link>
            <Link 
              href="/about" 
              className="text-sm font-semibold text-gray-400 hover:text-white transition-colors duration-200"
              style={{ textDecoration: 'none' }}
            >
              Tentang Aplikasi
            </Link>
            <a 
              href="/dev" 
              className="text-sm font-semibold text-gray-400 hover:text-white transition-colors duration-200"
              style={{ textDecoration: 'none' }}
            >
              API Dev Info
            </a>
          </nav>

          <div className="flex items-center gap-4">
            <span className="inline-flex items-center gap-1.5 px-3 h-8 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
              <span className="w-2 h-2 rounded-full bg-emerald-400" style={{ boxShadow: "0 0 10px #34d399" }} />
              Backend Online
            </span>
          </div>
        </div>
      </header>

      {/* Main Hero & Content */}
      <main className="max-w-7xl mx-auto px-6 py-16 flex-grow flex flex-col justify-center relative z-10">
        <div className="flex flex-col lg:flex-row gap-16 items-center">
          
          {/* Left Hero Texts */}
          <div className="flex-1 flex flex-col gap-6 text-left">
            <div className="inline-flex max-w-fit px-4 h-8 rounded-full text-indigo-300 text-xs font-bold items-center border border-indigo-500/25" style={{ background: "linear-gradient(to right, rgba(99, 102, 241, 0.1), rgba(6, 182, 212, 0.1))" }}>
              ✨ Modern Rust SPA Architecture
            </div>
            
            <h1 className="text-4xl md:text-6xl font-black tracking-tight leading-tight text-white m-0">
              Satu Langkah Lebih Cepat Dengan <br />
              <span className="bg-clip-text text-transparent bg-gradient-to-r from-indigo-400 via-purple-400 to-cyan-400">
                React + Inertia.js
              </span>
            </h1>

            <p className="text-gray-400 text-lg leading-relaxed max-w-xl m-0">
              Teknologi backend berkinerja tinggi dari <b>Rust (Axum)</b> bersatu dengan reaktivitas stateful dari <b>React</b> via jembatan elegan <b>Inertia.js</b>. Ucapkan selamat tinggal pada pemisahan API repo & reload halaman penuh!
            </p>

            {/* CTA Buttons */}
            <div className="flex flex-wrap gap-4 mt-4">
              <Link
                href="/about"
                className="px-8 h-14 rounded-2xl bg-indigo-600 hover:bg-indigo-500 font-bold text-white flex items-center justify-center transition-all duration-300"
                style={{ 
                  textDecoration: 'none',
                  boxShadow: '0 4px 20px rgba(99, 102, 241, 0.4)'
                }}
              >
                Jelajahi Navigasi SPA
              </Link>
              <a
                href="/dev"
                className="px-8 h-14 rounded-2xl bg-white/5 hover:bg-white/10 font-bold text-white flex items-center justify-center border border-white/10 hover:border-white/20 transition-all duration-300"
                style={{ textDecoration: 'none' }}
              >
                Cek JSON Config
              </a>
            </div>

            {/* Tech Badges */}
            <div className="flex items-center gap-4 mt-8 text-xs text-gray-500 font-medium">
              <span>POWERED BY:</span>
              <span className="px-2.5 py-1 rounded bg-white/5 text-gray-300 border border-white/5">Rust Axum</span>
              <span className="px-2.5 py-1 rounded bg-white/5 text-gray-300 border border-white/5">React.js</span>
              <span className="px-2.5 py-1 rounded bg-white/5 text-gray-300 border border-white/5">Vite</span>
              <span className="px-2.5 py-1 rounded bg-white/5 text-gray-300 border border-white/5">Inertia.js</span>
            </div>
          </div>

          {/* Right Interactive Widgets */}
          <div className="w-full lg:w-96 flex flex-col gap-6">
            
            {/* Interactive Card: React Counter widget */}
            <div className="p-8 rounded-3xl border border-white/10 relative overflow-hidden" style={{ background: "rgba(255, 255, 255, 0.02)", backdropFilter: "blur(20px)" }}>
              <div 
                className="absolute top-0 right-0 w-32 h-32 rounded-full pointer-events-none" 
                style={{ background: "rgba(99, 102, 241, 0.08)", filter: "blur(20px)" }} 
              />
              
              <div className="flex items-center justify-between mb-6">
                <span className="text-xs font-semibold text-indigo-400 tracking-wider uppercase">Reaktifitas React</span>
                <span className="w-2.5 h-2.5 rounded-full bg-indigo-400 animate-ping" />
              </div>
              
              <h3 className="text-xl font-bold text-white mb-2 mt-0">Uji State Counter</h3>
              <p className="text-sm text-gray-400 mb-6 leading-relaxed">
                Klik tombol di bawah untuk melihat performa reaktivitas client-side React tanpa melibatkan server round-trip.
              </p>
              
              <div className="flex items-center gap-6">
                <button
                  onClick={() => setCount(c => c + 1)}
                  className="px-6 h-12 rounded-xl bg-indigo-500 hover:bg-indigo-400 font-bold text-white flex items-center justify-center border-0 cursor-pointer transition-all duration-200 active:scale-95 shadow-md"
                >
                  Tambah Angka: {count}
                </button>
                <button
                  onClick={() => setCount(0)}
                  className="text-sm font-semibold text-gray-500 hover:text-gray-300 bg-transparent border-0 cursor-pointer transition-colors"
                >
                  Reset
                </button>
              </div>
            </div>

            {/* Info Card: Monolithic Connection widget */}
            <div className="p-8 rounded-3xl border border-indigo-500/20 shadow-2xl relative overflow-hidden" style={{ background: "linear-gradient(to bottom right, rgba(99, 102, 241, 0.05), rgba(6, 182, 212, 0.05))" }}>
              <h3 className="text-lg font-bold text-white mb-3 mt-0">Integrasi 100% Terhubung</h3>
              <p className="text-sm text-gray-400 leading-relaxed m-0">
                Backend Rust dan Frontend React Anda <b>tidak terpisah</b> dalam dua service. Mereka saling terikat erat (monolith) namun memiliki pemisahan modular yang rapi. Cepat dikembangkan, luar biasa kencang saat di-serve!
              </p>
              <div className="mt-4 flex items-center gap-2 text-xs font-semibold text-cyan-400">
                <span>Status Scaffold Auth:</span>
                <span className="px-2 py-0.5 rounded" style={{ 
                  background: auth_installed ? "rgba(16, 185, 129, 0.1)" : "rgba(245, 158, 11, 0.1)",
                  color: auth_installed ? "#34d399" : "#fbbf24",
                  border: auth_installed ? "1px solid rgba(16, 185, 129, 0.2)" : "1px solid rgba(245, 158, 11, 0.2)"
                }}>
                  {auth_installed ? 'Installed' : 'Belum Terinstal'}
                </span>
              </div>
            </div>

          </div>

        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-white/5 py-8 text-center text-sm text-gray-600 bg-[#070b13]/40">
        <div className="max-w-7xl mx-auto px-6 flex flex-col md:flex-row items-center justify-between gap-4">
          <p className="m-0">© {new Date().getFullYear()} RustBasic Framework. Made with ❤️ for developers.</p>
          <div className="flex gap-6 items-center">
            <a href="https://github.com/herisvan321/rustbasic" className="hover:text-gray-400 transition-colors" style={{ textDecoration: 'none', color: 'inherit' }}>GitHub</a>
            <span className="text-white/5">|</span>
            <span>Versi SPA 1.0.0</span>
          </div>
        </div>
      </footer>

    </div>
  );
}
