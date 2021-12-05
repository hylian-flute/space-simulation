// @ts-check
/** @typedef {{get_position(): Float64Array, get_weight(): number}} Planet */
/** @typedef {{tick: () => void, get_planets: () => Planet[]}} PlanetarySystemD */

// @ts-ignore
import init, { PlanetarySystem, get_planetary_system_radius } from "./pkg/wasm_app.js";

/**
 * @template T
 * @param {unknown} instance 
 * @param {{new(): T}} Type 
 */
function typeGuard(instance, Type) {
  if (instance instanceof Type) return instance;
  throw new TypeError();
}

const CANVAS_SIZE = 640;

const fpsCounter = document.getElementById("fps-counter");
const canvases = [
  document.getElementById("planetary-system-display-x"),
  document.getElementById("planetary-system-display-y"),
  document.getElementById("planetary-system-display-z"),
].map(canvas => typeGuard(canvas, HTMLCanvasElement));

/**
 * @param {PlanetarySystemD} planetarySystem
 * @param {number} planetarySystemRadius 
 */
function displayPlanetarySystem(planetarySystem, planetarySystemRadius) {
  const contexts = canvases.map(canvas => canvas.getContext("2d"));
  const dimension = contexts.length;
  for (const context of contexts) context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
  const planets = planetarySystem.get_planets();
  const maxWieght = Math.max(...planets.map(planet => planet.get_weight()));
  for (const planet of planets) {
    const color = `rgb(255, ${Math.floor(255 * planet.get_weight() / maxWieght)}, 0)`;
    const scaledPosition = planet
      .get_position()
      .map(axisPosition =>
        Math.floor(
          (axisPosition + planetarySystemRadius) * CANVAS_SIZE / (2 * planetarySystemRadius)
        ));
    for (const [contextIdx, context] of contexts.entries()) {
      context.fillStyle = color;
      context.fillRect(
        scaledPosition[(contextIdx + 1) % dimension],
        scaledPosition[(contextIdx + 2) % dimension],
        2,
        2
      );
    }
  }
}

/**
 * @param {PlanetarySystemD} planetarySystem
 * @param {number} planetarySystemRadius 
 */
function tick(planetarySystem, planetarySystemRadius) {
  const t = Date.now();
  planetarySystem.tick();
  displayPlanetarySystem(planetarySystem, planetarySystemRadius);
  setTimeout(() => {
    const time = Date.now() - t;
    fpsCounter.textContent = Math.floor(1000 / time).toString();
    tick(planetarySystem, planetarySystemRadius);
  }, 1000 / 10);
}

// @ts-ignore
await init();
/** @type {PlanetarySystemD} */
const planetarySystem = PlanetarySystem.new();
/** @type {Number} */
const planetarySystemRadius = get_planetary_system_radius();

tick(planetarySystem, planetarySystemRadius);
