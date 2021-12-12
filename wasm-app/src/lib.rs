use wasm_bindgen::prelude::*;
use std::collections::HashSet;

const DIMENSION: usize = 3;
const UNIT_TIME: f64 = 60.0 * 60.0 * 24.0; // 一日(秒)
const PLANETS_NUM: usize = 256;

// 惑星同士の当たり判定への補正
const PLANETS_COLLISION_COEFFICIENT: f64 = 256.0;

const PLANETARY_SYSTEM_RADIUS: f64 = 227920000.0; // 火星の公転半径(km)
const STAR_WEIGHT: f64 = 1.989E+30; // 太陽の質量(kg)
const STAR_RADIUS: f64 = 6.9551E+5; // 太陽の半径
const NEW_PLANET_LARGEST_RADIUS: f64 = 2439.7; // 水星の半径(km)
const MIN_PLANET_DENSITY: f64 = 687.0E+9 / 2.0; // 土星の密度の半分(kg/km3)
const MAX_PLANET_DENSITY: f64 = 2.0 * 5.51E+12; // 地球の密度の倍(kg/km3)
const MAX_PLANET_AXIS_SPEED: f64 = 64.93 * 1.0; // 水星の公転速度(km/s)
const GRAVITY_CONSTANT: f64 = 6.6743015E-20; // 万有引力定数(km3/s2kg)

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_planetary_system_radius() -> f64 {
  PLANETARY_SYSTEM_RADIUS
}

pub fn measure_distance(a: &[f64; DIMENSION], b: &[f64; DIMENSION]) -> f64 {
  let mut distance = 0f64;
  for i in 0..DIMENSION {
    distance += (b[i] - a[i]).powi(2);
  }
  distance.sqrt()
}

pub fn create_random_vector(axis_min: f64, axis_max: f64) -> [f64; DIMENSION] {
  let diff = axis_max - axis_min;
  (0..DIMENSION)
    .map(|_| diff * js_sys::Math::random() + axis_min)
    .collect::<Vec<f64>>()
    .try_into()
    .unwrap()
}

#[wasm_bindgen]
#[derive (Clone)]
struct Planet {
  position: [f64; DIMENSION],
  speed: [f64; DIMENSION],
  radius: f64,
  weight: f64,
}

impl Planet {
  pub fn new() -> Planet {
    let radius = NEW_PLANET_LARGEST_RADIUS * js_sys::Math::random();
    let density = MIN_PLANET_DENSITY
      + (MAX_PLANET_DENSITY - MIN_PLANET_DENSITY)
      * js_sys::Math::random();
    let weight = 4.0 / 3.0 * std::f64::consts::PI * radius.powi(3) * density;
    Planet {
      position: create_random_vector(-PLANETARY_SYSTEM_RADIUS, PLANETARY_SYSTEM_RADIUS),
      speed: create_random_vector(-MAX_PLANET_AXIS_SPEED, MAX_PLANET_AXIS_SPEED),
      radius: radius,
      weight: weight,
    }
  }
}

#[wasm_bindgen]
impl Planet {
  pub fn get_position(&self) -> js_sys::Float64Array {
    js_sys::Float64Array::from(&self.position[..])
  }

  pub fn get_weight(&self) -> f64 {
    self.weight
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
      for i in 0..DIMENSION {
        planet.position[i] += UNIT_TIME * planet.speed[i];
      }
    }

    // 恒星(原点)からの重力加速度を加算する
    let mut planets_accelerations: Vec<[f64; 3]> = Vec::new();
    let mut collided_planets_indexes: HashSet<usize> = HashSet::new();
    for planet in &self.planets {
      let distance_from_star = measure_distance(&[0f64; DIMENSION], &planet.position);
      let gravity_accelaration = GRAVITY_CONSTANT * STAR_WEIGHT / distance_from_star.powi(2);
      let mut accelaration = [0f64; DIMENSION];
      for dimension_i in 0..DIMENSION {
        // 重力加速度を分解するために恒星からの距離に対する各軸の距離を掛ける
        accelaration[dimension_i] =
          - gravity_accelaration * planet.position[dimension_i] / distance_from_star;
      }
      planets_accelerations.push(accelaration);
    }

    // 各惑星間の重力加速度を加算する
    for planet_i in 0..(planets_accelerations.len() - 1) {
      for planet_j in (planet_i + 1)..planets_accelerations.len() {
        let distance = measure_distance(
          &self.planets[planet_i].position,
          &self.planets[planet_j].position
        );

        let i_gravity_accelaration =
          GRAVITY_CONSTANT * self.planets[planet_j].weight / distance.powi(2);
        for dimension_i in 0..DIMENSION {
          // 重力加速度を分解するために惑星同士の距離に対する各軸に対する各軸の距離を掛ける
          let axis_distance = self.planets[planet_j].position[dimension_i]
            - self.planets[planet_i].position[dimension_i];
          planets_accelerations[planet_i][dimension_i] += i_gravity_accelaration
            * axis_distance
            / distance;
        }

        let j_gravity_accelaration =
          GRAVITY_CONSTANT * self.planets[planet_i].weight / distance.powi(2);
        for dimension_i in 0..DIMENSION {
          let axis_distance = self.planets[planet_i].position[dimension_i]
            - self.planets[planet_j].position[dimension_i];
          planets_accelerations[planet_i][dimension_i] +=
            j_gravity_accelaration * (axis_distance) / distance;
        }
      }
    }

    // 速度の変更
    for planet_i in 0..(planets_accelerations.len()) {
      for dimension_i in 0..DIMENSION {
        self.planets[planet_i].speed[dimension_i] +=
          UNIT_TIME * planets_accelerations[planet_i][dimension_i];
      }
    }

    // 惑星の脱出と恒星への衝突
    for (planet_i, planet) in self.planets.iter().enumerate() {
      let distance = measure_distance(&[0f64; DIMENSION], &planet.position);
      if distance >= PLANETARY_SYSTEM_RADIUS * 3f64.sqrt() {
        collided_planets_indexes.insert(planet_i);
        log("PlanetEscaped");
      }
      if distance < STAR_RADIUS + planet.radius {
        collided_planets_indexes.insert(planet_i);
        log("PlanetColliedWithStar");
      }
    }

    // 惑星同士の衝突
    let prev_planets_len = self.planets.len();
    for planet_i in 0..(prev_planets_len - 1) {
      for planet_j in (planet_i + 1)..prev_planets_len {
        let distance = measure_distance(&self.planets[planet_i].position, &self.planets[planet_j].position);
        if distance < PLANETS_COLLISION_COEFFICIENT * (self.planets[planet_i].radius + self.planets[planet_j].radius) {
          collided_planets_indexes.insert(planet_i);
          collided_planets_indexes.insert(planet_j);

          let mut new_position = [0f64; DIMENSION];
          for dimension_i in 0..DIMENSION {
            new_position[dimension_i] =
              (self.planets[planet_i].position[dimension_i]
                + self.planets[planet_j].position[dimension_i])
                / 2.0;
          }
          let new_weight = self.planets[planet_i].weight + self.planets[planet_j].weight;
          let mut new_speed = [0f64; DIMENSION];
          for dimension_i in 0..DIMENSION {
            let impulse =
              self.planets[planet_i].weight * self.planets[planet_i].speed[dimension_i]
                + self.planets[planet_j].weight * self.planets[planet_j].speed[dimension_i];
            new_speed[dimension_i] = impulse / new_weight;
          }
          let new_radius =
            (self.planets[planet_i].radius.powi(3) + self.planets[planet_j].radius.powi(3)).powf(1f64 / 3f64);
          
          self.planets.push(Planet {
            position: new_position,
            weight: new_weight,
            speed: new_speed,
            radius: new_radius,
          });

          log("PlanetsCollied");
        }
      }
    }

    // 惑星の削除
    for planet_i in (0..prev_planets_len).rev() {
      if collided_planets_indexes.contains(&planet_i) {
        self.planets.remove(planet_i);
      }
    }
  }

  pub fn get_planets(&self) -> js_sys::Array {
    let planets = self.planets.clone();
    planets.into_iter().map(JsValue::from).collect()
  }
}
