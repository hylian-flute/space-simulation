use wasm_bindgen::prelude::*;

const PLANETARY_SYSTEM_RADIUS: f64 = 4495060000.0; // 海王星の公転半径(km)
const I16_COEFFICIENT: f64 = 32767.0 / PLANETARY_SYSTEM_RADIUS;
const STAR_RADIUS: f64 = 696000.0; // 太陽の半径(km)
const NEW_PLANET_LARGEST_RADIUS: f64 = 2439.7; // 水星の半径(km)
const MIN_PLANET_DENSITY: f64 = 687000000000.0 / 2.0; // 土星の密度の半分(kg/km3)
const MAX_PLANET_DENSITY: f64 = 2.0 * 5510000000000.0; // 地球の密度の倍(kg/km3)
const TARGET_PLANETS_WEIGHT: f64 = 2589065785521177500000000000.0 / 256.0; // 太陽系の太陽以外の質量の1/256(kg)

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Planet {
    position: Vec3,
    radius: f64,
    weight: f64,
}

impl Planet {
    pub fn new() -> Planet {
        let radius = NEW_PLANET_LARGEST_RADIUS * js_sys::Math::random();
        let density = MIN_PLANET_DENSITY + (MAX_PLANET_DENSITY - MIN_PLANET_DENSITY) * js_sys::Math::random();
        let weight = 4.0 / 3.0 * std::f64::consts::PI * radius.powi(3) * density;
        Planet {
            position: Vec3 {
                x: PLANETARY_SYSTEM_RADIUS * (2.0 * js_sys::Math::random() - 1.0),
                y: PLANETARY_SYSTEM_RADIUS * (2.0 * js_sys::Math::random() - 1.0),
                z: PLANETARY_SYSTEM_RADIUS * (2.0 * js_sys::Math::random() - 1.0),
            },
            radius: radius,
            weight: weight,
        }
    }
}

#[wasm_bindgen]
pub struct PlanetarySystem {
    planets: Vec<Planet>,
}

#[wasm_bindgen]
impl PlanetarySystem {
    pub fn new() -> PlanetarySystem {
        PlanetarySystem {
            planets: Vec::new()
        }
    }
    pub fn tick(&mut self) {
        let mut planets_sum_weight = 0f64;
        for planet in &self.planets {
            planets_sum_weight += planet.weight;
        }
        while planets_sum_weight < TARGET_PLANETS_WEIGHT {
            self.planets.push(Planet::new());
            planets_sum_weight += self.planets.last().unwrap().weight;
        }
    }
    pub fn get_planets_positions(&self) -> js_sys::Int16Array {
        let mut vec = Vec::new();
        for planet in &self.planets {
            vec.push((I16_COEFFICIENT * planet.position.x) as i16);
            vec.push((I16_COEFFICIENT * planet.position.y) as i16);
            vec.push((I16_COEFFICIENT * planet.position.z) as i16);
        }
        js_sys::Int16Array::from(&vec[..])
    }
    pub fn get_planets_num(&self) -> usize {
        self.planets.len()
    }
}
