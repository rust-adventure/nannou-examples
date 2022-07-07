use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use itertools::Itertools;
use nannou::noise::BasicMulti;
use nannou::noise::NoiseFn;
use nannou::{image::DynamicImage, math::map_range};

fn get_noise(
    noise: &BasicMulti,
    points: &[(u32, u32)],
    size: (u32, u32),
) -> DynamicImage {
    let mut image: DynamicImage =
        DynamicImage::new_rgb8(size.0, size.1);
    for (x, y) in points {
        let value =
            noise.get([*x as f64 / 500., *y as f64 / 500.]);
        let mapped_value =
            map_range(value, -1.0, 1.0, 0., 255.) as u8;
        if let Some(buffer) = image.as_mut_rgb8() {
            buffer.put_pixel(
                *x,
                *y,
                nannou::image::Rgb::from([
                    mapped_value,
                    mapped_value,
                    mapped_value,
                ]),
            );
        }
    }
    image
}

fn res_1920x1080_benchmark(c: &mut Criterion) {
    let noise = BasicMulti::new();
    let points = (0..1920)
        .cartesian_product(0..1080)
        .collect::<Vec<(u32, u32)>>();
    c.bench_function("noise 1920x1080", |b| {
        b.iter(|| {
            get_noise(
                black_box(&noise),
                black_box(&points),
                black_box((1920, 1080)),
            )
        })
    });
}

fn res_3840x2160_benchmark(c: &mut Criterion) {
    let noise = BasicMulti::new();
    let points = (0..3840)
        .cartesian_product(0..2160)
        .collect::<Vec<(u32, u32)>>();
    c.bench_function("noise 3840x2160", |b| {
        b.iter(|| {
            get_noise(
                black_box(&noise),
                black_box(&points),
                black_box((3840, 2160)),
            )
        })
    });
}

criterion_group!(
    benches,
    res_1920x1080_benchmark,
    res_3840x2160_benchmark
);
criterion_main!(benches);
