use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let win_rect = app.window_rect();

    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    let win_p = win_rect.pad(25.0);
    draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.5));

    draw.ellipse().color(STEELBLUE);

    let rectangle =
        Rect::from_w_h(100.0, 100.0).top_left_of(win_p);

    let circle = Rect::from_w_h(200.0, 200.0)
        .top_left_of(win_p)
        .below(rectangle);
    draw.ellipse()
        .xy(circle.xy())
        .wh(circle.wh())
        .color(SALMON);

    draw.rect()
        .xy(rectangle.xy())
        .wh(rectangle.wh())
        .color(PLUM);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

// fn view(app: &App, _model: &Model, frame: Frame) {
//     // Prepare to draw.
//     let draw = app.draw();

//     // Generate sine wave data based on the time of the app
//     let sine = app.time.sin();
//     let slowersine = (app.time / 2.0).sin();

//     // Get boundary of the window (to constrain the movements of our circle)
//     let boundary = app.window_rect();

//     // Map the sine wave functions to ranges between the boundaries of the window
//     let x = map_range(
//         sine,
//         -1.0,
//         1.0,
//         boundary.left(),
//         boundary.right(),
//     );
//     let y = map_range(
//         slowersine,
//         -1.0,
//         1.0,
//         boundary.bottom(),
//         boundary.top(),
//     );

//     // Clear the background to purple.
//     draw.background().color(PLUM);

//     // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
//     draw.ellipse().color(STEELBLUE).x_y(x, y);

//     draw.to_frame(app, &frame).unwrap();
// }
