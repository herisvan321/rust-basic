{% extends "layouts/app.rsx" %}

{% block title %}Test RSX - RustBasic{% endblock %}

{% block content %}
<div class="container" style="padding: 5rem; max-width: 800px; margin: 0 auto;">
    <h1 style="font-weight: 800; margin-bottom: 2rem;">🚀 RSX Syntax Test</h1>
    
    <div style="background: #fff; padding: 2rem; border-radius: 1rem; box-shadow: 0 4px 20px rgba(0,0,0,0.05); margin-bottom: 2rem;">
        <h2 style="font-size: 1.25rem; margin-bottom: 1rem;">1. Self-Closing Tag Test</h2>
        <p class="text-muted mb-4">Sintaks: <code>&lt;Buttons.Button label="Self Closing" /&gt;</code></p>
        <Buttons.Button label="Self Closing Button" variant="success" />
    </div>

    <div style="background: #fff; padding: 2rem; border-radius: 1rem; box-shadow: 0 4px 20px rgba(0,0,0,0.05);">
        <h2 style="font-size: 1.25rem; margin-bottom: 1rem;">2. Block Tag Test (with Call)</h2>
        <p class="text-muted mb-4">Sintaks: <code>&lt;Display.Card title="..."&gt;...&lt;/Display.Card&gt;</code></p>
        
        <Display.Card title="RSX Card Premium">
            <p style="margin-bottom: 1.5rem;">Konten ini berada di dalam komponen <strong>Display.Card</strong> yang dipanggil menggunakan sintaks RSX block.</p>
            
            <h3 style="font-size: 1rem; font-weight: 700; margin-bottom: 1rem;">Input di dalam Card:</h3>
            <Forms.Input name="test_rsx" label="Nama RSX" placeholder="Ketik sesuatu..." />
        </Display.Card>
    </div>

    <div style="margin-top: 3rem; text-align: center;">
        <a href="/" class="text-primary" style="font-weight: 700;">&larr; Kembali ke Beranda</a>
    </div>
</div>
{% endblock %}
