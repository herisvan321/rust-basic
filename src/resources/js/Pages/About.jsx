import React from 'react';
import { Link } from '@inertiajs/react';

export default function About({ title, description, version, backend, frontend, bridge }) {
  return (
    <div className="min-h-screen bg-[#070b13] text-gray-100 flex flex-col justify-between relative overflow-hidden" style={{ fontFamily: "'Inter', sans-serif" }}>
      
      {/* Background Neon Glowing Orbs */}
      <div 
        className="absolute top-[-10%] right-[-10%] w-[500px] h-[500px] rounded-full pointer-events-none" 
        style={{ background: "rgba(168, 85, 247, 0.08)", filter: "blur(120px)" }} 
      />
      <div 
        className="absolute bottom-[-10%] left-[-10%] w-[500px] h-[500px] rounded-full pointer-events-none" 
        style={{ background: "rgba(99, 102, 241, 0.08)", filter: "blur(120px)" }} 
      />

      {/* Header/Navbar */}
      <header className="border-b border-white/5 sticky top-0 z-50" style={{ background: "rgba(7, 11, 19, 0.6)", backdropFilter: "blur(12px)", WebkitBackdropFilter: "blur(12px)" }}>
        <div className="max-w-7xl mx-auto px-6 h-20 flex items-center justify-between">
          <Link href="/" className="flex items-center gap-3" style={{ textDecoration: 'none' }}>
            <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-indigo-500 via-purple-500 to-cyan-500 flex items-center justify-center font-bold text-white shadow-[0_0_20px_rgba(99,102,241,0.5)]">
              RB
            </div>
            <span className="font-extrabold text-xl tracking-tight text-white">
              Rust<span className="text-indigo-400">Basic</span>
            </span>
          </Link>
          
          <nav className="hidden md:flex items-center gap-8">
            <Link 
              href="/" 
              className="text-sm font-semibold text-gray-400 hover:text-white transition-colors duration-200"
              style={{ textDecoration: 'none' }}
            >
              Beranda
            </Link>
            <Link 
              href="/about" 
              className="text-sm font-semibold text-indigo-400 hover:text-white transition-colors duration-200"
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
            <Link
              href="/"
              className="px-4 h-9 rounded-xl bg-white/5 hover:bg-white/10 text-xs font-bold text-white flex items-center border border-white/10 hover:border-white/20 transition-all duration-200"
              style={{ textDecoration: 'none' }}
            >
              Kembali Ke Beranda
            </Link>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-4xl mx-auto px-6 py-16 flex-grow flex flex-col justify-center relative z-10 w-full">
        <div className="flex flex-col gap-8 text-left">
          
          {/* Badge */}
          <div className="flex justify-start">
            <div className="inline-flex px-4 h-8 rounded-full text-indigo-300 text-xs font-bold items-center border border-indigo-500/25" style={{ background: "linear-gradient(to right, rgba(99, 102, 241, 0.1), rgba(168, 85, 247, 0.1))" }}>
              ⚙️ Framework Architecture Details
            </div>
          </div>

          {/* Heading */}
          <div className="flex flex-col gap-4">
            <h1 className="text-4xl md:text-5xl font-black tracking-tight text-white leading-none m-0">
              {title}
            </h1>
            <p className="text-gray-400 text-lg leading-relaxed m-0">
              {description}
            </p>
          </div>

          {/* Tech Spec Grid */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-4">
            
            {/* Spec Card: Backend */}
            <div className="p-6 rounded-2xl border border-white/5 flex flex-col gap-2" style={{ background: "rgba(255, 255, 255, 0.01)" }}>
              <span className="text-xs font-semibold text-indigo-400 uppercase tracking-wider">Backend Platform</span>
              <span className="text-xl font-bold text-white">{backend}</span>
              <span className="text-xs text-gray-500">Super cepat, aman, & hemat memori.</span>
            </div>

            {/* Spec Card: Frontend */}
            <div className="p-6 rounded-2xl border border-white/5 flex flex-col gap-2" style={{ background: "rgba(255, 255, 255, 0.01)" }}>
              <span className="text-xs font-semibold text-purple-400 uppercase tracking-wider">Frontend Engine</span>
              <span className="text-xl font-bold text-white">{frontend}</span>
              <span className="text-xs text-gray-500">Kompilasi super cepat dengan HMR Vite.</span>
            </div>

            {/* Spec Card: Bridge */}
            <div className="p-6 rounded-2xl border border-white/5 flex flex-col gap-2" style={{ background: "rgba(255, 255, 255, 0.01)" }}>
              <span className="text-xs font-semibold text-cyan-400 uppercase tracking-wider">Communication Bridge</span>
              <span className="text-xl font-bold text-white">{bridge}</span>
              <span className="text-xs text-gray-500">Kirim props tanpa API routing manual.</span>
            </div>

          </div>

          {/* Flow Diagram (CSS styled) */}
          <div className="mt-6 p-8 rounded-3xl border border-white/10 relative overflow-hidden flex flex-col gap-4" style={{ background: "rgba(255, 255, 255, 0.02)" }}>
            <h3 className="text-lg font-bold text-white m-0">Bagaimana SPA Ini Bekerja?</h3>
            <div className="flex flex-col md:flex-row items-center justify-between gap-6 text-sm text-gray-400 mt-2">
              <div className="flex-1 text-center p-4 rounded-xl border border-white/5 bg-white/5">
                <span className="font-bold text-white block mb-1">1. User Click Link</span>
                Inertia menangkap klik <code>&lt;Link&gt;</code> secara lokal tanpa reload browser.
              </div>
              <div className="text-indigo-400 text-xl font-bold rotate-90 md:rotate-0">➔</div>
              <div className="flex-1 text-center p-4 rounded-xl border border-white/5 bg-white/5">
                <span className="font-bold text-white block mb-1">2. Axios X-Inertia Request</span>
                Client mengirim request AJAX dengan header <code>X-Inertia: true</code>.
              </div>
              <div className="text-indigo-400 text-xl font-bold rotate-90 md:rotate-0">➔</div>
              <div className="flex-1 text-center p-4 rounded-xl border border-white/5 bg-white/5">
                <span className="font-bold text-white block mb-1">3. Rust Inertia JSON</span>
                Backend Axum langsung membalas dengan JSON berisi props & component name.
              </div>
            </div>
          </div>

          {/* Back button */}
          <div className="mt-4 flex justify-start">
            <Link
              href="/"
              className="inline-flex px-6 h-12 rounded-xl bg-white/5 hover:bg-white/10 font-bold text-white items-center justify-center border border-white/10 hover:border-white/20 transition-all duration-300"
              style={{ textDecoration: 'none' }}
            >
              ← Kembali ke Beranda
            </Link>
          </div>

        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-white/5 py-8 text-center text-sm text-gray-600 bg-[#070b13]/40">
        <p className="m-0">© {new Date().getFullYear()} RustBasic Framework. Halaman SPA Versi {version}.</p>
      </footer>

    </div>
  );
}
