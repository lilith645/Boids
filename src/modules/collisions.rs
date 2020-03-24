use maat_graphics::math;
use maat_graphics::cgmath::{Vector2, Vector3};
use maat_graphics::cgmath::InnerSpace;
use maat_graphics::cgmath::Zero;

use crate::modules::Boid;

pub fn boid_collision(boids: &mut Vec<Boid>, delta_time: f32) {
  let clone_boids = boids.clone();
  
  for i in 0..boids.len() {
    boid_math(i, boids, &clone_boids);
  }
  /*
  for i in 0..boids.len() {
    let vision_boids = boids_in_vision(i, boids, &clone_boids);
    
    if vision_boids.len() == 0 {
      continue;
    }
    
    seperation(i, boids, &vision_boids, delta_time);
    alignment(i, boids, &vision_boids, delta_time);
    cohesion(i, boids, &vision_boids, delta_time);
  }*/
}

fn boid_math(i: usize, boids: &mut Vec<Boid>, clone_boids: &Vec<Boid>) {
  boids[i].mut_local_boids().num_boids = 0;
  boids[i].mut_local_boids().average_dir_heading = Vector2::zero();
  boids[i].mut_local_boids().center = Vector2::zero();
  boids[i].mut_local_boids().average_seperation_heading = Vector2::zero();
  
  for j in 0..clone_boids.len() {
    if i == j {
      continue;
    }
    
    let offset = clone_boids[j].pos() - boids[i].pos();
    let sqr_dist = offset.x * offset.x + offset.y * offset.y;
    
    if (sqr_dist < boids[i].vision_radius() * boids[i].vision_radius()) {
      boids[i].mut_local_boids().num_boids += 1;
      boids[i].mut_local_boids().average_dir_heading += clone_boids[j].direction();
      boids[i].mut_local_boids().center += clone_boids[j].pos();
      if sqr_dist < boids[i].avoid_radius() * boids[i].avoid_radius() {
        boids[i].mut_local_boids().average_seperation_heading -= offset / sqr_dist;
      }
    }
  }
}

fn boids_in_vision(i: usize, boids: &mut Vec<Boid>, clone_boids: &Vec<Boid>) -> Vec<Boid> {
  let mut vision_boids = Vec::new();
  
  let boid_vision_circle = boids[i].pos().extend(boids[i].vision_radius());
  
  for j in 0..clone_boids.len() {
    if i == j { // is same boid ignore self
      continue;
    }
    
    if math::is_point_inside_circle(clone_boids[j].pos(), boid_vision_circle) {
      vision_boids.push(clone_boids[j].clone());
    }
  }
  
  vision_boids
}

fn seperation(i: usize, boids: &mut Vec<Boid>, vision_boids: &Vec<Boid>, delta_time: f32) {
  let mut total_vector = Vector2::new(0.0, 0.0);
  
  for j in 0..vision_boids.len() {
    let vector = vision_boids[j].pos() - boids[i].pos();
    
    total_vector += math::normalise_vector2(vector);
  }
  
  total_vector = math::normalise_vector2(total_vector);
  
  total_vector *= -1.0;
  
  let player_vector = math::normalise_vector2(
                       Vector2::new(
                        1.0 * math::to_radians(boids[i].angle()+90.0).cos(),
                        1.0 * math::to_radians(boids[i].angle()+90.0).sin()
                      ));
  
  let angle = math::to_degrees(player_vector.angle(total_vector).0);
  
  if angle < 0.0 && angle > -90.0 {
    boids[i].turn_left(delta_time);
  } else if angle > 0.0 && angle < 90.0 {
    boids[i].turn_right(delta_time);
  }
}

fn alignment(i: usize, boids: &mut Vec<Boid>, vision_boids: &Vec<Boid>, delta_time: f32) {
  let mut average_angle = 0.0;
  
  for boid in vision_boids {
    average_angle += boid.angle();
  }
  
  average_angle /= vision_boids.len() as f32;
  
  if boids[i].angle() < average_angle {
    boids[i].turn_right(delta_time);
  } else {
    boids[i].turn_left(delta_time);
  }
}

fn cohesion(i: usize, boids: &mut Vec<Boid>, vision_boids: &Vec<Boid>, delta_time: f32) {
  let mut average_x = 0.0;
  let mut average_y = 0.0;
  
  for boid in vision_boids {
    average_x += boid.pos().x;
    average_y += boid.pos().y;
  }
  
  average_x /= vision_boids.len() as f32;
  average_y /= vision_boids.len() as f32;
  
  let unit_vector = math::normalise_vector2(boids[i].pos() - Vector2::new(average_x, average_y));
  
  let x_angle = (boids[i].angle()+90.0).cos(); // -1 to 1
  let y_angle = (boids[i].angle()+90.0).sin(); // -1 to 1
  
  let unit_angle = Vector2::new(x_angle, y_angle);
  
  let angle = unit_angle.angle(unit_vector);
  
  if math::to_degrees(angle.0) <= 0.0 {
    boids[i].turn_left(delta_time);
  } else {
    boids[i].turn_right(delta_time);
  }
}
