/*
 * This is a game of snake. It is a simple game, where the player seeks out apples, which in turn, increase the length
 * of the player's snake. If the player overcomes the bounds of the game board, or collides with their own body, 
 * the game results in a loss. Score is measured by the length of the player's snake at the time of their death.
 * 
 * Press Escape to pause the game at any time.
 * 
 * Original Author: Cayce Harwood
 * Creation date: 06/09/2026
*/

mod player;
mod food;
mod UI;

use macroquad::prelude::*;
use crate::player::*;
use crate::food::*;
use crate::UI::*;
use ::rand::Rng;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Snake".to_owned(),
        window_width: 500,
        window_height: 500,
        window_resizable: false, 
        ..Default::default()
    }
}

// Game states to handle UI overlays
enum GameState {
    StartMenu,
    Playing,
    Paused,
    Dead,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::StartMenu;

    // Used to restrict the snake's 
    let mut move_timer = 0.0;
    let move_interval = 0.12;

    let mut rng = ::rand::rng();

    let mut player = Player::new (
        1,   
        vec![10, 10]
    );

    let mut food = Food::new (
        vec![12, 12]
    );

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::StartMenu => {
                UI::draw_start_menu();

                if is_key_pressed(KeyCode::Enter) {
                    let x = rng.random_range(2..18);
                    let y = rng.random_range(2..18);
                    player = Player::new(1, vec![x, y]);

                    food = Food::new(vec![12, 12]);
                    food.change_position(&player);

                    move_timer = 0.0;
                    game_state = GameState::Playing;
                }
            }

            // Primary game logic lives here
            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }
                
                player.keyboard_movement();

                // Making sure input commands are still heard regardless of frame restrictions
                move_timer += get_frame_time();

                if move_timer >= move_interval {
                    if player.update_position() == 0 {
                        game_state = GameState::Dead;
                    }
                    move_timer = 0.0;
                }

                player.check_for_food(&mut food);
                player.draw();
                food.draw_food();

                if is_key_pressed(KeyCode::Q) {
                    game_state = GameState::StartMenu;
                }

            }

            GameState::Paused => {
                player.draw();
                food.draw_food();

                UI::draw_pause_menu();

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::Q) {
                    game_state = GameState::StartMenu;
                }
            }

            GameState::Dead => {
                player.draw();
                food.draw_food();

                UI::draw_death_menu(food.get_spawns());

                if is_key_pressed(KeyCode::Enter) {
                    let x = rng.random_range(2..18);
                    let y = rng.random_range(2..18);
                    player = Player::new(1, vec![x, y]);

                    food = Food::new(vec![12, 12]);
                    food.change_position(&player);

                    move_timer = 0.0;
                    game_state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::Q) {
                    game_state = GameState::StartMenu;
                }
            }
        }

        next_frame().await;
    }
}
