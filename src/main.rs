use macroquad::prelude::*;

const MAX_STEPS: usize = 7;
const POINT_RADIUS: f32 = 4.0;
const PICK_RADIUS: f32 = 12.0;
const STEP_DURATION: f32 = 0.7;

fn chaikin_step(points: &[Vec2]) -> Vec<Vec2> {
    let n = points.len();
    if n < 3 {
        return points.to_vec();
    }

    let mut out = Vec::with_capacity(n * 2);
    out.push(points[0]);

    for i in 0..(n - 1) {
        let p0 = points[i];
        let p1 = points[i + 1];
        let q = p0 * 0.75 + p1 * 0.25;
        let r = p0 * 0.25 + p1 * 0.75;

        out.push(q);
        out.push(r);
    }

    out.push(points[n - 1]);
    out
}

fn build_steps(control: &[Vec2]) -> Vec<Vec<Vec2>> {
    let mut steps = Vec::with_capacity(MAX_STEPS + 1);
    steps.push(control.to_vec());

    let mut cur = control.to_vec();
    for _ in 0..MAX_STEPS {
        cur = chaikin_step(&cur);
        steps.push(cur.clone());
    }

    steps
}

fn draw_points(points: &[Vec2]) {
    for &p in points {
        draw_circle(p.x, p.y, POINT_RADIUS, YELLOW);
        draw_circle_lines(p.x, p.y, POINT_RADIUS + 2.0, 2.0, ORANGE);
    }
}

fn draw_polyline(points: &[Vec2]) {
    if points.len() < 2 {
        return;
    }
    for i in 0..(points.len() - 1) {
        let a = points[i];
        let b = points[i + 1];
        draw_line(a.x, a.y, b.x, b.y, 2.5, SKYBLUE);
    }
}

fn closest_point_index(points: &[Vec2], mouse: Vec2, max_dist: f32) -> Option<usize> {
    let mut best_i: Option<usize> = None;
    let mut best_d2 = max_dist * max_dist;

    for (i, &p) in points.iter().enumerate() {
        let d2 = (p - mouse).length_squared();
        if d2 <= best_d2 {
            best_d2 = d2;
            best_i = Some(i);
        }
    }
    best_i
}

#[macroquad::main("Chaikin Step Animation")]
async fn main() {
    let mut control_points: Vec<Vec2> = Vec::new();
    let mut dragging: Option<usize> = None;

    let mut anim_running = false;
    let mut anim_timer = 0.0_f32;
    let mut anim_step_index = 1_usize;

    loop {
        let dt = get_frame_time();
        let mouse = vec2(mouse_position().0, mouse_position().1);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            control_points.clear();
            dragging = None;
            anim_running = false;
            anim_timer = 0.0;
            anim_step_index = 1;
        }

        if is_key_pressed(KeyCode::Enter) {
            if control_points.len() >= 1 {
                anim_running = control_points.len() >= 3;
                anim_timer = 0.0;
                anim_step_index = 1;
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(i) = closest_point_index(&control_points, mouse, PICK_RADIUS) {
                dragging = Some(i);
            } else {
                if !anim_running {
                    control_points.push(mouse);
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(i) = dragging {
                if i < control_points.len() {
                    control_points[i] = mouse;
                } else {
                    dragging = None;
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging = None;
        }

        let steps = build_steps(&control_points);

        if anim_running {
            anim_timer += dt;

            if anim_timer >= STEP_DURATION {
                anim_timer = 0.0;
                anim_step_index += 1;
                if anim_step_index > MAX_STEPS {
                    anim_step_index = 1;
                }
            }
        }

        clear_background(BLACK);
        draw_text("Left Click: add point | Drag: move point", 20.0, 30.0, 24.0, GREEN);
        draw_text("Enter: start (7 steps loop) | C/Backspace: clear | Esc: quit", 20.0, 58.0, 22.0, GRAY);

        let status = if control_points.is_empty() {
            "Draw some control points..."
        } else if control_points.len() == 1 {
            "1 point: showing point only."
        } else if control_points.len() == 2 {
            "2 points: showing straight line."
        } else if anim_running {
            "Animating Chaikin (steps 1..7, looping)."
        } else {
            "Ready: press Enter to start Chaikin animation."
        };
        draw_text(status, 20.0, 90.0, 22.0, WHITE);

        if control_points.is_empty() {
        } else if control_points.len() == 1 {
            draw_points(&control_points);
        } else if control_points.len() == 2 {
            draw_polyline(&control_points);
            draw_points(&control_points);
        } else {
            let shown = if anim_running {
                &steps[anim_step_index]
            } else {
                &steps[0]
            };
            draw_polyline(shown);
            draw_points(&control_points);
            if anim_running {
                let txt = format!("Step: {}", anim_step_index);
                draw_text(&txt, 20.0, 120.0, 24.0, YELLOW);
            }
        }
        next_frame().await;
    }
}
