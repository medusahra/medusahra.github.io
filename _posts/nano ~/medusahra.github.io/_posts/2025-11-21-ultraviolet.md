---
title: "ultraviolet"
---
{% raw %}
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ultraviolet | medusahra</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    
    body {
      background: #000000;
      color: #ff2777;
      font-family: 'Courier New', monospace;
      padding: 20px;
      min-height: 100vh;
    }
    
    .container {
      max-width: 1200px;
      margin: 0 auto;
    }
    
    .back-btn {
      display: inline-block;
      color: #9d4edd;
      text-decoration: none;
      font-size: 14px;
      padding: 8px 15px;
      border: 1px solid #9d4edd;
      border-radius: 4px;
      margin-bottom: 20px;
      transition: all 0.3s;
    }
    
    .back-btn:hover {
      background: rgba(157, 78, 221, 0.2);
      box-shadow: 0 0 15px rgba(157, 78, 221, 0.6);
    }
    
    .header {
      text-align: center;
      margin-bottom: 40px;
    }
    
    h1 {
      font-size: 3em;
      background: linear-gradient(45deg, #9d4edd, #c77dff, #e0aaff);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
      text-shadow: 0 0 30px rgba(157, 78, 221, 0.8);
      margin-bottom: 10px;
      letter-spacing: 3px;
    }
    
    .subtitle {
      color: #c77dff;
      font-style: italic;
      text-shadow: 0 0 10px #c77dff;
    }
    
    .info-box {
      background: rgba(157, 78, 221, 0.05);
      border: 1px solid #9d4edd;
      border-radius: 8px;
      padding: 15px;
      margin-bottom: 30px;
      font-size: 13px;
    }
    
    .info-box strong {
      color: #c77dff;
    }
    
    .upload-section {
      background: rgba(157, 78, 221, 0.1);
      border: 2px solid #9d4edd;
      border-radius: 8px;
      padding: 30px;
      margin-bottom: 20px;
      box-shadow: 0 0 20px rgba(157, 78, 221, 0.3);
      text-align: center;
    }
    
    .upload-label {
      display: inline-block;
      background: linear-gradient(45deg, #9d4edd, #c77dff);
      color: #000;
      padding: 15px 40px;
      border-radius: 4px;
      cursor: pointer;
      font-weight: bold;
      transition: all 0.3s;
      box-shadow: 0 0 20px rgba(157, 78, 221, 0.5);
    }
    
    .upload-label:hover {
      box-shadow: 0 0 30px rgba(157, 78, 221, 0.8);
      transform: translateY(-2px);
    }
    
    input[type="file"] {
      display: none;
    }
    
    .controls {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
      gap: 15px;
      margin-bottom: 20px;
    }
    
    .control-group {
      background: rgba(157, 78, 221, 0.1);
      border: 1px solid #9d4edd;
      border-radius: 8px;
      padding: 15px;
    }
    
    label {
      display: block;
      color: #c77dff;
      font-weight: bold;
      margin-bottom: 8px;
      font-size: 12px;
      text-transform: uppercase;
      letter-spacing: 1px;
    }
    
    input[type="range"] {
      width: 100%;
      margin: 10px 0;
    }
    
    select {
      width: 100%;
      background: #000;
      color: #c77dff;
      border: 1px solid #9d4edd;
      padding: 8px;
      font-family: monospace;
      border-radius: 4px;
    }
    
    .value-display {
      color: #e0aaff;
      font-size: 14px;
      text-align: right;
    }
    
    .preview-section {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 20px;
      margin-bottom: 20px;
    }
    
    .preview-box {
      background: rgba(157, 78, 221, 0.05);
      border: 2px solid #9d4edd;
      border-radius: 8px;
      padding: 15px;
      min-height: 300px;
    }
    
    .preview-title {
      color: #c77dff;
      font-weight: bold;
      margin-bottom: 10px;
      text-transform: uppercase;
      font-size: 12px;
      letter-spacing: 2px;
    }
    
    #imagePreview {
      max-width: 100%;
      display: block;
      border: 1px solid #9d4edd;
      border-radius: 4px;
    }
    
    #asciiOutput {
      font-family: 'Courier New', monospace;
      font-size: 8px;
      line-height: 1;
      white-space: pre;
      overflow: auto;
      max-height: 600px;
      background: #000;
      padding: 10px;
      border-radius: 4px;
    }
    
    .actions {
      display: flex;
      gap: 10px;
      flex-wrap: wrap;
      margin-top: 20px;
    }
    
    button {
      background: rgba(157, 78, 221, 0.2);
      border: 2px solid #9d4edd;
      color: #c77dff;
      padding: 12px 25px;
      font-family: monospace;
      font-weight: bold;
      cursor: pointer;
      border-radius: 4px;
      transition: all 0.3s;
      text-shadow: 0 0 5px #c77dff;
    }
    
    button:hover {
      background: rgba(157, 78, 221, 0.4);
      box-shadow: 0 0 20px rgba(157, 78, 221, 0.6);
    }
    
    button:disabled {
      opacity: 0.3;
      cursor: not-allowed;
    }
    
    @media (max-width: 768px) {
      .preview-section {
        grid-template-columns: 1fr;
      }
      
      h1 {
        font-size: 2em;
      }
    }
  </style>
</head>
<body>
  <div class="container">
    <a href="/" class="back-btn">← volver al home</a>
    
    <div class="header">
      <h1>ULTRAVIOLET</h1>
      <p class="subtitle">hyper hex medusahra · image to ASCII converter</p>
    </div>
    
    <div class="info-box">
      <strong>→ cómo funciona:</strong> Sube una imagen y conviértela en arte ASCII ultravioleta. 
      Ajusta la densidad, elige tu set de caracteres y el algoritmo genera una representación en texto 
      con gradientes violeta/magenta. Perfecto para glitch art textual.
      <br><br>
      <em style="font-size: 11px; color: #9d4edd;">// algoritmo de conversión · JavaScript · by medusahra</em>
    </div>
    
    <div class="upload-section">
      <label for="imageInput" class="upload-label">
        📸 SELECCIONAR IMAGEN
      </label>
      <input type="file" id="imageInput" accept="image/*">
      <p style="margin-top: 15px; color: #9d4edd; font-size: 12px;">
        Soporta: JPG, PNG, GIF, WebP
      </p>
    </div>
    
    <div class="controls">
      <div class="control-group">
        <label>Densidad / Resolution</label>
        <input type="range" id="widthSlider" min="50" max="200" value="100">
        <div class="value-display">Ancho: <span id="widthValue">100</span> chars</div>
      </div>
      
      <div class="control-group">
        <label>Character Set</label>
        <select id="charsetSelect">
          <option value="standard">Standard ASCII</option>
          <option value="dense">Dense Block</option>
          <option value="minimal">Minimal</option>
          <option value="emoji">Emoji Glitch</option>
          <option value="custom">Custom Symbols</option>
          <option value="binary">Binary (01010101)</option>
          <option value="at">At Signs (@@@@@)</option>
          <option value="glitch">Glitch Mix</option>
        </select>
      </div>
      
      <div class="control-group">
        <label>Color Mode</label>
        <select id="colorMode">
          <option value="medusahra">medusahra signature</option>
          <option value="gradient">Gradient Ultraviolet</option>
          <option value="monochrome">Monochrome Violet</option>
          <option value="inverted">Inverted</option>
          <option value="matrix">Matrix Green</option>
          <option value="matrix-rain">Matrix Rain Effect</option>
          <option value="gameboy">Game Boy Camera</option>
          <option value="polaroid">Polaroid Vintage</option>
          <option value="vaporwave">Vaporwave</option>
        </select>
      </div>
    </div>
    
    <div class="preview-section">
      <div class="preview-box">
        <div class="preview-title">→ Original</div>
        <img id="imagePreview" style="display: none;">
        <div id="noImage" style="color: #9d4edd; text-align: center; padding: 50px;">
          Esperando imagen...
        </div>
      </div>
      
      <div class="preview-box">
        <div class="preview-title">→ ASCII Output</div>
        <div id="asciiOutput"></div>
      </div>
    </div>
    
    <div class="actions">
      <button id="generateBtn" disabled>⚡ Generar ASCII</button>
      <button id="copyBtn" disabled>📋 Copiar</button>
      <button id="downloadBtn" disabled>💾 Descargar .txt</button>
      <button id="downloadImgBtn" disabled>🖼️ Descargar PNG</button>
    </div>
    
    <div style="text-align: center; margin-top: 30px; padding: 20px; border-top: 1px solid #9d4edd;">
      <p style="font-size: 11px; color: #c77dff; opacity: 0.8; letter-spacing: 1px;">
        // algoritmo de conversión píxel a ASCII · JavaScript · Canvas API
      </p>
      <p style="font-size: 10px; color: #9d4edd; margin-top: 5px;">
        coded with JavaScript by <a href="https://github.com/medusahra" target="_blank" style="color: #e0aaff; text-decoration: none; text-shadow: 0 0 8px #e0aaff; transition: all 0.3s;">medusahra</a>
      </p>
    </div>
  </div>
  
  <script>
    let currentImage = null;
    let asciiText = '';
    
    const charsets = {
      standard: ' .:-=+*#%@',
      dense: ' ░▒▓█',
      minimal: ' ·▪▫■',
      emoji: ' ·✦✧★☆',
      custom: ' ◊◈◇◆',
      binary: ' 01',
      at: ' ·@@@',
      glitch: ' ▓▒░01@#'
    };
    
    const imageInput = document.getElementById('imageInput');
    const imagePreview = document.getElementById('imagePreview');
    const noImage = document.getElementById('noImage');
    const asciiOutput = document.getElementById('asciiOutput');
    const generateBtn = document.getElementById('generateBtn');
    const copyBtn = document.getElementById('copyBtn');
    const downloadBtn = document.getElementById('downloadBtn');
    const downloadImgBtn = document.getElementById('downloadImgBtn');
    const widthSlider = document.getElementById('widthSlider');
    const widthValue = document.getElementById('widthValue');
    
    widthSlider.addEventListener('input', (e) => {
      widthValue.textContent = e.target.value;
    });
    
    imageInput.addEventListener('change', (e) => {
      const file = e.target.files[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          const img = new Image();
          img.onload = () => {
            currentImage = img;
            imagePreview.src = event.target.result;
            imagePreview.style.display = 'block';
            noImage.style.display = 'none';
            generateBtn.disabled = false;
          };
          img.src = event.target.result;
        };
        reader.readAsDataURL(file);
      }
    });
    
    generateBtn.addEventListener('click', generateASCII);
    
    function generateASCII() {
      if (!currentImage) return;
      
      const width = parseInt(widthSlider.value);
      const charset = charsets[document.getElementById('charsetSelect').value];
      const colorMode = document.getElementById('colorMode').value;
      
      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d');
      
      const aspectRatio = currentImage.height / currentImage.width;
      const height = Math.floor(width * aspectRatio * 0.5);
      
      canvas.width = width;
      canvas.height = height;
      
      ctx.drawImage(currentImage, 0, 0, width, height);
      const imageData = ctx.getImageData(0, 0, width, height);
      
      let ascii = '';
      
      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          const i = (y * width + x) * 4;
          const r = imageData.data[i];
          const g = imageData.data[i + 1];
          const b = imageData.data[i + 2];
          
          const brightness = (r + g + b) / 3;
          const charIndex = Math.floor((brightness / 255) * (charset.length - 1));
          const char = charset[charIndex];
          
          let colorStyle = '';
          
          switch(colorMode) {
            case 'medusahra':
              const pinkIntensity = Math.floor(brightness);
              const glowStrength = brightness > 200 ? '0 0 15px #ff1493, 0 0 25px #ff1493' : '0 0 8px #ff1493, 0 0 12px #ff1493';
              colorStyle = `color: rgb(255, ${Math.floor(pinkIntensity * 0.3)}, ${Math.floor(pinkIntensity * 0.6)}); text-shadow: ${glowStrength}`;
              break;
              
            case 'gradient':
              const hue = 270 + (y / height) * 60;
              colorStyle = `color: hsl(${hue}, 80%, ${50 + brightness/10}%)`;
              break;
              
            case 'monochrome':
              colorStyle = `color: #c77dff`;
              break;
              
            case 'inverted':
              const inverted = 255 - brightness;
              colorStyle = `color: hsl(270, 80%, ${inverted/3}%)`;
              break;
              
            case 'matrix':
              const greenShade = Math.floor(100 + (brightness / 255) * 155);
              colorStyle = `color: rgb(0, ${greenShade}, 0); text-shadow: 0 0 5px rgba(0, 255, 0, 0.5)`;
              break;
              
            case 'matrix-rain':
              const greenIntensity = Math.floor(brightness);
              const glow = brightness > 200 ? '0 0 10px rgba(0, 255, 0, 0.8)' : '0 0 3px rgba(0, 255, 0, 0.3)';
              colorStyle = `color: rgb(0, ${greenIntensity}, 0); text-shadow: ${glow}`;
              break;
              
            case 'gameboy':
              const palette = ['#0f380f', '#306230', '#8bac0f', '#9bbc0f'];
              const paletteIndex = Math.floor((brightness / 255) * 3);
              colorStyle = `color: ${palette[paletteIndex]}`;
              break;
              
            case 'polaroid':
              const warmth = Math.floor(brightness * 0.9);
              colorStyle = `color: rgb(${warmth + 40}, ${warmth}, ${warmth - 20})`;
              break;
              
            case 'vaporwave':
              const vaporColors = brightness > 170 ? '#ff71ce' : brightness > 85 ? '#01cdfe' : '#05ffa1';
              colorStyle = `color: ${vaporColors}; text-shadow: 0 0 8px ${vaporColors}`;
              break;
              
            default:
              colorStyle = `color: #c77dff`;
          }
          
          ascii += `<span style="${colorStyle}">${char}</span>`;
        }
        ascii += '\n';
      }
      
      asciiOutput.innerHTML = ascii;
      asciiText = asciiOutput.innerText;
      
      copyBtn.disabled = false;
      downloadBtn.disabled = false;
      downloadImgBtn.disabled = false;
    }
    
    copyBtn.addEventListener('click', () => {
      navigator.clipboard.writeText(asciiText).then(() => {
        alert('¡ASCII copiado al portapapeles! 💜');
      });
    });
    
    downloadBtn.addEventListener('click', () => {
      const blob = new Blob([asciiText], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'ultraviolet-ascii.txt';
      a.click();
      URL.revokeObjectURL(url);
    });
    
    // Descarga como PNG
    downloadImgBtn.addEventListener('click', () => {
      const asciiDiv = document.getElementById('asciiOutput');
      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d');
      
      const fontSize = 8;
      const lineHeight = 8;
      const padding = 20;
      
      const lines = asciiOutput.innerText.split('\n');
      const maxLineLength = Math.max(...lines.map(l => l.length));
      
      canvas.width = maxLineLength * fontSize * 0.6 + padding * 2;
      canvas.height = lines.length * lineHeight + padding * 2;
      
      ctx.fillStyle = '#000000';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      ctx.font = `${fontSize}px "Courier New", monospace`;
      ctx.textBaseline = 'top';
      
      const spans = asciiDiv.querySelectorAll('span');
      let currentLine = 0;
      let currentX = padding;
      let charCount = 0;
      
      spans.forEach(span => {
        const char = span.textContent;
        
        if (char === '\n' || char === '') {
          currentLine++;
          currentX = padding;
        } else {
          const computedStyle = window.getComputedStyle(span);
          const color = computedStyle.color;
          
          ctx.fillStyle = color;
          ctx.fillText(char, currentX, padding + currentLine * lineHeight);
          currentX += fontSize * 0.6;
          charCount++;
          
          if (charCount >= maxLineLength) {
            currentLine++;
            currentX = padding;
            charCount = 0;
          }
        }
      });
      
      canvas.toBlob(blob => {
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'ultraviolet-ascii.png';
        a.click();
        URL.revokeObjectURL(url);
      });
    });
  </script>
</body>
</html>
{% endraw %}
