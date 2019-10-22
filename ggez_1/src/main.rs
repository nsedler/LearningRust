use ggez;
use ggez::{event, timer};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::NumSamples;
use ggez::input::keyboard::KeyCode;
use ggez::event::KeyMods;

#[derive(Debug)]
struct InputState {
	xaxi: f32,
	yaxi: f32
}

impl Default for InputState {
	fn default() -> Self {
		InputState {
			xaxi: 0.0,
			yaxi: 0.0,
		}
	}
}

struct MainState {
	input: InputState
}

impl MainState {
	fn new() -> GameResult<MainState> {
		let s = MainState {
			input: InputState::default()
		};
		Ok(s)
	}
}

impl event::EventHandler for MainState {

	fn update(&mut self, ctx: &mut Context) -> GameResult {

		const DESIRED_FPS: u32 = 60;

		while timer::check_update_time(ctx, DESIRED_FPS) {
			let seconds = 1.0 / (DESIRED_FPS as f32);
			println!("{}", seconds)
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {

		graphics::clear(ctx, [0.2, 0.2, 0.2, 1.0].into());

		let (width, height) = ggez::graphics::size(ctx);

		let circle = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			na::Point2::new(width/2.0, height/2.0),
			50.0,
			1.0,
			[1.0,1.0,1.0,1.0].into()
		)?;

		graphics::draw(ctx, &circle, (na::Point2::new(self.input.xaxi, self.input.yaxi),))?;

		graphics::present(ctx)?;
		Ok(())
	}

	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		keycode: KeyCode,
		_keymod: KeyMods,
		_repeat: bool,
	) {
		match keycode {
			KeyCode::W => {
				println!("W");
				self.input.yaxi -= 3.0;
			}
			KeyCode::S => {
				println!("S");
				self.input.yaxi += 3.0;
			}
			KeyCode::D => {
				println!("D");
				self.input.xaxi += 3.0;
			}
			KeyCode::A => {
				println!("A");
				self.input.xaxi -= 3.0;
			}
			KeyCode::Escape => event::quit(ctx),
			_ => {}
		}
	}

}

pub fn main() -> GameResult {

	let ws = ggez::conf::WindowSetup {
		title: "Test".to_string(),
		samples: NumSamples::Sixteen,
		vsync: true,
		icon: "".to_string(),
		srgb: false
	};

	let cb = ggez::ContextBuilder::new("aaaa", "ggez")
		.window_setup(ws);
	let (ctx, event_loop) = &mut cb.build()?;
	let state = &mut MainState::new()?;

	event::run(ctx, event_loop, state)
}