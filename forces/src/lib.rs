use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};
use js_sys::{Uint32Array};

const PARTICLES: u32 = 10;
const PARTICLE_SIZE: u32 = 20;

struct Particle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Particle {
    fn new(x: u32, y: u32) -> Particle {
        Particle { x, y, width: PARTICLE_SIZE, height: PARTICLE_SIZE }
    }
}

#[wasm_bindgen]
pub fn get_system() -> Uint32Array {
    let mut particles = Vec::new();

    for n in 0..PARTICLES {
        let particle = Particle::new(n * PARTICLE_SIZE, n * PARTICLE_SIZE);
        particles.push(particle.x);
        particles.push(particle.y);
        particles.push(particle.width);
        particles.push(particle.height);
    }

    Uint32Array::from(particles.as_slice())
}
    

#[wasm_bindgen]
pub fn get_coordinates(_: &CanvasRenderingContext2d) -> Uint32Array {

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    Uint32Array::from(&vec[..])
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(ctx: &CanvasRenderingContext2d, msg: &str);
}
