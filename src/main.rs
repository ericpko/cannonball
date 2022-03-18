// #![allow(unused)]
use ggez::event;
use ggez::graphics::{self, Color, Mesh};
use ggez::{Context, GameResult};
use glam::Vec2;

mod ball;
use ball::Ball;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = WIDTH / ASPECT_RATIO;

const SIM_MIN_WIDTH: f32 = 20.0;
// const SIM_SCALE: f32 = f32::min(WIDTH, HEIGHT) / SIM_MIN_WIDTH;
const SIM_SCALE: f32 = HEIGHT / SIM_MIN_WIDTH;
const SIM_WIDTH: f32 = WIDTH / SIM_SCALE;
const SIM_HEIGHT: f32 = HEIGHT / SIM_SCALE;

const _DT: f32 = 1.0 / 60.0;
const GRAVITY: f32 = 9.81;
const DAMPENING: f32 = 0.9;

fn map_sim_to_context_position(pos: Vec2) -> Vec2 {
    let ctx_pos = Vec2::new(pos.x * SIM_SCALE, HEIGHT - pos.y * SIM_SCALE);
    return ctx_pos;
}

struct SimulationState {
    ball: Ball,
    mesh: Mesh,
}

impl SimulationState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ball = Ball::new();
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            0.2 * SIM_SCALE,
            0.3,
            Color::from_rgb(234, 224, 213),
        )?;
        let s = Self { ball, mesh };
        Ok(s)
    }

    fn symplectic_euler(&mut self, _dt: f32) {
        let substeps = 5;
        let sdt = _DT / substeps as f32;

        let mut v = self.ball.vel;
        let mut r = self.ball.pos;

        for _ in 0..substeps {
            v += -GRAVITY * Vec2::new(0.0, 1.0) * sdt;
            r += v * sdt;
        }

        // Check window bounds
        if r.x < 0.0 {
            v.x = -v.x * DAMPENING;
            r.x = 0.0;
        } else if r.x > SIM_WIDTH {
            v.x = -v.x * DAMPENING;
            r.x = SIM_WIDTH;
        }
        if r.y < 0.4 {
            v.y = -v.y * DAMPENING;
            r.y = 0.4;
        } else if r.y > SIM_HEIGHT {
            v.y = -v.y * DAMPENING;
            r.y = SIM_HEIGHT;
        }

        self.ball.vel = v;
        self.ball.pos = r;
    }
}

impl event::EventHandler<ggez::GameError> for SimulationState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx);
        let tick = dt.as_secs_f32();

        self.symplectic_euler(tick);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(30, 30, 30));

        let ctx_pos = map_sim_to_context_position(self.ball.pos);
        let params = graphics::DrawParam::new().dest(ctx_pos);
        graphics::draw(ctx, &self.mesh, params)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("SPH-Fluid", "Eric Koehli");

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, event_loop) = cb
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(WIDTH, HEIGHT)
                .resizable(true),
        )
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Cannonball Simulation!")
                .samples(ggez::conf::NumSamples::Eight)
                .vsync(true),
        )
        .build()?;
    let state = SimulationState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
