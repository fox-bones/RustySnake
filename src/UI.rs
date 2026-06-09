use macroquad::prelude::*;

// Center text helper
fn draw_centered_text(text: &str, y:f32, size:f32, color: Color) {
    let dimensions = measure_text(text, None, size as u16, 1.0);

    draw_text(
        text, 
        screen_width() / 2.0 - dimensions.width / 2.0, 
        y, 
        size, 
        color
    );
}

pub fn draw_start_menu() {
    draw_centered_text("Rusty Snake", 180.0, 48.0, GREEN);
    draw_centered_text("Press Enter to Start", 260.0, 28.0, WHITE);
}

pub fn draw_pause_menu() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    draw_centered_text("Paused", 210.0, 48.0, YELLOW);
    draw_centered_text("Esc - Resume", 270.0, 28.0, WHITE);
    draw_centered_text("Q - Main Menu", 310.0, 28.0, WHITE);
}

pub fn draw_death_menu(score: i32) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    let score = format!("Score: {score}");

    draw_centered_text("Game Over", 180.0, 48.0, RED);
    draw_centered_text(&score, 240.0, 28.0, WHITE);
    draw_centered_text("Enter - Restart", 270.0, 28.0, WHITE);
    draw_centered_text("Q - Main Menu", 310.0, 28.0, WHITE);
}