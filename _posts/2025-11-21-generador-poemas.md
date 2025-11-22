---
layout: default
title: "Generador Poetico"
date: 2025-11-21
---

<div style="max-width: 800px; margin: 0 auto;">

<a href="/" style="color: #ff1493; text-decoration: none; font-size: 14px; display: inline-block; padding: 8px 15px; border: 1px solid #ff1493; border-radius: 4px; margin-bottom: 20px; transition: all 0.3s;">
  ← volver al home
</a>

<div style="text-align: center; margin-bottom: 40px;">
  <h1 style="color: #ff1493; text-shadow: 0 0 10px #ff1493, 0 0 20px #ff1493; font-size: 2em; margin-bottom: 10px;">
    🪶 Generador Poético 🪶
  </h1>
  <p style="color: #ffffff; text-shadow: 0 0 5px #ffffff; font-style: italic; font-size: 0.9em;">
    poemas cuasi-ensayísticos vagamente lujuriosos
  </p>
</div>

<!-- Instructivo -->
<div style="background: rgba(255, 20, 147, 0.05); border: 1px solid #ff1493; border-radius: 8px; padding: 15px; margin-bottom: 30px; font-size: 13px; line-height: 1.6;">
  <p style="color: #ff1493; font-weight: bold; margin-bottom: 10px;">
    → cómo funciona
  </p>
  <p style="margin-bottom: 10px;">
    Este generador usa <strong>cadenas de Markov</strong> para analizar patrones en poemas
    y crear nuevas combinaciones. El corpus incluye fragmentos de mis textos junto con poemas 
    de dominio público (Lorca, Vallejo, Storni, Neruda) creando constelaciones algorítmicas únicas.
  </p>
  <p style="font-size: 11px; color: #ff2777; font-style: italic;">
    // generador algorítmico · JavaScript · by medusahra
  </p>
</div>

<!-- Input Section -->
<div style="background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 20px; margin-bottom: 20px; box-shadow: 0 0 20px rgba(255, 20, 147, 0.3), inset 0 0 20px rgba(255, 20, 147, 0.05);">
  <label style="display: block; color: #ff1493; font-weight: bold; margin-bottom: 10px;">
    tema o palabra semilla (opcional):
  </label>
  <input
    type="text"
    id="theme-input"
    placeholder="ej: muerte, soledad, fuego..."
    style="width: 100%; background: #000; color: #ff2777; border: 1px solid #ff1493; padding: 12px; font-family: monospace; font-size: 14px; border-radius: 4px; margin-bottom: 15px;"
  />

  <button
    onclick="generatePoem()"
    id="generate-btn"
    class="generate-btn"
    style="background: #ff1493; color: #000; border: none; padding: 15px 40px; font-family: monospace; font-weight: bold; font-size: 16px; cursor: pointer; border-radius: 4px; width: 100%; transition: all 0.3s; box-shadow: 0 0 10px rgba(255, 20, 147, 0.5);"
  >
    🪶 Generar Poema
  </button>
</div>

<div id="loading" style="display: none; text-align: center; color: #ff1493; font-style: italic; margin: 20px 0;">
  ⏳ Generando tu poema...
</div>

<!-- Output Section -->
<div id="output-section" style="display: none; background: rgba(255, 20, 147, 0.1); border: 2px solid #ff1493; border-radius: 8px; padding: 20px; margin-top: 20px; box-shadow: 0 0 20px rgba(255, 20, 147, 0.3), inset 0 0 20px rgba(255, 20, 147, 0.05);">
  <label style="display: block; color: #ff1493; font-weight: bold; margin-bottom: 15px;">
    tu poema:
  </label>
  <div id="poem-output" style="color: #00ff00; line-height: 1.8; white-space: pre-wrap; font-size: 15px; margin-bottom: 20px; min-height: 200px;"></div>
  <button
    onclick="copyPoem()"
    class="copy-btn"
    style="background: rgba(255, 20, 147, 0.2); border: 2px solid #ff1493; color: #ff1493; padding: 10px 20px; font-family: monospace; font-weight: bold; cursor: pointer; border-radius: 4px; transition: all 0.3s; text-shadow: 0 0 5px #ff1493;"
  >
    📋 Copiar Poema
  </button>
</div>

<!-- Footer info -->
<div style="text-align: center; margin-top: 40px; font-size: 11px; color: #ff2777; opacity: 0.6;">
  <p id="corpus-info"></p>
  <p style="margin-top: 5px;">
    algoritmo de cadenas de Markov · orden 2 · JavaScript ES6
  </p>
</div>

</div>

<style>
.generate-btn:hover {
  box-shadow: 0 0 20px rgba(255, 20, 147, 0.8) !important;
  transform: translateY(-2px);
}

.copy-btn:hover {
  background: rgba(255, 20, 147, 0.4) !important;
  box-shadow: 0 0 15px rgba(255, 20, 147, 0.6);
}

a:hover {
  background: rgba(255, 20, 147, 0.2) !important;
}
</style>

<script>
// Corpus expandido: poemas propios + dominio público
const corpus = `
Traigo la historia de un libro roto
de sus páginas manchadas de amarillo
de la humedad y la belleza
de las letras subrayadas con lápiz y su interior lleno de misteriosa profundidad
Traigo la historia de un cenicero roto
del humo que asciende en una nube de aspiraciones grises
de la botella de vino abierta y a medio consumir
de las tardes solitarias y melancólicas
de los días en que caminamos solos
dormimos solos
amanecemos solos
para ver a través de la ventana que el mundo no se ha detenido ni un sólo instante
por nosotros
Traigo perfume de jazmines amapolas y geranios
recuerdos confusos cartas ilegibles tinta oxidada sobre la alfombra
traigo el olor de la almaza en mi garganta
las noches de luna llena
el aire condensado en eternidad
Traigo conmigo los juegos de la infancia rota la sencillez que viste glamorosa
las pestañas negras y profundas como un nocturno de Chopin
Traigo un morral de cuero gastado lleno de
Saavedras Bukowskis Kafkas Qabbanis Gibranes Darwishes Delmares
Cuervos y poemas junto a los bolígrafos
Negros Azules
Quizá también grises como el cielo de hoy
Traigo la noche en mis ojos
las ojeras la piel porosa la piel pálida la piel dormida
las uñas color violeta la muerte color violeta
Traigo días soleados y calurosos
luces que brillan en la retina
traigo fatiga
y la lengua seca y adormecida y filosa
traigo frío traigo niebla
traigo versos cubiertos de melancolía
versos que huelen a jengibre
versos que se esfuman y huyen y se camuflan
por las paredes resquebrajadas de Beirut
Traigo camellos y escarabajos
traigo agujas y moscardones
traigo tu mirada y tu voz
Traigo esta libreta roja con poemas rojos y sangre roja
traigo fechas y arrebatos cartas y cumpleaños
e imágenes contrapuestas bien guardadas que la gente no conoce
traigo vacío traigo vacío

Su corazón se detuvo
La noche fue derramada por el piso del estudio
en el instante primario que atravesó la frontera entre el escritor y el Samurai
El brillo de la fatalidad fue relámpago en su mirada
el eje fundamental que rompe la vigilia del sueño final
el hechizo del objetivo
el punto final de su obra
Planificar la muerte es la anticipación del trabajo
en la obra de arte perfecta más grande de la vida
El artista frente al espejo del no retorno
El tono de rojo más preciso
el golpe de estado en la otredad
el kanji definitivo
El método adecuado desarrollado durante varios siglos
Hundir la espada corta en el lado izquierdo del abdomen
deslizar la hoja lateralmente hacia la derecha
luego girarla hacia arriba
Herido de muerte el silencio resumió su noche en un grito sordo
La furia resumida
las palabras de los otros fueron ecos vacíos carentes de inmortalidad
Un impulso bastó para representar algo universal
Cuando nos enfrentamos a la muerte
nuestra propia muerte o a la muerte ajena
nos enfrentamos a la pregunta
Cómo seremos recordados
Una hoja de plata brillante afilada
se mimetiza entre las hojas bajo la luna de invierno

Todo cabe dentro del silencio
la mirada la voz la música
el templo la carroza la muerte
el hombre quiso destruir el hambre de los sótanos
confundiendo el ocaso con el amanecer
intentó implantar belleza como ideología
modificando el curso natural a su antojo
niños jugando en el patio trasero haciendo y deshaciendo castillos
A la gran máquina no le importaron sus deseos los derrumbó todos
La gran máquina soñó con llenar de oscuridad sus pupilas
las grietas fueron más grandes que los reinos
los hombres fueron borrados de la historia y todos sus movimientos
sus luchas sus antojos
se encuentran en bóvedas que nada ven
triturados bajo tierra
las tinieblas son su nuevo dios
visible e invisible
El más allá está aquí mismo coexistiendo
dentro de las grietas

Verde que te quiero verde
verde viento verdes ramas
El barco sobre la mar
y el caballo en la montaña
Con la sombra en la cintura
ella sueña en su baranda
verde carne pelo verde
con ojos de fría plata
Verde que te quiero verde
Bajo la luna gitana
las cosas la están mirando
y ella no puede mirarlas

La aurora de Nueva York tiene
cuatro columnas de cieno
y un huracán de negras palomas
que chapotean las aguas podridas
La aurora de Nueva York gime
por las inmensas escaleras
buscando entre las aristas
nardos de angustia dibujada

Me moriré en París con aguacero
un día del cual tengo ya el recuerdo
Me moriré en París y no me corro
tal vez un jueves como es hoy de otoño
Jueves será porque hoy jueves que proso
estos versos los húmeros me he puesto
a la mala y jamás como hoy me he vuelto
con todo mi camino a verme solo
César Vallejo ha muerto le pegaban
todos sin que él les haga nada
le daban duro con un palo y duro
también con una soga

Peso ancestral
Tú me dijiste no lloró tu padre
tú me dijiste no lloró tu abuelo
no han llorado los hombres de mi raza
eran de acero
Así diciendo te brotó una lágrima
y me cayó en la boca más veneno
yo no he bebido nunca en otro vaso
así pequeño
Débil mujer pobre mujer que entiende
dolor de siglos conocí al beberlo
Oh el alma mía soportar no puede
todo su peso

Voy a dormir nodriza mía acuéstame
Ponme una lámpara a la cabecera
una constelación la que te guste
todas son buenas bájame las manos
Me las han visto y ya no me las quiero
y déjame sola oyes romper los brotes
te acuna un pie celeste desde arriba
y un pájaro te traza unos compases

La noche está estrellada
y tiritan azules los astros a lo lejos
El viento de la noche gira en el cielo y canta
Puedo escribir los versos más tristes esta noche
Escribir por ejemplo la noche está estrellada
y tiritan azules los astros a lo lejos
El viento de la noche gira en el cielo y canta

Cuerpo de mujer blancas colinas muslos blancos
te pareces al mundo en tu actitud de entrega
Mi cuerpo de labriego salvaje te socava
y hace saltar el hijo del fondo de la tierra
`;

// Construir cadenas de Markov
function buildMarkovChain(text, order = 2) {
  const words = text.toLowerCase()
    .replace(/[^\wáéíóúñ\s]/g, ' ')
    .split(/\s+/)
    .filter(w => w.length > 0);
  
  const chain = {};
  
  for (let i = 0; i < words.length - order; i++) {
    const key = words.slice(i, i + order).join(' ');
    const next = words[i + order];
    
    if (!chain[key]) {
      chain[key] = [];
    }
    chain[key].push(next);
  }
  
  return chain;
}

function generateFromMarkov(chain, length = 80) {
  const keys = Object.keys(chain);
  let currentKey = keys[Math.floor(Math.random() * keys.length)];
  let result = currentKey.split(' ');
  
  for (let i = 0; i < length; i++) {
    const possibilities = chain[currentKey];
    if (!possibilities || possibilities.length === 0) {
      currentKey = keys[Math.floor(Math.random() * keys.length)];
      continue;
    }
    
    const nextWord = possibilities[Math.floor(Math.random() * possibilities.length)];
    result.push(nextWord);
    
    const words = currentKey.split(' ');
    words.shift();
    words.push(nextWord);
    currentKey = words.join(' ');
  }
  
  return result.join(' ');
}

function formatPoem(text) {
  const lines = [];
  const words = text.split(' ');
  let currentLine = '';
  
  for (let i = 0; i < words.length; i++) {
    currentLine += words[i] + ' ';
    
    if (Math.random() < 0.15 || currentLine.length > 60) {
      lines.push(currentLine.trim());
      currentLine = '';
      
      if (Math.random() < 0.1) {
        lines.push('');
      }
    }
  }
  
  if (currentLine) {
    lines.push(currentLine.trim());
  }
  
  return lines.join('\n');
}

function generatePoem() {
  const btn = document.getElementById('generate-btn');
  const loading = document.getElementById('loading');
  const output = document.getElementById('output-section');
  
  btn.disabled = true;
  btn.style.opacity = '0.5';
  loading.style.display = 'block';
  output.style.display = 'none';
  
  setTimeout(() => {
    const chain = buildMarkovChain(corpus, 2);
    const rawPoem = generateFromMarkov(chain, 60 + Math.floor(Math.random() * 40));
    const formatted = formatPoem(rawPoem);
    
    document.getElementById('poem-output').textContent = formatted;
    output.style.display = 'block';
    loading.style.display = 'none';
    btn.disabled = false;
    btn.style.opacity = '1';
  }, 800);
}

function copyPoem() {
  const poem = document.getElementById('poem-output').textContent;
  navigator.clipboard.writeText(poem).then(() => {
    alert('¡Poema copiado! 🪶');
  }).catch(err => {
    alert('Error al copiar: ' + err);
  });
}

// Info del corpus
document.getElementById('corpus-info').textContent = 
  `generador basado en corpus de ${corpus.split(' ').length} palabras`;

// Enter para generar
document.getElementById('theme-input').addEventListener('keypress', (e) => {
  if (e.key === 'Enter') {
    generatePoem();
  }
});
</script>
