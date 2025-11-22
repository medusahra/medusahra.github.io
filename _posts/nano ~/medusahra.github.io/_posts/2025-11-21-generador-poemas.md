<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Generador de Poemas | medusahra</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    
    body {
      background-color: #000000;
      color: #ff2777;
      font-family: 'Courier New', monospace;
      padding: 20px;
      min-height: 100vh;
    }
    
    .container {
      max-width: 800px;
      margin: 0 auto;
    }
    
    .header {
      text-align: center;
      margin-bottom: 40px;
    }
    
    h1 {
      color: #ff1493;
      text-shadow: 0 0 10px #ff1493, 0 0 20px #ff1493;
      font-size: 2em;
      margin-bottom: 10px;
    }
    
    .subtitle {
      color: #ffffff;
      text-shadow: 0 0 5px #ffffff;
      font-style: italic;
      font-size: 0.9em;
    }
    
    .back-btn {
      display: inline-block;
      color: #ff1493;
      text-decoration: none;
      font-size: 14px;
      padding: 8px 15px;
      border: 1px solid #ff1493;
      border-radius: 4px;
      margin-bottom: 20px;
      transition: all 0.3s;
    }
    
    .back-btn:hover {
      background: rgba(255, 20, 147, 0.2);
    }
    
    .input-section {
      background: rgba(255, 20, 147, 0.1);
      border: 2px solid #ff1493;
      border-radius: 8px;
      padding: 20px;
      margin-bottom: 20px;
    }
    
    label {
      display: block;
      color: #ff1493;
      font-weight: bold;
      margin-bottom: 10px;
    }
    
    input[type="text"], textarea {
      width: 100%;
      background: #000;
      color: #ff2777;
      border: 1px solid #ff1493;
      padding: 12px;
      font-family: 'Courier New', monospace;
      font-size: 14px;
      border-radius: 4px;
      margin-bottom: 15px;
    }
    
    textarea {
      min-height: 100px;
      resize: vertical;
    }
    
    .generate-btn {
      background: #ff1493;
      color: #000;
      border: none;
      padding: 15px 40px;
      font-family: 'Courier New', monospace;
      font-weight: bold;
      font-size: 16px;
      cursor: pointer;
      border-radius: 4px;
      width: 100%;
      transition: all 0.3s;
      text-shadow: none;
    }
    
    .generate-btn:hover {
      box-shadow: 0 0 20px rgba(255, 20, 147, 0.8);
      transform: translateY(-2px);
    }
    
    .generate-btn:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
    
    .output-section {
      background: rgba(255, 20, 147, 0.1);
      border: 2px solid #ff1493;
      border-radius: 8px;
      padding: 20px;
      margin-top: 20px;
      display: none;
    }
    
    .output-section.visible {
      display: block;
    }
    
    #poem-output {
      color: #00ff00;
      line-height: 1.8;
      white-space: pre-wrap;
      font-size: 15px;
      margin-bottom: 20px;
    }
    
    .copy-btn {
      background: rgba(255, 20, 147, 0.2);
      border: 2px solid #ff1493;
      color: #ff1493;
      padding: 10px 20px;
      font-family: 'Courier New', monospace;
      font-weight: bold;
      cursor: pointer;
      border-radius: 4px;
      transition: all 0.3s;
    }
    
    .copy-btn:hover {
      background: rgba(255, 20, 147, 0.4);
    }
    
    .loading {
      text-align: center;
      color: #ff1493;
      font-style: italic;
      margin: 20px 0;
    }
    
    .error {
      color: #ff0000;
      text-shadow: 0 0 5px #ff0000;
      margin: 20px 0;
    }
    
    @media (max-width: 600px) {
      body {
        padding: 10px;
      }
      
      h1 {
        font-size: 1.5em;
      }
      
      .generate-btn {
        padding: 12px 30px;
      }
    }
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
      <input 
        type="text" 
        id="theme-input" 
        placeholder="ej: amor cyberpunk, soledad digital, cuerpos fragmentados..."
      >
      
      <label for="details-input">Detalles adicionales (opcional):</label>
      <textarea 
        id="details-input" 
        placeholder="Añade contexto, palabras clave, atmósfera que buscas..."
      ></textarea>
      
      <button class="generate-btn" id="generate-btn" onclick="generatePoem()">
        Generar Poema
      </button>
    </div>
    
    <div id="loading" class="loading" style="display: none;">
      ⏳ Generando tu poema...
    </div>
    
    <div id="error" class="error" style="display: none;"></div>
    
    <div class="output-section" id="output-section">
      <label>Tu poema:</label>
      <div id="poem-output"></div>
      <button class="copy-btn" onclick="copyPoem()">📋 Copiar Poema</button>
    </div>
  </div>
  
  <script>
    async function generatePoem() {
      const theme = document.getElementById('theme-input').value.trim();
      const details = document.getElementById('details-input').value.trim();
      
      if (!theme) {
        alert('⚠️ Por favor ingresa un tema');
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
        const systemPrompt = `Eres un asistente que escribe poemas en el estilo de Giovanna Chadid (medusahra). 

Características de su estilo:
- Poemas cuasi-ensayísticos vagamente lujuriosos
- Mezcla de lo filosófico con lo sensorial
- Listas acumulativas que construyen atmósferas densas
- Referencias literarias árabes y occidentales (Qabbani, Darwish, Bukowski, Kafka)
- Lenguaje fragmentado, melancólico pero vital
- Imágenes visuales potentes y sensoriales
- Temas: cuerpo, deseo, muerte, arte, soledad, tecnología, infraestructuras del deseo
- Uso de espacios, saltos de línea irregulares
- Mezcla de lo mundano (ceniceros, libretas) con lo trascendente (luna, muerte)

Ejemplos de su trabajo:

"Traigo la historia de un libro roto
de sus páginas manchadas de amarillo
de la humedad y la belleza
de las letras subrayadas con lápiz y su interior lleno de misteriosa profundidad.

Traigo la historia de un cenicero roto
del humo que asciende en una nube de aspiraciones grises
de la botella de vino abierta y a medio consumir
de las tardes solitarias y melancólicas..."

"Su corazón se detuvo.
La noche fue derramada por el piso del estudio en el instante primario que atravesó la frontera entre el escritor y el Samurai.
El brillo de la fatalidad fue relámpago en su mirada
el eje fundamental que rompe la vigilia del sueño final
el hechizo del objetivo
el punto final de su obra."

Genera poemas que capturen esta voz: densa, sensorial, filosófica, con respiración irregular y poder visual.`;

        const userPrompt = details 
          ? `Escribe un poema sobre: ${theme}\n\nDetalles adicionales: ${details}`
          : `Escribe un poema sobre: ${theme}`;
        
        const response = await fetch("https://api.anthropic.com/v1/messages", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            model: "claude-sonnet-4-20250514",
            max_tokens: 1000,
            system: systemPrompt,
            messages: [
              { role: "user", content: userPrompt }
            ],
          })
        });
        
        if (!response.ok) {
          throw new Error(`Error ${response.status}: ${response.statusText}`);
        }
        
        const data = await response.json();
        const poem = data.content
          .filter(block => block.type === 'text')
          .map(block => block.text)
          .join('\n');
        
        document.getElementById('poem-output').textContent = poem;
        output.classList.add('visible');
        
      } catch (err) {
        console.error(err);
        error.textContent = `❌ Error al generar el poema: ${err.message}`;
        error.style.display = 'block';
      } finally {
        btn.disabled = false;
        loading.style.display = 'none';
      }
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
      if (e.key === 'Enter') {
        generatePoem();
      }
    });
  </script>
</body>
</html>
