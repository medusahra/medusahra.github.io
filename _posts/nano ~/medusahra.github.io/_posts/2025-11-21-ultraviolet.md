<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>ULTRAVIOLET ASCII Converter</title>
<style>
  body {
    background-color: #000;
    color: #fff;
    font-family: monospace;
    margin: 0;
    padding: 0;
  }
  .container {
    width: 95%;
    margin: 20px auto;
    text-align: center;
  }
  #asciiOutput {
    font-size: 10px;
    white-space: pre;
    line-height: 1;
    padding: 10px;
    background-color: #000;
    color: #fff;
    display: inline-block;
    margin-top: 20px;
    border: 1px solid #333;
  }
  #preview {
    max-width: 300px;
    margin-top: 20px;
    border: 1px solid #555;
  }
  select, input[type="number"], input[type="file"], button {
    background: #111;
    color: #fff;
    border: 1px solid #444;
    padding: 5px;
    margin: 5px;
  }
  button:disabled {
    opacity: 0.4;
  }
</style>
</head>
<body>

<div class="container">
  <h1>ULTRAVIOLET ASCII Converter</h1>

  <input type="file" id="imageUpload" accept="image/*">
  <br>

  <img id="preview" style="display:none">

  <br>
  <select id="charsetSelect">
    <option value="blocks">Block Characters</option>
    <option value="braille">Braille Dots</option>
    <option value="dense">Dense Blocks</option>
    <option value="standard">Standard ASCII</option>
    <option value="minimal">Minimal</option>
    <option value="binary">Binary (01)</option>
    <option value="symbols">Symbols</option>
  </select>

  <br>
  <label>Width:</label>
  <input type="number" id="widthInput" min="20" max="500" value="200">

  <label>Height:</label>
  <input type="number" id="heightInput" min="20" max="500" value="200">

  <br>
  <button id="generateBtn" disabled>Generate ASCII</button>
  <button id="downloadTxtBtn" disabled>Download as .txt</button>

  <div id="asciiOutput"></div>
</div>

<script>
var charsets = {
  blocks: ' ░▒▓█',
  standard: ' .:-=+*@',
  dense: ' ▁▂▃▄▅▆▇█',
  minimal: ' .oO@',
  binary: ' 01',
  symbols: ' ·+*@',
  braille: ' ⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏'
};

var imageUpload = document.getElementById('imageUpload');
var preview = document.getElementById('preview');
var charsetSelect = document.getElementById('charsetSelect');
var asciiOutput = document.getElementById('asciiOutput');
var generateBtn = document.getElementById('generateBtn');
var downloadTxtBtn = document.getElementById('downloadTxtBtn');

imageUpload.addEventListener('change', function () {
  var file = imageUpload.files[0];
  if (file) {
    preview.src = URL.createObjectURL(file);
    preview.style.display = 'block';
    generateBtn.disabled = false;
  }
});

function escapeHTML(str) {
  return str.replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;");
}

generateBtn.addEventListener('click', function () {
  var img = new Image();
  img.src = preview.src;

  img.onload = function () {
    var width = parseInt(document.getElementById('widthInput').value);
    var height = parseInt(document.getElementById('heightInput').value);
    var selectedCharset = charsetSelect.value;

    var charset = charsets[selectedCharset];
    var canvas = document.createElement('canvas');
    var ctx = canvas.getContext('2d');
    canvas.width = width;
    canvas.height = height;

    ctx.drawImage(img, 0, 0, width, height);
    var imageData = ctx.getImageData(0, 0, width, height).data;

    var aspectMultiplier;

    if (selectedCharset === 'blocks' || selectedCharset === 'dense') {
      aspectMultiplier = 0.5;
    } else if (selectedCharset === 'braille') {
      aspectMultiplier = 0.4;
    } else {
      aspectMultiplier = 0.55;
    }

    var ascii = '';

    for (var y = 0; y < height; y += aspectMultiplier) {
      for (var x = 0; x < width; x++) {
        var index = (Math.floor(y) * width + x) * 4;
        var r = imageData[index];
        var g = imageData[index + 1];
        var b = imageData[index + 2];
        var brightness = (r + g + b) / 3;

        var charIndex = Math.floor((brightness / 255) * (charset.length - 1));
        var char = charset[charIndex] || charset[0];

        var colorStyle = "color: rgb(" + r + "," + g + "," + b + ");";
        ascii += '<span style="' + colorStyle + '">' + escapeHTML(char) + '</span>';
      }
      ascii += '<br>';
    }

    asciiOutput.innerHTML = ascii;
    downloadTxtBtn.disabled = false;
  }
});

downloadTxtBtn.addEventListener('click', function () {
  var text = asciiOutput.innerText;
  var blob = new Blob([text], { type: "text/plain" });
  var link = document.createElement("a");
  link.download = "ascii_art.txt";
  link.href = URL.createObjectURL(blob);
  link.click();
});
</script>

</body>
</html>
