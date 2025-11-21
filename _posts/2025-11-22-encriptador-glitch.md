---
layout: default
title: "Encriptador Glitch 🔐"
date: 2025-11-22
---

<div style="max-width: 700px; margin: 0 auto;">

<h1 style="text-align: center;">🔐 Encriptador Glitch 🔐</h1>

<p style="text-align: center; margin-bottom: 40px;">
<em>cifra tus mensajes en el vacío digital</em>
</p>

<div style="background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 20px; margin: 20px 0;">
  <label style="display: block; margin-bottom: 10px; color: #ff1493; font-weight: bold;">mensaje original:</label>
  <textarea id="input-text" rows="4" style="width: 100%; background: #000; color: #ff2777; border: 1px solid #ff1493; padding: 10px; font-family: monospace; font-size: 14px; border-radius: 4px;"></textarea>
</div>

<div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin: 20px 0;">
  <button onclick="applyRot13()" class="cipher-btn">ROT13</button>
  <button onclick="applyReverse()" class="cipher-btn">Reversa</button>
  <button onclick="applyGlitch()" class="cipher-btn">Glitch</button>
  <button onclick="applyCaesar()" class="cipher-btn">Caesar</button>
</div>

<div id="caesar-controls" style="display: none; margin: 20px 0; background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 15px;">
  <label style="color: #ff1493; font-weight: bold;">Desplazamiento:</label>
  <input type="number" id="caesar-shift" value="3" min="1" max="25" style="width: 60px; background: #000; color: #ff2777; border: 1px solid #ff1493; padding: 5px; margin: 0 10px; font-family: monospace;">
  <button onclick="encryptCaesar()" class="cipher-btn" style="margin: 0 5px;">Encriptar</button>
  <button onclick="decryptCaesar()" class="cipher-btn">Desencriptar</button>
</div>

<div style="background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 20px; margin: 20px 0;">
  <label style="display: block; margin-bottom: 10px; color: #ff1493; font-weight: bold;">resultado:</label>
  <textarea id="output-text" rows="4" readonly style="width: 100%; background: #000; color: #00ff00; border: 1px solid #ff1493; padding: 10px; font-family: monospace; font-size: 14px; border-radius: 4px;"></textarea>
</div>

<div style="text-align: center; margin: 30px 0;">
  <button onclick="copyToClipboard()" style="background: #ff1493; color: #000; border: none; padding: 12px 30px; font-family: monospace; font-weight: bold; cursor: pointer; border-radius: 4px; font-size: 16px;">
    📋 Copiar
  </button>
</div>

</div>

<style>
.cipher-btn {
  background: rgba(255, 20, 147, 0.2);
  border: 2px solid #ff1493;
  color: #ff1493;
  padding: 12px;
  font-family: monospace;
  font-weight: bold;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.3s;
  text-shadow: 0 0 5px #ff1493;
}

.cipher-btn:hover {
  background: rgba(255, 20, 147, 0.4);
  box-shadow: 0 0 15px rgba(255, 20, 147, 0.6);
  transform: translateY(-2px);
}

.cipher-btn:active {
  transform: translateY(0);
}
</style>

<script type="module">
  import init, { 
    caesar_encrypt, 
    caesar_decrypt, 
    rot13, 
    reverse_text, 
    glitch_text 
  } from '/assets/encriptador_glitch.js';

  let wasmModule;

  async function loadWasm() {
    wasmModule = await init('/assets/encriptador_glitch_bg.wasm');
    console.log('WASM loaded ✓');
  }

  loadWasm();

  window.applyRot13 = function() {
    const input = document.getElementById('input-text').value;
    const output = rot13(input);
    document.getElementById('output-text').value = output;
    document.getElementById('caesar-controls').style.display = 'none';
  };

  window.applyReverse = function() {
    const input = document.getElementById('input-text').value;
    const output = reverse_text(input);
    document.getElementById('output-text').value = output;
    document.getElementById('caesar-controls').style.display = 'none';
  };

  window.applyGlitch = function() {
    const input = document.getElementById('input-text').value;
    const output = glitch_text(input);
    document.getElementById('output-text').value = output;
    document.getElementById('caesar-controls').style.display = 'none';
  };

  window.applyCaesar = function() {
    document.getElementById('caesar-controls').style.display = 'block';
  };

  window.encryptCaesar = function() {
    const input = document.getElementById('input-text').value;
    const shift = parseInt(document.getElementById('caesar-shift').value);
    const output = caesar_encrypt(input, shift);
    document.getElementById('output-text').value = output;
  };

  window.decryptCaesar = function() {
    const input = document.getElementById('input-text').value;
    const shift = parseInt(document.getElementById('caesar-shift').value);
    const output = caesar_decrypt(input, shift);
    document.getElementById('output-text').value = output;
  };

  window.copyToClipboard = function() {
    const output = document.getElementById('output-text');
    output.select();
    document.execCommand('copy');
    alert('¡Copiado! 🔐');
  };
</script>
