mod utils;

use std::f32::consts::PI;
use std::collections::HashMap;
use bracket_noise::prelude::*;
use bracket_random::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use cgmath::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct NoiseModel {
    noise: FastNoise,
    pub frequency: f32,
    pub lacunarity: f32,
    pub gain: f32,
    pub octaves: i32,
}

#[wasm_bindgen]
impl NoiseModel {
    pub fn new(frequency: f32, lacunarity: f32, gain: f32, octaves: i32) -> NoiseModel {
        let mut rng = RandomNumberGenerator::new();
        let mut noise = FastNoise::seeded(rng.next_u64());
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(octaves);
        noise.set_fractal_lacunarity(lacunarity);
        noise.set_fractal_gain(gain);
        noise.set_frequency(frequency);

        NoiseModel { noise, frequency, gain, octaves, lacunarity }
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct FlowModel {
    pub vector_size: u32,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub field_width: u32,
    pub field_height: u32,
    noise_model: NoiseModel,
}

#[wasm_bindgen]
impl FlowModel {
    pub fn new(noise_model: NoiseModel, vector_size: u32, canvas_width: u32, canvas_height: u32) -> FlowModel {
        let field_width = canvas_width / vector_size;
        let field_height = canvas_height / vector_size;

        FlowModel {
            vector_size,
            canvas_width,
            canvas_height,
            field_width,
            field_height,
            noise_model,
        }
    }

    fn compute_field(&self) -> HashMap<(u32, u32), Vector2<f32>> {
        let mut vector_field = HashMap::new();

        for x in 0..self.field_width {
            for y in 0..self.field_height {
                let raw_val = self.noise_model.noise.get_noise(
                    (x as f32) / ((self.field_width as f32)),
                    (y as f32) / ((self.field_height as f32)),
                );

                let normalized_val = (raw_val + 1.0) * PI / 2.0;
                let (vec_x, vec_y) = Rad(normalized_val).sin_cos();
                let vector = Vector2::new(vec_x + x as f32, vec_y + y as f32);

                vector_field.insert((x, y), vector);
            }
        }

        vector_field
    }
}

#[wasm_bindgen]
pub struct FlowView {
    ctx: CanvasRenderingContext2d,
    flow_model: FlowModel
}

#[wasm_bindgen]
impl FlowView {
    pub fn new(ctx: CanvasRenderingContext2d, flow_model: FlowModel) -> FlowView {
        FlowView {
            ctx,
            flow_model
        }
    }

    pub fn render_vector_field(&self) {
        let vector_field = self.flow_model.compute_field();

        for x in 0..self.flow_model.field_width {
            for y in 0..self.flow_model.field_height {
                let vector = vector_field[&(x, y)];

                let canvas_x = x * self.flow_model.vector_size;
                let canvas_y = y * self.flow_model.vector_size;

                let vector_x = vector.x * self.flow_model.vector_size as f32;
                let vector_y = vector.y * self.flow_model.vector_size as f32;

                self.ctx.begin_path();
                self.ctx.move_to(canvas_x as f64, canvas_y as f64);
                self.ctx.line_to(vector_x as f64, vector_y as f64);
                self.ctx.stroke();
            }
        }
    }

    pub fn render_height_map(&self) {
        let canvas_width = self.flow_model.canvas_width.clone();
        let canvas_height = self.flow_model.canvas_height.clone();

        for x in 0..canvas_width {
            for y in 0..canvas_height {
                let raw_val = self.flow_model.noise_model.noise.get_noise(
                    (x as f32) / ((canvas_width as f32)),
                    (y as f32) / ((canvas_height as f32)),
                );
                let color = JsValue::from(format!("rgba(0, 0, 0, {})", (raw_val / 2.0) + 0.5));

                self.ctx.set_fill_style(&color);
                self.ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}