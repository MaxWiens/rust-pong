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
		let mut dx : f32 = 0.0;
		let mut dy : f32 = 0.0;
		let mut new_angle : Option<f32> = None;
		for (ball, transform) in (&balls, &mut transforms).join() {
			dx = (transform.translation.x + ball.velocity.x)
				.min(ARENA_WIDTH - BALL_RADIUS)
				.max(BALL_RADIUS);
			dy = (transform.translation.y + ball.velocity.y)
				.min(ARENA_HEIGHT - BALL_RADIUS)
				.max(BALL_RADIUS);
		}
		for (paddle, transform) in  (&paddles, &transforms).join() {
			// if is colliding
			new_angle = Some((dy/dx).atan());
		}
		if let Some(angle) = new_angle {
			angle
		}
	}
}
