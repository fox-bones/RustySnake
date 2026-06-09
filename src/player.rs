use macroquad::prelude::*;
use crate::food::*;

pub struct Player {
    length: u32,

    width: f32,
    height: f32,
    speed: f32,
    queued_direction: u32, // These two exist to validate movement, and prevent backtracking over the snake body
    has_queued_turn: bool, // ----------------------------------------------------------------------------------

    position: Vec<u32>,
    direction: u32,

    body: Vec<Vec<u32>>
}

impl Player {
    pub fn new(length: u32, position: Vec<u32>) -> Self {
        Player {
            length,

            width: 25.0,
            height: 25.0,
            speed: 25.0,
            queued_direction: 4,
            has_queued_turn: false,

            position: position.clone(),
            direction: 4,

            body: vec![position]
        }
    }

    // Necessary accessor used in Driver
    pub fn get_body(&self) -> &Vec<Vec<u32>> {
        &self.body
    }

    /*
     * Keyboard input
     * If a queued position is present, ignore input command
     * Queued position is reset once the snake's positon is updated
     */
    pub fn keyboard_movement(&mut self) {
        if self.has_queued_turn {
            return;
        }

        if is_key_pressed(KeyCode::W) && self.direction != 1 {
            self.queued_direction = 0;
            self.has_queued_turn = true;
        }
        if is_key_pressed(KeyCode::S) && self.direction != 0 {
            self.queued_direction = 1;
            self.has_queued_turn = true;
        }
        if is_key_pressed(KeyCode::A) && self.direction != 3 {
            self.queued_direction = 2;
            self.has_queued_turn = true;
        }
        if is_key_pressed(KeyCode::D) && self.direction != 2 {
            self.queued_direction = 3;
            self.has_queued_turn = true;
        }
    }

    /*
     * update_position utilizes check_sub to protect against negative, non-real numbers, which would otherwise crash at runtime
     * Returns 1 on legal moves, then adds position to the body
     * Returns 0 on illegal moves, such as hitting the walls of the window 
     * Uses a queued turn system to prevent the player from reversing over the snake's body
     */
    pub fn update_position(&mut self) ->  i32 {
        self.direction = self.queued_direction;
        self.has_queued_turn = false;

        match self.direction {
            0 => {
                if let Some(y) = self.position[1].checked_sub(1) {
                    self.position[1] = y;
                    self.body.insert(0, self.position.clone());
                }
                else {
                    print!("Player hit the top edge of the board!");
                    return 0;
                }
            }
            1 => {
                if self.position[1] < 19 {
                    self.position[1] += 1;
                    self.body.insert(0, self.position.clone());
                }
                else {
                    print!("Player hit the bottom edge of the board!");
                    return 0;
                }

            }
            2 => {
                if let Some(x) = self.position[0].checked_sub(1) {
                    self.position[0] = x;
                    self.body.insert(0, self.position.clone());
                }
                else {
                    print!("Player hit the left edge of the board!");
                    return 0;
                }
            }
            3 => {
                if self.position[0] < 19 {
                    self.position[0] += 1;
                    self.body.insert(0, self.position.clone());
                }
                else {
                    print!("Player hit the right edge of the board!");
                    return 0;
                }
            }
            _ => { return 1 }

        }

        // Return 0 if the head is at the sname position as a body segment
        if self.check_for_body() {
            println!("Player hit their own body!");
            return 0;
        }

        while self.body.len() > self.length as usize {
            self.body.pop();
        }

        1
    }

    /*
     * Iterate through every element in the snake's body, excluding the head 
     * Measure each iteration of the body against the snake head's current position
     */ 
    fn check_for_body(&self) -> bool {
        for chunk in self.body.iter().skip(1) {
            if chunk == &self.position {
                return true
            }
        }
        false
    }

    /* 
     * Adding + 1 to length when player position is equal to the coordinates of food
     * Resetting food coordinates to another random position
     */
    fn eat_food(&mut self, food: &mut Food) {
        if self.position == food.get_position() {
            self.length += 1;
            food.change_position(self);
        }
    }

    // This is redundant
    pub fn check_for_food(&mut self, food: &mut Food) {
        if self.position == food.get_position() {
            self.eat_food(food);
        }
    }


    // Drawing snake head and body
    pub fn draw(&mut self) {
    
        // head
        draw_rectangle(
            self.position[0] as f32 * self.width,
            self.position[1] as f32 * self.height,
            self.width,
            self.height,
            GREEN
        );

        // body
        for chunk in &self.body {
            draw_rectangle(
                chunk[0] as f32 * self.width,
                chunk[1] as f32 * self.height,
                self.width,
                self.height,
                GREEN
            )
        }
    }
}