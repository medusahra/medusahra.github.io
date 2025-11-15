---
layout: home
---
<img src="/assets/logo-small.png" alt="Logo" class="logo-home" style="max-width: 200px; display: block; margin: 80px auto 20px; filter: drop-shadow(0 0 20px #ff1493);">
 
<h1 style="line-height: 1.2;">
hola soy 𝕲𝖎𝖌𝖎 𝕮𝖍𝖆𝖉𝖎𝖉 ⚔️👺⚔️ <br>
AKA ♡ 𝖒𝖊𝖉𝖚𝖘𝖆𝖍𝖗𝖆
</h1>

Bienvenidxxs a mi sitio web 🏴‍☠️

he levantado mi propio templo de código y deseo: un glitch rosa en el mapa.
## whoami
colombo lebanese 🇱🇧 hacker <br>
enfocada en seguridad ofensiva <br>
*fine & dandy honeypot*
todo tiene fisuras: sistemas, cuerpos e infraestructuras del deseo, a esas grietas me asomo. ♡
Escribo filosofía, ensayo, crítica cultural y poemas quasi-ensayísticos vagamente lujuriosos.

<h1 style="line-height: 1.2;">
𝑁𝑒𝑢𝑟𝑜𝑞𝑢𝑒𝑒𝑟 𝐻𝑒𝑟𝑒𝑠𝑖𝑒𝑠 
</h1>

## literatura
- **[archivo de textos >](/archive.html)**
- [descarga mi libro: weltschmerz >](/2025/11/11/weltschmerz.html)
- **[existir desde la neurodivergencia >](/2025/11/13/neurodivergente.html)**
- <a href="https://goadletter.substack.com/" target="_blank">goad letters en substack ></a>
- **<a href="https://www.goodreads.com/author/show/52505402.Giovanna_Chadid" target="_blank">goodreads author ></a>**
- **[enlaces de interés >](/enlaces.html)**
## cosas que hice cuando debería estar trabajando
- ☣ **<a href="https://open.spotify.com/user/oer0flire345iy1g4vaifowzc/playlists" target="_blank">listen to my sick playlists</a>** ☣
## otros proyectos
- 💎 **<a href="https://instagram.com/flabelum" target="_blank">flabelum</a>** · sacred objects for the body ·
## contacto
- GitHub: <a href="https://github.com/medusahra" target="_blank">@medusahra</a>
- Email: <a href="mailto:blackobjkt@protonmail.com">blackobjkt@protonmail.com</a>
- Instagram: <a href="https://www.instagram.com/medusahra/" target="_blank">@medusahra</a>
- X: <a href="https://x.com/medusahra" target="_blank">@medusahra</a>
## apoya mi trabajo
- ☕ <a href="https://ko-fi.com/chadidgiovanna" target="_blank">ko-fi</a>
- 💸 <a href="https://www.paypal.com/paypalme/medusahra" target="_blank">paypal</a>
<div style="text-align: center; margin-top: 80px; margin-bottom: 40px;">
<em>𝔫𝔬𝔫 𝔞𝔡 𝔩𝔲𝔠𝔢𝔪, 𝔰𝔢𝔡 𝔦𝔫 𝔦𝔫𝔠𝔢𝔫𝔡𝔦𝔲𝔪</em>
</div>
<div style="text-align: center; margin-top: 40px;">
<img src="/assets/piratepinkglam2x.png" alt="Pirate Flag" style="max-width: 150px; filter: drop-shadow(0 0 15px #ff1493);">
</div>

<!-- Reproductor Musical -->
<div class="music-player-home">
  <div class="player-home-title">♪ soundtrack ♪</div>
  <div class="player-home-track" id="homeTrackInfo">click play para comenzar</div>
  <div class="player-home-controls">
    <button class="player-home-btn" id="homePrevBtn" title="Anterior">⏮</button>
    <button class="player-home-btn" id="homePlayBtn" title="Play">▶</button>
    <button class="player-home-btn" id="homeNextBtn" title="Siguiente">⏭</button>
  </div>
  <div class="player-home-volume">
    <span class="player-home-volume-label">vol</span>
    <input type="range" class="player-home-slider" id="homeVolumeSlider" min="0" max="100" value="70">
  </div>
</div>

<script>
const homePlaylist = [
  { name: "impenetrable", file: "https://medusahra.github.io/assets/music/impenetrable.mp3" },
  { name: "rootkali", file: "https://medusahra.github.io/assets/music/rootkali.mp3" },
  { name: "405.3", file: "https://medusahra.github.io/assets/music/405.3.mp3" },
  { name: "409.1", file: "https://medusahra.github.io/assets/music/409.1.mp3" },
  { name: "409.3", file: "https://medusahra.github.io/assets/music/409.3.mp3" }
];

let homeCurrentTrack = 0;
let homeIsPlaying = false;
const homeAudio = new Audio();

const homePlayBtn = document.getElementById('homePlayBtn');
const homePrevBtn = document.getElementById('homePrevBtn');
const homeNextBtn = document.getElementById('homeNextBtn');
const homeTrackInfo = document.getElementById('homeTrackInfo');
const homeVolumeSlider = document.getElementById('homeVolumeSlider');

homeAudio.src = homePlaylist[0].file;
homeAudio.volume = 0.7;

function updateHomeTrackInfo() {
  homeTrackInfo.textContent = `${homeCurrentTrack + 1}/${homePlaylist.length} - ${homePlaylist[homeCurrentTrack].name}`;
}

homePlayBtn.addEventListener('click', () => {
  if (homeIsPlaying) {
    homeAudio.pause();
    homePlayBtn.textContent = '▶';
    homeIsPlaying = false;
  } else {
    homeAudio.play();
    homePlayBtn.textContent = '⏸';
    homeIsPlaying = true;
    updateHomeTrackInfo();
  }
});

homePrevBtn.addEventListener('click', () => {
  homeCurrentTrack = (homeCurrentTrack - 1 + homePlaylist.length) % homePlaylist.length;
  homeAudio.src = homePlaylist[homeCurrentTrack].file;
  if (homeIsPlaying) homeAudio.play();
  updateHomeTrackInfo();
});

homeNextBtn.addEventListener('click', () => {
  homeCurrentTrack = (homeCurrentTrack + 1) % homePlaylist.length;
  homeAudio.src = homePlaylist[homeCurrentTrack].file;
  if (homeIsPlaying) homeAudio.play();
  updateHomeTrackInfo();
});

homeAudio.addEventListener('ended', () => {
  homeCurrentTrack = (homeCurrentTrack + 1) % homePlaylist.length;
  homeAudio.src = homePlaylist[homeCurrentTrack].file;
  homeAudio.play();
  updateHomeTrackInfo();
});

homeVolumeSlider.addEventListener('input', (e) => {
  homeAudio.volume = e.target.value / 100;
});

updateHomeTrackInfo();
</script>
