use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use js_sys::Math::random;

#[wasm_bindgen]
pub struct Ball {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    radius: f64,
    color: String,
}

#[wasm_bindgen]
impl Ball {
    pub fn new(width: f64, height: f64) -> Ball {
        let radius = 10.0;
        Ball {
            x: random() * width,
            y: random() * height,
            dx: (random() - 0.5) * 4.0,
            dy: (random() - 0.5) * 4.0,
            radius,
            color: format!(
                "rgb({}, {}, {})",
                (random() * 255.0) as u8,
                (random() * 255.0) as u8,
                (random() * 255.0) as u8,
            ),
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        if self.x + self.radius > width || self.x - self.radius < 0.0 {
            self.dx = -self.dx;
        }
        if self.y + self.radius > height || self.y - self.radius < 0.0 {
            self.dy = -self.dy;
        }

        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str(&self.color));
        ctx
            .arc(self.x, self.y, self.radius, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        ctx.fill();
    }
}

#[wasm_bindgen]
pub struct World {
    balls: Vec<Ball>,
    width: f64,
    height: f64,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, count: usize) -> World {
        let mut balls = Vec::new();
        for _ in 0..count {
            balls.push(Ball::new(width, height));
        }
        World { balls, width, height }
    }

    pub fn update(&mut self) {
        for ball in &mut self.balls {
            ball.update(self.width, self.height);
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.width, self.height);
        for ball in &self.balls {
            ball.draw(ctx);
        }
    }
}