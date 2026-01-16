use macroquad::{audio, prelude::*};
mod chai_algo;
use chai_algo::chaikin_algo;
#[macroquad::main("CHAIKAIN CANVAS")]
async fn main() {
    set_pc_assets_folder("audios");

    let sound1 = audio::load_sound("mrhba.wav").await.unwrap();
    let sound2 = audio::load_sound("hhh.wav").await.unwrap();
    let mut chaikin_steps: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut last_update = get_time();
    let mut current_step: usize = 0;
    let mut is_animating = false;
    let mut show_line = false;

    let button_rect = Rect::new(670.0, 10.0, 120.0, 30.0);
    loop {
        clear_background(BLACK);
        draw_text(
            "# to start chaikin Click on Enter ",
            20.0,
            20.0,
            20.0,
            GREEN,
        );
        draw_text("# to quit click on Escape", 20.0, 80.0, 20.0, RED);
        draw_text(
            "# to draw a point click left on your mouse",
            20.0,
            40.0,
            20.0,
            GREEN,
        );
        draw_text(
            "# you need 2 points to draw a line and 3 points to start the chaikin algo",
            20.0,
            60.0,
            20.0,
            GREEN,
        );
        draw_rectangle(670.0, 10.0, 120.0, 30.0, GREEN);
        draw_text("Merhba", 690.0, 30.0, 30.0, WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            if button_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                audio::play_sound_once(&sound1);
            } else {
                points.push((mouse_x, mouse_y));
                audio::play_sound_once(&sound2);
            }
        }
        for (x, y) in points.clone() {
            draw_circle(x, y, 3.0, GRAY);
            draw_circle(x, y, 1.0, BLACK);
        }
        if is_key_pressed(KeyCode::Enter) {
            if points.len() >= 3 {
                chaikin_steps.clear();
                chaikin_steps.push(points.clone());

                for i in 0..7 {
                    let next = chaikin_algo(&chaikin_steps[i]);
                    chaikin_steps.push(next);
                }

                current_step = 0;
                is_animating = true;
                show_line = true;
            }
        }

        if is_animating {
            if get_time() - last_update > 0.7 {
                current_step += 1;
                if current_step > 7 {
                    current_step = 0;
                }
                last_update = get_time();
            }
        }

        if show_line {
            if is_animating && points.len() >= 3 {
                draw_polyline(&chaikin_steps[current_step], 2.0, GREEN);
            } else if points.len() == 2 {
                draw_line(
                    points[0].0,
                    points[0].1,
                    points[1].0,
                    points[1].1,
                    2.0,
                    GREEN,
                );
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

fn draw_polyline(points: &Vec<(f32, f32)>, thickness: f32, color: Color) {
    for i in 0..points.len() - 1 {
        draw_line(
            points[i].0,
            points[i].1,
            points[i + 1].0,
            points[i + 1].1,
            thickness,
            color,
        );
    }
}
