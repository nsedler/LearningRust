use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::timer;
use ggez::input::keyboard::KeyCode;
use ggez::event::KeyMods;
use ggez::input::keyboard;
use ggez::input;
use std::ptr::hash;

struct MainState {
    xaxis: f32,
    yaxis: f32
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { 
            xaxis: 0.0,
            yaxis: 0.0

         };
        Ok(s)
    }

}

impl event::EventHandler for MainState {


    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIREDFPS: u32 = 60;

        while timer::check_update_time(ctx, DESIREDFPS) {

            let seconds = ggez::timer::average_delta(ctx).as_secs_f32();

            if input::keyboard::is_key_pressed(ctx, KeyCode::W) {
                self.yaxis += -300.0 * seconds;
            }
            if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
                self.xaxis += -300.0 * seconds;
            }
            if input::keyboard::is_key_pressed(ctx, KeyCode::S) {
                self.yaxis += 300.0 * seconds;
            }
            if input::keyboard::is_key_pressed(ctx, KeyCode::D) {
                self.xaxis += 300.0 * seconds;
            }
            if input::keyboard::is_key_pressed(ctx, KeyCode::Space) {

            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let (width, height) = ggez::graphics::size(ctx);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(width / 2.0, height / 2.0),
            100.0,
            1.0,
            [1.0, 1.0, 1.0, 1.0].into()
        )?;

        circle.asd();

        graphics::draw(ctx, &circle, (na::Point2::new(self.xaxis, self.yaxis), ))?;

        graphics::present(ctx)?;
        Ok(())
    }
}


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}