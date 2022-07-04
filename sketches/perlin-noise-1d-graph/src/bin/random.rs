use names::Generator;
use nannou::{
    glam::Vec2,
    noise::{BasicMulti, NoiseFn, Perlin, Seedable},
    prelude::*,
};
use rand::{prelude::ThreadRng, Rng};

// interesting variables
const NOISE_STEP: f32 = 500.;
const NOISE_SCALE: f32 = 1.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: ThreadRng,
    points: Vec<Vec2>,
    frame_start: u64,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1200, 600)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();

    Model {
        noise: rng,
        points: vec![],
        frame_start: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window_rect();

    // advance by 1/500 per frame
    let current_step = (app.elapsed_frames()
        - model.frame_start) as f32
        / NOISE_STEP;
    let y = model.noise.gen_range(-1.0..1.0);
    let mapped_y = map_range(
        y,
        -1.0,
        1.0,
        win_rect.top(),
        win_rect.bottom(),
    );
    model.points.push(pt2(current_step, mapped_y));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let background = rgb(0.439, 0.039, 0.467);
    let foreground = rgb(0.855, 0.31, 0.671);

    // set up containing rectangles
    let win_rect = app.window_rect();
    let win_p = win_rect.pad(25.0);

    let draw = app.draw();
    draw.background().color(background);

    // x=0 line
    draw.line()
        .start(Vec2::new(win_rect.left(), 0.))
        .end(Vec2::new(win_rect.right(), 0.))
        .color(rgb(0.255, 0.02, 0.275));

    // noise line
    draw.polyline()
        .x(0 as f32 - model.points.len() as f32)
        .weight(1.0)
        .points(
            model
                .points
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, mut point)| {
                    point.x = index as f32;
                    point
                })
                .collect::<Vec<Vec2>>(),
        )
        .color(foreground);

    // current noise dot as ellipse
    draw.ellipse()
        .x(0.)
        .y(model.points.iter().last().unwrap().y)
        .w_h(10.0, 10.0)
        .color(foreground);

    for i in 0..(app.elapsed_frames() / 500) {
        draw.line()
            .start(Vec2::new(
                (i * 500 - model.points.len() as u64)
                    as f32,
                win_rect.top(),
            ))
            .end(Vec2::new(
                (i * 500 - model.points.len() as u64)
                    as f32,
                win_rect.bottom(),
            ));
    }
    // x = noise input
    // y = value
    // draw.text(&format!(
    //     "x: {}\ny: {}",
    //     app.elapsed_frames() as f32 / 500.0,
    //     model.points.iter().last().unwrap().y
    // ))
    // .font_size(24)
    // .wh(win_rect.wh())
    // .left_justify()
    // .align_text_bottom()
    // .color(foreground);

    // display noise seed

    draw.text(&format!("{}", "random"))
        .font_size(48)
        .wh(win_p.wh())
        .right_justify()
        .align_text_bottom()
        .color(foreground);

    // draw to frame
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
    app: &App,
    model: &mut Model,
    _button: MouseButton,
) {
    let mut rng = rand::thread_rng();

    // set relevant values on model,
    // clearing any old data so that data
    // from previous seeds doesn't render
    model.noise = rng;
    model.frame_start = app.elapsed_frames();
    model.points = vec![];
}
