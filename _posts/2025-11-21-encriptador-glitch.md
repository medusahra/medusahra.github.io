---
layout: default
title: "Encriptador Glitch 🔐"
date: 2025-11-21
---

<div style="max-width: 700px; margin: 0 auto;">

<div style="margin-bottom: 20px;">
  <a href="/" style="color: #ff1493; text-decoration: none; font-size: 14px; display: inline-block; padding: 8px 15px; border: 1px solid #ff1493; border-radius: 4px; transition: all 0.3s;">
    ← volver al home
  </a>
</div>

<h1 style="text-align: center;">🔐 Encriptador Glitch 🔐</h1>

<div id="loading-status" style="text-align: center; color: #ff1493; margin: 20px 0; font-weight: bold;">
  ⏳ Cargando encriptador...
</div>

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
  <button onclick="applyXOR()" class="cipher-btn">XOR</button>
  <button onclick="applyBase64()" class="cipher-btn">Base64</button>
</div>

<div id="caesar-controls" style="display: none; margin: 20px 0; background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 15px;">
  <label style="color: #ff1493; font-weight: bold;">Desplazamiento:</label>
  <input type="number" id="caesar-shift" value="3" min="1" max="25" style="width: 60px; background: #000; color: #ff2777; border: 1px solid #ff1493; padding: 5px; margin: 0 10px; font-family: monospace;">
  <button onclick="encryptCaesar()" class="cipher-btn" style="margin: 0 5px;">Encriptar</button>
  <button onclick="decryptCaesar()" class="cipher-btn">Desencriptar</button>
</div>

<div id="xor-controls" style="display: none; margin: 20px 0; background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 15px;">
  <label style="color: #ff1493; font-weight: bold;">Clave secreta:</label>
  <input type="text" id="xor-key" placeholder="tu clave aquí" style="width: 200px; background: #000; color: #ff2777; border: 1px solid #ff1493; padding: 5px; margin: 0 10px; font-family: monospace;">
  <button onclick="encryptXOR()" class="cipher-btn" style="margin: 0 5px;">Encriptar</button>
  <button onclick="decryptXOR()" class="cipher-btn">Desencriptar</button>
</div>

<div id="base64-controls" style="display: none; margin: 20px 0; background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 15px;">
  <button onclick="encodeBase64()" class="cipher-btn" style="margin: 0 5px;">Codificar</button>
  <button onclick="decodeBase64()" class="cipher-btn">Decodificar</button>
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
    glitch_text,
    xor_cipher,
    base64_encode,
    base64_decode
  } from '/assets/encriptador_glitch.js';

  let wasmModule;

  async function loadWasm() {
    try {
      wasmModule = await init('/assets/encriptador_glitch_bg.wasm');
      console.log('WASM loaded ✓');
      document.getElementById('loading-status').innerHTML = '✅ Listo para encriptar';
      document.getElementById('loading-status').style.color = '#00ff00';
    } catch (error) {
      console.error('Error loading WASM:', error);
      document.getElementById('loading-status').innerHTML = '❌ Error al cargar. Recarga la página.';
      document.getElementById('loading-status').style.color = '#ff0000';
    }
  }

  loadWasm();

  function checkWasmLoaded() {
    if (!wasmModule) {
      alert('⚠️ El encriptador aún está cargando. Espera un momento.');
      return false;
    }
    return true;
  }

  window.applyRot13 = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const output = rot13(input);
      document.getElementById('output-text').value = output;
      document.getElementById('caesar-controls').style.display = 'none';
      document.getElementById('xor-controls').style.display = 'none';
      document.getElementById('base64-controls').style.display = 'none';
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.applyReverse = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const output = reverse_text(input);
      document.getElementById('output-text').value = output;
      document.getElementById('caesar-controls').style.display = 'none';
      document.getElementById('xor-controls').style.display = 'none';
      document.getElementById('base64-controls').style.display = 'none';
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.applyGlitch = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const output = glitch_text(input);
      document.getElementById('output-text').value = output;
      document.getElementById('caesar-controls').style.display = 'none';
      document.getElementById('xor-controls').style.display = 'none';
      document.getElementById('base64-controls').style.display = 'none';
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.applyCaesar = function() {
    if (!checkWasmLoaded()) return;
    document.getElementById('caesar-controls').style.display = 'block';
    document.getElementById('xor-controls').style.display = 'none';
    document.getElementById('base64-controls').style.display = 'none';
  };

  window.encryptCaesar = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const shift = parseInt(document.getElementById('caesar-shift').value);
      const output = caesar_encrypt(input, shift);
      document.getElementById('output-text').value = output;
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.decryptCaesar = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const shift = parseInt(document.getElementById('caesar-shift').value);
      const output = caesar_decrypt(input, shift);
      document.getElementById('output-text').value = output;
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.applyXOR = function() {
    if (!checkWasmLoaded()) return;
    document.getElementById('xor-controls').style.display = 'block';
    document.getElementById('caesar-controls').style.display = 'none';
    document.getElementById('base64-controls').style.display = 'none';
  };

  window.encryptXOR = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const key = document.getElementById('xor-key').value;
      if (!key) {
        alert('⚠️ Necesitas ingresar una clave');
        return;
      }
      const output = xor_cipher(input, key);
      document.getElementById('output-text').value = output;
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.decryptXOR = function() {
    encryptXOR();
  };

  window.applyBase64 = function() {
    if (!checkWasmLoaded()) return;
    document.getElementById('base64-controls').style.display = 'block';
    document.getElementById('caesar-controls').style.display = 'none';
    document.getElementById('xor-controls').style.display = 'none';
  };

  window.encodeBase64 = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const output = base64_encode(input);
      document.getElementById('output-text').value = output;
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.decodeBase64 = function() {
    if (!checkWasmLoaded()) return;
    try {
      const input = document.getElementById('input-text').value;
      const output = base64_decode(input);
      document.getElementById('output-text').value = output;
    } catch (error) {
      alert('Error: ' + error.message);
    }
  };

  window.copyToClipboard = function() {
    const output = document.getElementById('output-text');
    output.select();
    document.execCommand('copy');
    alert('¡Copiado! 🔐');
  };
</script>
