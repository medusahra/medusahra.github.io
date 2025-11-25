<!DOCTYPE html>
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
      font-family: 'MS Gothic', 'Courier New', monospace;
      font-size: 5px;
      line-height: 0.65;
      letter-spacing: -0.5px;
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
        <input type="range" id="widthSlider" min="100" max="300" value="200">
        <div class="value-display">Ancho: <span id="widthValue">200</span> chars</div>
      </div>
      
      <div class="control-group">
        <label>Character Set</label>
        <select id="charsetSelect">
          <option value="hiragana">Hiragana Japanese (Best)</option>
          <option value="blocks">Block Characters</option>
          <option value="braille">Braille Dots</option>
          <option value="dense">Dense Blocks</option>
          <option value="standard">Standard ASCII</option>
          <option value="minimal">Minimal</option>
          <option value="binary">Binary (01)</option>
          <option value="symbols">Symbols</option>
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
        coded with JavaScript by <a href="https://github.com/medusahra" target="_blank" style="color: #e0aaff; text-decoration: none; text-shadow: 0 0 8px #e0aaff;">medusahra</a>
      </p>
    </div>
  </div>
  
  <script>
    var currentImage = null;
    var asciiText = '';
    
    var charsets = {
      hiragana: ' ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ',
      blocks: ' ░▒▓█',
      standard: ' .:-=+*@',
      dense: ' ▁▂▃▄▅▆▇█',
      minimal: ' .oO@',
      binary: ' 01',
      symbols: ' ·+*@',
      braille: ' ⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏'
    };
    
    var imageInput = document.getElementById('imageInput');
    var imagePreview = document.getElementById('imagePreview');
    var noImage = document.getElementById('noImage');
    var asciiOutput = document.getElementById('asciiOutput');
    var generateBtn = document.getElementById('generateBtn');
    var copyBtn = document.getElementById('copyBtn');
    var downloadBtn = document.getElementById('downloadBtn');
    var downloadImgBtn = document.getElementById('downloadImgBtn');
    var widthSlider = document.getElementById('widthSlider');
    var widthValue = document.getElementById('widthValue');
    
    widthSlider.addEventListener('input', function(e) {
      widthValue.textContent = e.target.value;
    });
    
    imageInput.addEventListener('change', function(e) {
      var file = e.target.files[0];
      if (file) {
        var reader = new FileReader();
        reader.onload = function(event) {
          var img = new Image();
          img.onload = function() {
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
      
      var width = parseInt(widthSlider.value);
      var selectedCharset = document.getElementById('charsetSelect').value;
      var charset = charsets[selectedCharset];
      var colorMode = document.getElementById('colorMode').value;
      
      var canvas = document.createElement('canvas');
      var ctx = canvas.getContext('2d');
      
      var aspectMultiplier;
      if (selectedCharset === 'hiragana') {
        aspectMultiplier = 0.9;
      } else if (selectedCharset === 'blocks' || selectedCharset === 'dense') {
        aspectMultiplier = 0.5;
      } else if (selectedCharset === 'braille') {
        aspectMultiplier = 0.4;
      } else {
        aspectMultiplier = 0.55;
      }
      
      var aspectRatio = currentImage.height / currentImage.width;
      var height = Math.floor(width * aspectRatio * aspectMultiplier);
      
      canvas.width = width;
      canvas.height = height;
      
      ctx.drawImage(currentImage, 0, 0, width, height);
      var imageData = ctx.getImageData(0, 0, width, height);
      
      var ascii = '';
      
      for (var y = 0; y < height; y++) {
        for (var x = 0; x < width; x++) {
          var i = (y * width + x) * 4;
          var r = imageData.data[i];
          var g = imageData.data[i + 1];
          var b = imageData.data[i + 2];
          
          var brightness = (r + g + b) / 3;
          var charIndex = Math.floor((brightness / 255) * (charset.length - 1));
          var char = charset[charIndex];
          
          var colorStyle = '';
          
          if (colorMode === 'medusahra') {
            var intensity = brightness / 255;
            var pink = Math.floor(255 * intensity);
            var magenta = Math.floor(150 * intensity);
            var violet = Math.floor(180 * intensity);
            var glowStrength = intensity > 0.7 ? '0 0 15px #ff1493, 0 0 25px #c77dff' : '0 0 8px #9d4edd';
            colorStyle = 'color: rgb(' + pink + ', ' + magenta + ', ' + violet + '); text-shadow: ' + glowStrength;
          } else if (colorMode === 'gradient') {
            var hue = 270 + (y / height) * 60;
            var lightness = 30 + (brightness / 255) * 50;
            colorStyle = 'color: hsl(' + hue + ', 80%, ' + lightness + '%); text-shadow: 0 0 5px hsl(' + hue + ', 80%, ' + lightness + '%)';
          } else if (colorMode === 'monochrome') {
            var violetIntensity = 50 + (brightness / 255) * 50;
            colorStyle = 'color: hsl(280, 70%, ' + violetIntensity + '%)';
          } else if (colorMode === 'inverted') {
            var inverted = 255 - brightness;
            var invertedLight = 30 + (inverted / 255) * 50;
            colorStyle = 'color: hsl(270, 80%, ' + invertedLight + '%)';
          } else if (colorMode === 'matrix') {
            var greenShade = Math.floor(100 + (brightness / 255) * 155);
            colorStyle = 'color: rgb(0, ' + greenShade + ', 0); text-shadow: 0 0 5px rgba(0, 255, 0, 0.5)';
          } else if (colorMode === 'matrix-rain') {
            var greenIntensity = Math.floor(brightness);
            var glow = brightness > 200 ? '0 0 10px rgba(0, 255, 0, 0.8)' : '0 0 3px rgba(0, 255, 0, 0.3)';
            colorStyle = 'color: rgb(0, ' + greenIntensity + ', 0); text-shadow: ' + glow;
          } else if (colorMode === 'gameboy') {
            var palette = ['#0f380f', '#306230', '#8bac0f', '#9bbc0f'];
            var paletteIndex = Math.floor((brightness / 255) * 3);
            colorStyle = 'color: ' + palette[paletteIndex];
          } else if (colorMode === 'polaroid') {
            var warmth = Math.floor(brightness * 0.9);
            var warmR = Math.min(255, warmth + 40);
            var warmG = warmth;
            var warmB = Math.max(0, warmth - 20);
            colorStyle = 'color: rgb(' + warmR + ', ' + warmG + ', ' + warmB + ')';
          } else if (colorMode === 'vaporwave') {
            var vaporColors = brightness > 170 ? '#ff71ce' : brightness > 85 ? '#01cdfe' : '#05ffa1';
            colorStyle = 'color: ' + vaporColors + '; text-shadow: 0 0 8px ' + vaporColors;
          } else {
            colorStyle = 'color: #c77dff';
          }
          
          ascii += '<span style="' + colorStyle + '">' + char + '</span>';
        }
        ascii += '\n';
      }
      
      asciiOutput.innerHTML = ascii;
      asciiText = asciiOutput.innerText;
      
      copyBtn.disabled = false;
      downloadBtn.disabled = false;
      downloadImgBtn.disabled = false;
    }
    
    copyBtn.addEventListener('click', function() {
      navigator.clipboard.writeText(asciiText).then(function() {
        alert('¡ASCII copiado al portapapeles! 💜');
      });
    });
    
    downloadBtn.addEventListener('click', function() {
      var blob = new Blob([asciiText], { type: 'text/plain' });
      var url = URL.createObjectURL(blob);
      var a = document.createElement('a');
      a.href = url;
      a.download = 'ultraviolet-ascii.txt';
      a.click();
      URL.revokeObjectURL(url);
    });
    
    downloadImgBtn.addEventListener('click', function() {
      var canvas = document.createElement('canvas');
      var ctx = canvas.getContext('2d');
      
      var fontSize = 5;
      var lineHeight = 5;
      var padding = 20;
      
      var lines = asciiOutput.innerText.split('\n');
      var maxLineLength = 0;
      for (var i = 0; i < lines.length; i++) {
        if (lines[i].length > maxLineLength) {
          maxLineLength = lines[i].length;
        }
      }
      
      canvas.width = maxLineLength * fontSize * 0.6 + padding * 2;
      canvas.height = lines.length * lineHeight + padding * 2;
      
      ctx.fillStyle = '#000000';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      ctx.font = fontSize + 'px "Courier New", monospace';
      ctx.textBaseline = 'top';
      
      var spans = asciiOutput.querySelectorAll('span');
      var currentLine = 0;
      var currentX = padding;
      var charCount = 0;
      
      for (var i = 0; i < spans.length; i++) {
        var span = spans[i];
        var char = span.textContent;
        
        if (char === '\n' || char === '') {
          currentLine++;
          currentX = padding;
          charCount = 0;
        } else {
          var computedStyle = window.getComputedStyle(span);
          var color = computedStyle.color;
          
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
      }
      
      canvas.toBlob(function(blob) {
        var url = URL.createObjectURL(blob);
        var a = document.createElement('a');
        a.href = url;
        a.download = 'ultraviolet-ascii.png';
        a.click();
        URL.revokeObjectURL(url);
      });
    });
  </script>
</body>
</html>
