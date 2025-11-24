---
layout: default
title: "notes"
---
<div style="margin: 40px 0;"></div>
[< volver](/)
<div style="margin: 40px 0;"></div>

<style>
  .fire-container {
    text-align: center;
    margin: 40px 0;
    font-family: monospace;
  }
  
  .fire {
    font-size: 20px;
    line-height: 1.2;
    color: #ff1493;
    text-shadow: 0 0 10px #ff1493;
    animation: flicker 1.5s infinite alternate;
  }
  
  .fire-line-1 { animation-delay: 0s; }
  .fire-line-2 { animation-delay: 0.3s; }
  .fire-line-3 { animation-delay: 0.6s; }
  
  @keyframes flicker {
    0%, 100% {
      color: #ff1493;
      text-shadow: 
        0 0 5px #ff1493,
        0 0 10px #ff1493,
        0 0 20px #ff1493,
        0 0 40px #c77dff;
      opacity: 1;
    }
    25% {
      color: #c77dff;
      text-shadow: 
        0 0 10px #c77dff,
        0 0 20px #c77dff,
        0 0 30px #9d4edd;
      opacity: 0.8;
    }
    50% {
      color: #ff1493;
      text-shadow: 
        0 0 15px #ff1493,
        0 0 25px #ff1493,
        0 0 35px #c77dff;
      opacity: 1;
    }
    75% {
      color: #e0aaff;
      text-shadow: 
        0 0 8px #e0aaff,
        0 0 15px #c77dff,
        0 0 25px #9d4edd;
      opacity: 0.9;
    }
  }
  
  .notes-box {
    border: 2px solid #ff1493;
    border-radius: 8px;
    padding: 20px;
    background: rgba(255, 20, 147, 0.05);
    box-shadow: 0 0 20px rgba(255, 20, 147, 0.3);
    margin: 20px 0;
  }
  
  .notes-title {
    color: #ff1493;
    text-shadow: 0 0 10px #ff1493;
    font-size: 1.5em;
    margin-bottom: 20px;
  }
</style>

<div class="fire-container">
  <pre class="fire fire-line-1">    ▲ ▲    </pre>
  <pre class="fire fire-line-2">   ▲ ▲ ▲   </pre>
  <pre class="fire fire-line-3">  ▲ ▲ ▲ ▲  </pre>
  <div style="margin-top: 10px;">
    <pre style="color: #ff1493; text-shadow: 0 0 15px #ff1493; font-size: 18px;">╔═══════════╗
║   NOTES   ║
╚═══════════╝</pre>
  </div>
</div>

<div class="notes-box">
  <h2 class="notes-title">→ guestbook</h2>
  
  <p style="margin-bottom: 20px;">
    Este es un espacio abierto para dejar mensajes, preguntas, pensamientos, o simplemente decir hola.
  </p>

  <h3 style="color: #c77dff; font-size: 14px; text-transform: uppercase; margin-top: 30px; margin-bottom: 10px;">
    → cómo funciona
  </h3>
  
  <p style="font-size: 13px; line-height: 1.6;">
    Los comentarios funcionan via <strong>GitHub Issues</strong> usando <a href="https://utteranc.es" style="color: #ff1493;">Utterances</a>:
  </p>
  
  <ul style="font-size: 13px; margin-left: 20px; line-height: 1.8;">
    <li>Necesitas cuenta GitHub para comentar (filtro natural contra spam)</li>
    <li>Tu mensaje se guarda como un issue público en mi repo</li>
    <li>Aparece instantáneamente aquí</li>
    <li>Yo puedo moderar/eliminar mensajes inapropiados</li>
    <li>Tu username GitHub queda vinculado al comentario</li>
  </ul>
  
  <p style="font-size: 13px; margin-top: 15px; font-style: italic; color: #c77dff;">
    TL;DR: Es seguro, moderado, y requiere ser parte de la comunidad GitHub.
  </p>
</div>

<div style="margin: 40px 0; padding: 20px; border: 2px solid #ff1493; border-radius: 8px; background: rgba(255, 20, 147, 0.05);">
  <h3 style="color: #ff1493; text-shadow: 0 0 10px #ff1493; margin-bottom: 20px;">
    → deja tu mensaje
  </h3>
  
  <p style="font-style: italic; margin-bottom: 20px; font-size: 14px;">
    Sé respetuosx, constructivx, o simplemente auténticx.
  </p>
  
  <!-- Utterances comments -->
  <script src="https://utteranc.es/client.js"
          repo="medusahra/medusahra.github.io"
          issue-term="title"
          label="guestbook"
          theme="photon-dark"
          crossorigin="anonymous"
          async>
  </script>
</div>

<div style="text-align: center; margin-top: 40px; font-size: 12px; opacity: 0.7;">
  <p>
    <em>Contacto directo:</em> 
    <a href="mailto:blackobjkt@protonmail.com" style="color: #ff1493;">email</a> · 
    <a href="https://x.com/medusahra" style="color: #ff1493;">x/twitter</a>
  </p>
</div>
<div style="margin: 40px 0;"></div>
[< volver](/)
<div style="margin: 40px 0;"></div>
