<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Generador de Poemas | medusahra</title>
  <style>
    /* (todo tu CSS original aquí - sin cambios) */
  </style>
</head>
<body>
  <div class="container">
    <a href="/" class="back-btn">← volver al home</a>
    
    <div class="header">
      <h1>✍️ Generador de Poemas ✍️</h1>
      <p class="subtitle">poemas cuasi-ensayísticos vagamente lujuriosos</p>
    </div>
    
    <div class="input-section">
      <label for="theme-input">Tema o inspiración:</label>
      <input type="text" id="theme-input" placeholder="ej: amor cyberpunk, soledad digital...">
      
      <label for="details-input">Detalles adicionales (opcional):</label>
      <textarea id="details-input" placeholder="Contexto, palabras clave, atmósfera..."></textarea>
      
      <button class="generate-btn" id="generate-btn" onclick="generatePoem()">
        Generar Poema
      </button>
    </div>
    
    <div id="loading" class="loading" style="display: none;">⏳ Generando tu poema...</div>
    <div id="error" class="error" style="display: none;"></div>
    
    <div class="output-section" id="output-section">
      <label>Tu poema:</label>
      <div id="poem-output"></div>
      <button class="copy-btn" onclick="copyPoem()">📋 Copiar Poema</button>
    </div>
  </div>
  
  <script>
    // 🔑 PASO 1: Coloca tu API Key de Anthropic aquí
    // Obtén una gratuita en: https://console.anthropic.com/
    const ANTHROPIC_API_KEY = 'sk-ant-api03-TU_CLAVE_AQUI';
    
    // 🎯 PASO 2: Opción para pruebas sin API (modo demo)
    const USE_MOCK_MODE = false; // Cambia a true para probar sin API
    
    // Datos de ejemplo para modo demo
    const MOCK_POEMS = [
      `en la madrugada digital
los cuerpos se deshacen en píxeles
y el amor es un archivo corrupto
que jamás abriremos

ceniceros llenos de nostalgía
versos subrayados en amarillo
la luna se ha vuelto neon
y sangra luz fria

así, entre tabuladores y silencio,
te escribo una elegía
que el tiempo borrará
como un disco dañado`,
      
      `fragmentos de tu piel
en cada ventana del metro
el humo de un cigarro
que jamás encendiste

somos infraestructura del deseo
cableados de melancolía
solitarios en esta red
que nos separa más que une`
    ];
    
    async function generatePoem() {
      const theme = document.getElementById('theme-input').value.trim();
      const details = document.getElementById('details-input').value.trim();
      
      if (!theme) {
        showError('⚠️ Por favor ingresa un tema');
        return;
      }
      
      const btn = document.getElementById('generate-btn');
      const loading = document.getElementById('loading');
      const error = document.getElementById('error');
      const output = document.getElementById('output-section');
      
      btn.disabled = true;
      loading.style.display = 'block';
      error.style.display = 'none';
      output.classList.remove('visible');
      
      try {
        let poem;
        
        if (USE_MOCK_MODE) {
          // 🎭 Modo demo: usa un poema aleatorio
          await new Promise(r => setTimeout(r, 1500)); // Simula delay
          poem = MOCK_POEMS[Math.floor(Math.random() * MOCK_POEMS.length)];
        } else {
          // 🔌 Modo real: llama a la API
          if (!ANTHROPIC_API_KEY || ANTHROPIC_API_KEY.includes('TU_CLAVE_AQUI')) {
            throw new Error('API Key no configurada. Edita ANTHROPIC_API_KEY en el script');
          }
          
          const systemPrompt = `Eres un asistente que escribe poemas en el estilo de Giovanna Chadid (medusahra)...`;
          
          const userPrompt = details 
            ? `Escribe un poema sobre: ${theme}\n\nDetalles: ${details}`
            : `Escribe un poema sobre: ${theme}`;
          
          // ⚠️ IMPORTANTE: La URL correcta sin espacio al final
          const response = await fetch("https://api.anthropic.com/v1/messages", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
              "x-api-key": ANTHROPIC_API_KEY, // 🔑 Aquí va la API Key
              "anthropic-version": "2023-06-01"
            },
            body: JSON.stringify({
              model: "claude-3-sonnet-20240229", // ✅ Modelo CORRECTO
              max_tokens: 1000,
              system: systemPrompt,
              messages: [{ role: "user", content: userPrompt }]
            })
          });
          
          if (!response.ok) {
            const errorData = await response.json().catch(() => ({}));
            throw new Error(`Error ${response.status}: ${errorData.error?.message || response.statusText}`);
          }
          
          const data = await response.json();
          poem = data.content?.[0]?.text || "No se pudo generar el poema.";
        }
        
        document.getElementById('poem-output').textContent = poem;
        output.classList.add('visible');
        
      } catch (err) {
        console.error(err);
        showError(`❌ ${err.message}`);
      } finally {
        btn.disabled = false;
        loading.style.display = 'none';
      }
    }
    
    function showError(message) {
      const error = document.getElementById('error');
      error.textContent = message;
      error.style.display = 'block';
    }
    
    function copyPoem() {
      const poem = document.getElementById('poem-output').textContent;
      navigator.clipboard.writeText(poem).then(() => {
        alert('¡Poema copiado! 📋');
      }).catch(err => {
        alert('Error al copiar: ' + err);
      });
    }
    
    // Enter para generar
    document.getElementById('theme-input').addEventListener('keypress', (e) => {
      if (e.key === 'Enter') generatePoem();
    });
  </script>
</body>
</html>
