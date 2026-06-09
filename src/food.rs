use macroquad::prelude::*;
use crate::player::*;
use ::rand::Rng;

pub struct Food {
    width: f32,
    height: f32,
    position: Vec<u32>,
    spawns: i32
}

impl Food {
    pub fn new(position: Vec<u32>) -> Self {
        Food {
            width: 25.0,
            height: 25.0,
            position: position.clone(),
            spawns: 0
        }
    }

    // Necessary accessors used in Driver
    pub fn get_position(&self) -> Vec<u32> {
        return self.position.clone();
    }

    pub fn get_spawns(&self) -> i32 {
        return self.spawns;
    }

    // Used to randomly update food position to a coordinate that is NOT on the snake's body
    pub fn change_position(&mut self, player: &Player) {
        let mut rng = ::rand::rng();

        let mut random_x = rng.random_range(0..20);
        let mut random_y = rng.random_range(0..20);

        while player.get_body().contains(&vec![random_x, random_y]) {
            random_x = rng.random_range(0..20);
            random_y = rng.random_range(0..20);
        }

        self.position = vec![random_x, random_y];
        self.spawns += 1;
    }

    pub fn draw_food(&self) {
        draw_rectangle (
            self.position[0] as f32 * self.width,
            self.position[1] as f32 * self.height,
            self.width,
            self.height,
            RED
        );
    }
}