use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::NumSamples;
use ggez::timer;
use rand::Rng;
use ggez::nalgebra::Point2;

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

#[allow(non_snake_case)]
impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {

		const DESIREDFPS: u32 = 60;

		while timer::check_update_time(_ctx, DESIREDFPS) {
			let randNum:f32 = rand::thread_rng().gen::<f32>();
			let mousePos = ggez::input::mouse::position(_ctx);
			let test = mousePos;


			match randNum {
				0.0 ..=0.5 => {
					let mut xDir = test.x - self.x;
					let mut yDir = test.y - self.y;

					if xDir != 0.0 {
						xDir /= xDir.abs();
					}
					if yDir != 0.0 {
						yDir /= yDir.abs();
					}

					self.x += xDir;
					self.y += yDir;

					println!("1 {}, {}", self.x, self.y)
				},
				0.51 ..=1.0 => {
					let mut xDir = rand::thread_rng().gen_range(-2, 2);
					let mut yDir = rand::thread_rng().gen_range(-2, 2);

					self.x += xDir as f32;
					self.y += yDir as f32;

					println!("2 {}, {}", self.x, self.y)
				},
				_ => ()
			};
		}


		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {

		let (width, height) = graphics::size(ctx);

		let circle = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			na::Point2::new(width / 2.0, height / 2.0),
			2.0,
			1.0,
			graphics::WHITE
		)?;

		graphics::draw(ctx, &circle, (na::Point2::new(self.x, self.y),))?;

		graphics::present(ctx)?;
		Ok(())
	}
}

pub fn main() -> GameResult {
	let cb = ggez::ContextBuilder::new("super_simple", "ggez")
		.window_setup(
			ggez::conf::WindowSetup {
				title: "Random Number Distribution".to_string(),
				samples: NumSamples::Zero,
				vsync: false,
				icon: "".to_string(),
				srgb: false
			}
		);
	let (ctx, event_loop) = &mut cb.build()?;
	let state = &mut MainState::new()?;
	event::run(ctx, event_loop, state)
}