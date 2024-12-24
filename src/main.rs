use nannou::prelude::*;

struct Model {
    zoom: f32,
    rotation: f32,
    speed: f32,
    auto_rotate: bool,
    spiral_size: f32,
    turns: f32,
    n: f32,
    multiplier: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 600)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_wheel(mouse_wheel)
        .build()
        .unwrap();

    Model {
        zoom: 1.0,
        rotation: 0.0,
        speed: 0.01,
        auto_rotate: false,
        spiral_size: 5.0,
        turns: 4.0,
        n: 2.0,
        multiplier: 10.0, // Multiplicateur initial
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.auto_rotate {
        model.rotation += model.speed;
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.auto_rotate = !model.auto_rotate,
        Key::Up => model.spiral_size = (model.spiral_size + 1.0).min(20.0),
        Key::Down => model.spiral_size = (model.spiral_size - 1.0).max(1.0),
        Key::Left => model.turns = (model.turns - 0.5).max(1.0),
        Key::Right => model.turns = (model.turns + 0.5).min(10.0),
        Key::Q => model.n = (model.n + 0.1).min(10.0),
        Key::A => model.n = (model.n - 0.1).max(0.5),
        Key::R => {
            model.zoom = 1.0;
            model.rotation = 0.0;
            model.speed = 0.01;
            model.spiral_size = 5.0;
            model.turns = 4.0;
            model.n = 2.0;
            model.multiplier = 10.0;
        },
        Key::M => model.multiplier += 1.0,
        Key::N => model.multiplier = (model.multiplier - 1.0).max(1.0),
        _ => (),
    }
}

fn mouse_wheel(_app: &App, model: &mut Model, dt: MouseScrollDelta, _touch: TouchPhase) {
    match dt {
        MouseScrollDelta::LineDelta(_x, y) => {
            model.zoom *= 1.0 + y * 0.1;
            model.zoom = model.zoom.max(0.1).min(5.0);
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().rotate(model.rotation).scale(model.zoom);
    draw.background().color(BLACK);

    const TOTAL_POINTS: usize = 2000;
    const LINE_WEIGHT: f32 = 2.0;

    let mut points = Vec::new();
    let max_angle = model.turns * PI * 2.0;

    for i in 0..=TOTAL_POINTS {
        let t = i as f32 / TOTAL_POINTS as f32;
        let angle = t * max_angle;
        let radius = model.spiral_size * (1.0 + angle / model.n);

        let x = radius * angle.cos() * model.multiplier;
        let y = radius * angle.sin() * model.multiplier;
        points.push(pt2(x, y));
    }

    for (i, point) in points.iter().enumerate() {
        let color = hsla(
            i as f32 / TOTAL_POINTS as f32,
            0.7,
            0.5 + 0.3 * (i as f32 / TOTAL_POINTS as f32),
            1.0,
        );
        if i > 0 {
            draw.line()
                .start(points[i - 1])
                .end(*point)
                .weight(LINE_WEIGHT)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
