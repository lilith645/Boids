use maat_graphics::cgmath::{Vector2, Vector3};
use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::rand::Rng;

use crate::maat_graphics::cgmath::InnerSpace;

use rand::prelude::ThreadRng;
use rand::thread_rng;

const MIN_SPEED: f32 = 50.0;
const MAX_SPEED: f32 = 450.0;

const ALIGNMENT_WEIGHT: f32 = 1.0;
const SEPERATION_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 0.5;

#[derive(Clone)]
pub struct BoidDetails {
  pub average_seperation_heading: Vector2<f32>,
  pub average_dir_heading: Vector2<f32>,
  pub center: Vector2<f32>,
  pub num_boids: u32,
}

impl BoidDetails {
  pub fn new() -> BoidDetails {
    BoidDetails {
      average_seperation_heading: Vector2::new(0.0, 0.0),
      average_dir_heading: Vector2::new(0.0, 0.0),
      center: Vector2::new(0.0, 0.0),
      num_boids: 0,
    }
  }
  
  pub fn len(&self) -> u32 {
    self.num_boids
  }
  
  pub fn cneter(&self) -> Vector2<f32> {
    self.center
  }
  
  pub fn seperation(&self) -> Vector2<f32> {
    self.average_seperation_heading
  }
  
  pub fn average_dir_heading(&self) -> Vector2<f32> {
    self.average_dir_heading
  }
}

#[derive(Clone)]
pub struct Boid {
  pos: Vector2<f32>,
  size: Vector2<f32>,
  
  texture: String,
  
  angle: f32,
  
  vision_radius: f32,
  avoid_radius: f32,
  
  turn_rate: f32,
  
  forward: Vector2<f32>,
  velocity: Vector2<f32>,
  
  local_boids: BoidDetails,
}

impl Boid {
  pub fn new(rng: &mut ThreadRng, window_size: Vector2<f32>) -> Boid {
    let angle = rng.gen::<f32>() * 360.0;
    let x = rng.gen::<f32>() * window_size.x;
    let y = rng.gen::<f32>() * window_size.y;
    
    let forward = Vector2::new(math::to_radians(angle+90.0).cos(), 
                                  math::to_radians(angle+90.0).sin());
    
    let velocity = forward * ((MIN_SPEED + MAX_SPEED)*0.5);
    
    let vision_radius = 24.0;
    let avoid_radius = vision_radius*0.7;
    
    Boid {
      pos: Vector2::new(x, y),
      size: Vector2::new(32.0, 48.0)*0.5,
      texture: "boid".to_string(),
      angle,
      
      vision_radius,
      avoid_radius,
      turn_rate: 60.0,
      
      forward,
      velocity,
      local_boids: BoidDetails::new(),
    }
  }
  
  pub fn local_boids(&mut self) -> &BoidDetails {
    &self.local_boids
  }
  
  pub fn mut_local_boids(&mut self) -> &mut BoidDetails {
    &mut self.local_boids
  }
  
  pub fn pos(&self) -> Vector2<f32> {
    self.pos
  }
  
  pub fn direction(&self) -> Vector2<f32> {
    self.forward
  }
  
  pub fn vision_radius(&self) -> f32 {
    self.vision_radius
  }
  
  pub fn avoid_radius(&self) -> f32 {
    self.avoid_radius
  }
  
  pub fn angle(&self) -> f32 {
    self.angle
  }
  
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn turn_left(&mut self, delta_time: f32) {
    self.angle -= self.turn_rate*delta_time;
  }
  
  pub fn turn_right(&mut self, delta_time: f32) {
    self.angle += self.turn_rate*delta_time;
  }
  
  pub fn update(&mut self, window_dim: Vector2<f32>, delta_time: f32) {
    let mut acceleration = Vector2::new(0.0, 0.0);
    
    if self.local_boids.len() > 0 {
      self.local_boids.center /= self.local_boids.len() as f32;
    //  self.local_boids.average_dir_heading /= self.local_boids.len() as f32;
      //self.local_boids.average_seperation_heading /= self.local_boids.len() as f32;
      
      let offset_to_center = (self.local_boids.center - self.pos);
      
      let alignment_force = math::normalise_vector2(self.local_boids.average_dir_heading()) * MAX_SPEED - self.velocity;
      let cohesion_force = math::normalise_vector2(offset_to_center) * MAX_SPEED - self.velocity;
      let seperation_force = math::normalise_vector2(self.local_boids.seperation()) * MAX_SPEED - self.velocity;
      
      acceleration += alignment_force * ALIGNMENT_WEIGHT;
      acceleration += cohesion_force * COHESION_WEIGHT;
      acceleration += seperation_force * SEPERATION_WEIGHT;
    }
    
    self.velocity += acceleration * delta_time;
    let mut speed = self.velocity.magnitude();
    let dir = self.velocity / speed;
    speed = (speed.min(MAX_SPEED)).max(MIN_SPEED);
    self.velocity = dir * speed;
    
    self.pos += self.velocity * delta_time;
    self.forward = dir;
    
    let angle_dir = Vector2::new(250.0*(math::to_radians(self.angle+90.0)).cos() * delta_time,
                                 250.0*(math::to_radians(self.angle+90.0)).sin() * delta_time);
    
    self.angle += math::to_degrees((angle_dir).angle(self.forward).0);
    
    if self.pos.x < 0.0 {
      self.pos.x = window_dim.x+self.pos.x;
    }
    if self.pos.x > window_dim.x {
      self.pos.x = self.pos.x-window_dim.x;
    }
    if self.pos.y < 0.0 {
      self.pos.y = window_dim.y+self.pos.y;
    }
    if self.pos.y > window_dim.y {
      self.pos.y = self.pos.y-window_dim.y;
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    /*draw_calls.push(DrawCall::add_instanced_sprite_sheet(self.pos,
                                                         self.size,
                                                         self.angle,
                                                         self.texture.to_string(),
                                                         Vector3::new(0, 0, 1)));*/
    draw_calls.push(DrawCall::draw_textured(self.pos,
                                                         self.size,
                                                         self.angle,
                                                         self.texture.to_string()));
  }
}


