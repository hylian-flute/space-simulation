  import init, { PlanetarySystem } from "./pkg/wasm_app.js";
  await init();
  const planetarySystem = PlanetarySystem.new();
  console.log(planetarySystem);
  console.log(planetarySystem.tick());
  console.log(planetarySystem.get_planets_positions());
  console.log(planetarySystem.get_planets_num());
