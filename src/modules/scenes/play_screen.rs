use maat_graphics::math;
use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::{LoadScreen};
use crate::cgmath::{Vector2, Vector3, Vector4};

use crate::modules::collisions;

use crate::rand::Rng;

use rand::prelude::ThreadRng;
use rand::thread_rng;

use crate::modules::Boid;

pub struct PlayScreen {
  data: SceneData,
  rng: ThreadRng,
  boids: Vec<Boid>,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>, terrain_data: Vec<(String, Vec<Vec<f32>>)>) -> PlayScreen {
    let mut rng = thread_rng();
    
    let mut boids = Vec::new();
    
    let amount = 650;//(rng.gen::<f32>() * 100.0 + 20.0).floor() as usize;
    for _ in 0..amount {
      boids.push(Boid::new(&mut rng, window_size));
    }
    
    PlayScreen {
      data: SceneData::new(window_size, model_sizes, terrain_data),
      rng,
      boids,
    }
  }
}

impl Scene for PlayScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<dyn Scene> {
    let dim = self.data().window_dim;
    Box::new(PlayScreen::new(dim, self.data.model_sizes.clone(), self.data.terrain_data.clone()))
  }
  
  fn update(&mut self, delta_time: f32) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    collisions::boid_collision(&mut self.boids, delta_time);
    
    for boid in &mut self.boids {
      boid.update(dim, delta_time);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(
        DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*5.0, height*5.0),
                                Vector4::new(0.2, 0.2, 0.2, 1.0),
                                0.0)
    );
    
    for boid in &self.boids {
      boid.draw(draw_calls);
    }
    
    draw_calls.push(DrawCall::draw_instanced("boid".to_string()));
  }
}
