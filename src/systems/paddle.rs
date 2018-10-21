use amethyst::core::transform::components::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT, PADDLE_SPEED};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
	type SystemData = (
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Paddle>,
		Read<'s, InputHandler<String, String>>,
	);

	fn run(&mut self, (mut transforms, mut paddles, input): Self::SystemData) {
		for (paddle, transform) in (&mut paddles, &mut transforms).join() {
			paddle.velocity.y = 0.0;
			let movement = match paddle.side{
				Side::Left => input.axis_value("left_paddle"),
				Side::Right => input.axis_value("right_paddle"),
			};
			if let Some(mv_amount) = movement {
				paddle.velocity.y = PADDLE_SPEED * mv_amount as f32;

				transform.translation[1] = (transform.translation[1] + paddle.velocity.y)
					.min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
					.max(PADDLE_HEIGHT * 0.5);
				/*
				if mv_amount != 0.0 {
					let side_name = match paddle.side {
						Side::Left => "left",
						Side::Right => "right",
					};
					println!("Side {:?} moving {}", side_name, mv_amount);
				}
				*/
			}
		}
	}
}