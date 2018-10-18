use amethyst::core::transform::components::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use pong::{Paddle, Ball, ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS};

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
	type SystemData = (
		// writing ot the transform values
		WriteStorage<'s, Transform>,

		// reading form the paddle values
		ReadStorage<'s, Paddle>,

		// reading form the Ball values
		ReadStorage<'s, Ball>,
	);

	fn run(&mut self, (mut transforms, paddles, balls): Self::SystemData) {
		for (ball, transform) in (&balls, &mut transforms).join() {


			let dx = (transform.translation.x + ball.velocity.x)
				.min(ARENA_WIDTH - BALL_RADIUS)
				.max(BALL_RADIUS);
			let dy = (transform.translation.y + ball.velocity.y)
				.min(ARENA_HEIGHT - BALL_RADIUS)
				.max(BALL_RADIUS);

			for (paddles, transform) in  ().join() {


			}


		}
	}
}
