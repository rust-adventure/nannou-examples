use std::time::Instant;

use nannou::glam::Vec2;
use nannou::noise::NoiseFn;
use nannou::{noise::Perlin, prelude::*};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: Perlin,
    points: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 256)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model {
        noise: Perlin::new(),
        points: vec![],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window_rect();

    let tick = app.time;
    let y = model.noise.get([tick.into(), 0.]);
    let mapped_y = map_range(
        y,
        -1.0,
        1.0,
        win_rect.top(),
        win_rect.bottom(),
    );
    model.points.push(pt2(tick, mapped_y));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win_rect = app.window_rect();
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(rgb(0.439, 0.039, 0.467));

    let win_p = win_rect.pad(25.0);

    draw.polyline()
        .x(0 as f32 - model.points.len() as f32)
        .weight(1.0)
        .points(
            model
                .points
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let mut v2 = v.clone();
                    v2.x = i as f32;
                    v2
                })
                .collect::<Vec<Vec2>>(),
        )
        .color(rgb(0.855, 0.31, 0.671));

    draw.ellipse()
        .x(0.)
        .y(model.points.iter().last().unwrap().y)
        .w_h(50.0, 50.0)
        .color(rgb(0.855, 0.31, 0.671));

    draw.text(&format!(
        "x: {}\ny: {}",
        app.time,
        model.points.iter().last().unwrap().y
    ))
    .font_size(24)
    .wh(win_rect.wh())
    .left_justify()
    .align_text_bottom()
    .color(rgb(0.855, 0.31, 0.671));

    draw.text(&format!("FPS: {}", app.fps().floor(),))
        .font_size(48)
        .wh(win_rect.wh())
        .right_justify()
        .align_text_bottom()
        .color(rgb(0.855, 0.31, 0.671));

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    let now = chrono::offset::Local::now();

    if key == Key::S {
        app.main_window().capture_frame(format!(
            "{}{}{}",
            app.exe_name().unwrap(),
            now,
            ".png"
        ));
    }
}

fn mouse_pressed(
    _app: &App,
    model: &mut Model,
    _button: MouseButton,
) {
    model.noise = Perlin::new();
}
