use itertools::Itertools;
use names::Generator;
use nannou::{
    color::Lch,
    glam::Vec2,
    image::{DynamicImage, GenericImage},
    noise::{BasicMulti, NoiseFn, Perlin, Seedable},
    prelude::*,
    wgpu::Texture,
};
use nannou_egui::{self, egui, Egui};

// interesting variables
const NOISE_STEP: f64 = 500.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    lightness: f64,
    chroma: f64,
    grayscale: bool,
    hue_center: f64,
    hue_range: f64,
}
struct Model {
    settings: Settings,
    egui: Egui,
    image_window: WindowId,
    should_redraw: bool,

    noise: BasicMulti,
    image: DynamicImage,
    size: Vec2, // frame_start: u64,
}

impl Model {
    fn new(
        noise: Option<BasicMulti>,
        egui: Egui,
        image_window: WindowId,
    ) -> Self {
        let image = DynamicImage::new_rgb8(10, 10);

        Model {
            settings: Settings {
                lightness: 50.,
                chroma: 80.,
                grayscale: false,
                hue_center: 0.0,
                hue_range: 180.0,
            },
            egui,
            should_redraw: false,
            image_window,
            noise: noise.unwrap_or(BasicMulti::new()),
            image: image,
            size: Vec2::new(10., 10.),
        }
    }
    fn update_noise(&mut self, noise: BasicMulti) {
        self.noise = noise;
        self.redraw_image();
    }
    fn update_size(&mut self, size: Vec2) {
        let image = DynamicImage::new_rgb8(
            size.x.floor() as u32,
            size.y.floor() as u32,
        );
        self.image = image;
        self.size = size;

        self.redraw_image()
    }
    fn redraw_image(&mut self) {
        for (x, y) in (0..self.size.x.floor() as u32)
            .cartesian_product(
                0..self.size.y.floor() as u32,
            )
        {
            let value = self.noise.get([
                x as f64 / NOISE_STEP,
                y as f64 / NOISE_STEP,
            ]);
            if self.settings.grayscale {
                let mapped_value =
                    map_range(value, -1.0, 1.0, 0., 255.);
                if let Some(buffer) =
                    self.image.as_mut_rgb8()
                {
                    buffer.put_pixel(
                        x,
                        y,
                        nannou::image::Rgb::from([
                            mapped_value as u8,
                            mapped_value as u8,
                            mapped_value as u8,
                        ]),
                    );
                }
            } else {
                let mapped_hue: f64 = map_range(
                    value,
                    -1.0,
                    1.0,
                    self.settings.hue_center
                        - self.settings.hue_range,
                    self.settings.hue_center
                        + self.settings.hue_range,
                );
                // 0-100, 0-128,181, -180-180
                let color = Lch::new(
                    self.settings.lightness,
                    self.settings.chroma,
                    mapped_hue,
                );
                let rgb_color: Rgb<f64> =
                    nannou::color::rgb::Rgb::from(color);
                let (r, g, b) = rgb_color.into_components();
                let mapped_r =
                    map_range(r, -1.0, 1.0, 0., 255.);
                let mapped_g =
                    map_range(g, -1.0, 1.0, 0., 255.);
                let mapped_b =
                    map_range(b, -1.0, 1.0, 0., 255.);
                if let Some(buffer) =
                    self.image.as_mut_rgb8()
                {
                    buffer.put_pixel(
                        x,
                        y,
                        nannou::image::Rgb::from([
                            mapped_r as u8,
                            mapped_g as u8,
                            mapped_b as u8,
                        ]),
                    );
                }
            }
        }
    }
}

fn model(app: &App) -> Model {
    let image_window = app
        .new_window()
        .size(1200, 630)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let window_id = app
        .new_window()
        .view(egui_view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model::new(None, egui, image_window)
}
fn raw_window_event(
    _app: &App,
    model: &mut Model,
    event: &nannou::winit::event::WindowEvent,
) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}
fn update(app: &App, model: &mut Model, update: Update) {
    let win_rect =
        app.window(model.image_window).unwrap().rect();

    if model.size != win_rect.wh() || model.should_redraw {
        model.update_size(win_rect.wh());
        model.redraw_image();
        model.should_redraw = false;
    }
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        let mut changed = false;

        // Lightness slider
        let slider = egui::Slider::new(
            &mut settings.lightness,
            0.0..=100.0,
        );
        ui.label("Lightness:");
        changed |= ui.add(slider).changed();

        // Chroma slider
        ui.label("Chroma:");
        changed |= ui
            .add(egui::Slider::new(
                &mut settings.chroma,
                0.0..=181.0,
            ))
            .changed();

        // Chroma slider
        ui.label("Hue Center:");
        changed |= ui
            .add(egui::Slider::new(
                &mut settings.hue_center,
                -180.0..=180.0,
            ))
            .changed();

        ui.label("Hue range:");
        changed |= ui
            .add(egui::Slider::new(
                &mut settings.hue_range,
                0.0..=180.0,
            ))
            .changed();

        changed |= ui
            .add(egui::Checkbox::new(
                &mut settings.grayscale,
                "Grayscale",
            ))
            .changed();
        // Random color button
        let clicked = ui.button("update").clicked();

        if clicked || changed {
            model.should_redraw = true;
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let background = rgb(0.439, 0.039, 0.467);
    let foreground = rgb(0.855, 0.31, 0.671);

    // set up containing rectangles
    let win_rect = app.window_rect();
    let win_p = win_rect.pad(25.0);

    let draw = app.draw();
    draw.background().color(background);

    let texture = Texture::from_image(app, &model.image);
    draw.texture(&texture);

    // display noise seed
    let seed = model.noise.seed();
    let seed_bytes = seed.to_be_bytes();
    let seed_display =
        std::str::from_utf8(&seed_bytes).unwrap();

    draw.text(&format!("{}", seed_display))
        .font_size(48)
        .wh(win_p.wh())
        .right_justify()
        .align_text_bottom()
        .color(background);

    // draw to frame
    draw.to_frame(app, &frame).unwrap();
}
fn egui_view(app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
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
    // generate a new seed using ascii characters, which
    // tend to be more human-readable than numbers
    let mut generator = Generator::default();
    let seed = generator.next().unwrap();
    let mut bytes: [u8; 4] = [0; 4];
    for (i, byte) in
        seed.as_bytes().iter().take(4).enumerate()
    {
        bytes[i] = *byte;
    }
    let seed_number: u32 = u32::from_be_bytes(bytes);

    // instantiate the new noise
    let noise = BasicMulti::new();
    let seeded_noise = noise.set_seed(seed_number);

    // set relevant values on model,
    // clearing any old data so that data
    // from previous seeds doesn't render
    model.update_noise(seeded_noise);
}
