use wasm_bindgen::prelude::*;

const UNIT_TIME: f64 = 60.0 * 60.0 * 24.0; // 一日(秒)
const PLANETS_NUM: usize = 128;

const PLANETARY_SYSTEM_RADIUS: f64 = 4495060000.0; // 海王星の公転半径(km)
const I16_COEFFICIENT: f64 = 32767.0 / PLANETARY_SYSTEM_RADIUS;
const STAR_RADIUS: f64 = 696000.0; // 太陽の半径(km)
const STAR_WEIGHT: f64 = 1.989E+30; // 太陽の質量(kg)
const NEW_PLANET_LARGEST_RADIUS: f64 = 2439.7; // 水星の半径(km)
const MIN_PLANET_DENSITY: f64 = 687.0E+9 / 2.0; // 土星の密度の半分(kg/km3)
const MAX_PLANET_DENSITY: f64 = 2.0 * 5.51E+12; // 地球の密度の倍(kg/km3)
const MAX_PLANET_AXIS_SPEED: f64 = 64.93 * 16.0; // 水星の公転速度の16倍(km/s)
const GRAVITY_CONSTANT: f64 = 6.6743015E-20; // 万有引力定数(km3/s2kg)

// const TARGET_PLANETS_WEIGHT: f64 = 2589065785521177500000000000.0 / 256.0; // 太陽系の太陽以外の質量の1/256(kg)

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn distance(v1: &Vec3, v2: &Vec3) -> f64 {
        ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2) + (v2.y - v1.y).powi(2)).powf(0.5)
    }
}

struct Planet {
    position: Vec3,
    speed: Vec3,
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
            speed: Vec3 {
                x: MAX_PLANET_AXIS_SPEED * (2.0 * js_sys::Math::random() - 1.0),
                y: MAX_PLANET_AXIS_SPEED * (2.0 * js_sys::Math::random() - 1.0),
                z: MAX_PLANET_AXIS_SPEED * (2.0 * js_sys::Math::random() - 1.0),
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
        while self.planets.len() < PLANETS_NUM {
            self.planets.push(Planet::new());
        }

        for planet in &mut self.planets {
            planet.position.x += UNIT_TIME * planet.speed.x;
            planet.position.y += UNIT_TIME * planet.speed.y;
            planet.position.z += UNIT_TIME * planet.speed.z;
        }

        let mut planets_accelerations: Vec<Vec3> = Vec::new();
        for planet in &self.planets {
            let r = Vec3::distance(&Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }, &planet.position);
            let a = GRAVITY_CONSTANT * STAR_WEIGHT / r.powi(2);
            planets_accelerations.push(Vec3 {
                x: a * planet.position.x / r,
                y: a * planet.position.y / r,
                z: a * planet.position.z / r,
            });
        }

        for i in 0..(planets_accelerations.len() - 1) {
            for j in (i + 1)..planets_accelerations.len() {
                let r = Vec3::distance(&self.planets[i].position, &self.planets[j].position);

                let a_i = GRAVITY_CONSTANT * self.planets[j].weight / r.powi(2);
                planets_accelerations[i].x += a_i * (self.planets[j].position.x - self.planets[i].position.x) / r;
                planets_accelerations[i].y += a_i * (self.planets[j].position.y - self.planets[i].position.y) / r;
                planets_accelerations[i].z += a_i * (self.planets[j].position.z - self.planets[i].position.z) / r;

                let a_j = GRAVITY_CONSTANT * self.planets[i].weight / r.powi(2);
                planets_accelerations[i].x += a_i * (self.planets[i].position.x - self.planets[j].position.x) / r;
                planets_accelerations[i].y += a_i * (self.planets[i].position.y - self.planets[j].position.y) / r;
                planets_accelerations[i].z += a_i * (self.planets[i].position.z - self.planets[j].position.z) / r;
            }
        }

        for i in 0..(planets_accelerations.len()) {
            self.planets[i].speed.x += UNIT_TIME * planets_accelerations[i].x;
            self.planets[i].speed.y += UNIT_TIME * planets_accelerations[i].y;
            self.planets[i].speed.z += UNIT_TIME * planets_accelerations[i].z;
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
