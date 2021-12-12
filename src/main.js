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

let prevSecond = Math.floor(Date.now() / 1000);
let frameCount = 0;

/**
 * @param {PlanetarySystemD} planetarySystem
 * @param {number} planetarySystemRadius 
 */
function displayPlanetarySystem(planetarySystem, planetarySystemRadius) {
  const contexts = canvases.map(canvas => canvas.getContext("2d"));
  const dimension = contexts.length;
  for (const context of contexts) {
    context.clearRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
    context.fillStyle = "red";
    context.beginPath();
    context.arc(CANVAS_SIZE / 2, CANVAS_SIZE / 2, 8, 0, 2 * Math.PI);
    context.fill();
    context.fillStyle = "white";
  }
  const planets = planetarySystem.get_planets();
  const maxWieght = Math.max(...planets.map(planet => planet.get_weight()));
  for (const planet of planets) {
    const weightRate = planet.get_weight() / maxWieght;
    const scaledPosition = planet
      .get_position()
      .map(axisPosition =>
        Math.floor(
          (axisPosition + planetarySystemRadius) * CANVAS_SIZE / (2 * planetarySystemRadius)
        ));
    for (const [contextIdx, context] of contexts.entries()) {
      context.beginPath();
      context.arc(
        scaledPosition[(contextIdx + 1) % dimension],
        scaledPosition[(contextIdx + 2) % dimension],
        Math.round(8 * weightRate),
        0,
        2 * Math.PI
      );
      context.fill();
    }
  }
}

/**
 * @param {PlanetarySystemD} planetarySystem
 * @param {number} planetarySystemRadius 
 */
function tick(planetarySystem, planetarySystemRadius) {
  ++frameCount;
  const second = Math.floor(Date.now() / 1000);
  if (second > prevSecond) {
    fpsCounter.textContent = frameCount.toString();
    frameCount = 0;
    prevSecond = second;
  }

  planetarySystem.tick();
  displayPlanetarySystem(planetarySystem, planetarySystemRadius);
  requestAnimationFrame(() => tick(planetarySystem, planetarySystemRadius));
}

// @ts-ignore
await init();
/** @type {PlanetarySystemD} */
const planetarySystem = PlanetarySystem.new();
/** @type {Number} */
const planetarySystemRadius = get_planetary_system_radius();

requestAnimationFrame(() => tick(planetarySystem, planetarySystemRadius));
