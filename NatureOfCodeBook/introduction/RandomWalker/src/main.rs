use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples, FullscreenType};
use rand::Rng;
use ggez::timer;

struct MainState {
    x: f32,
    y: f32
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            x: 0.0,
            y: 0.0
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        const FPS: u32 = 60;

        while timer::check_update_time(_ctx, FPS) {

            let randNum = rand::thread_rng().gen_range(0, 8);

            match randNum {

                0 => self.x += 1.0,
                1 => self.x -= 1.0,
                2 => self.y += 1.0,
                3 => self.y -= 1.0,
                4 => {
                    self.x += 1.0;
                    self.y += 1.0;
                },
                5 => {
                    self.x -= 1.0;
                    self.y -= 1.0;
                },
                6 => {
                    self.x += 1.0;
                    self.y -= 1.0;
                },
                7 => {
                    self.x -= 1.0;
                    self.y += 1.0;
                },

                _ => ()
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        let (width, height) = ggez::graphics::size(ctx);

        let point = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(width / 2.0, height / 2.0),
            1.0,
            1.0,
            graphics::WHITE
        )?;

        graphics::draw(ctx, &point, (na::Point2::new(self.x, self.y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(
            ggez::conf::WindowSetup {
                title: "Random Walker".to_string(),
                samples: NumSamples::Zero,
                vsync: false,
                icon: "".to_string(),
                srgb: false
            }
        )
        .window_mode(
            ggez::conf::WindowMode{
                width: 300.0,
                height: 300.0,
                maximized: false,
                fullscreen_type: FullscreenType::Windowed,
                borderless: false,
                min_width: 0.0,
                min_height: 0.0,
                max_width: 0.0,
                max_height: 0.0,
                resizable: false
            }
        );
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}