use amethyst::core::transform::components::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use pong::{Paddle, Ball, ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS};
pub struct BallMovement;

impl<'s> System<'s> for BallMovement {
	type SystemData = (
		// writing ot the transform values
		WriteStorage<'s, Transform>,

		// reading form the Ball values
		ReadStorage<'s, Ball>,
	);

	fn run(&mut self, (mut transforms, balls): Self::SystemData) {
		for (ball, transform) in (&balls, &mut transforms).join() {
			transform.translation.x = (transform.translation.x + ball.velocity.x)
				.min(ARENA_WIDTH - BALL_RADIUS)
				.max(BALL_RADIUS);
			transform.translation.y = (transform.translation.y + ball.velocity.y)
				.min(ARENA_HEIGHT - BALL_RADIUS)
				.max(BALL_RADIUS);
		}
	}
}


pub struct BallBounce;
impl<'s> System<'s> for BallBounce {
	type SystemData = (
		WriteStorage<'s, Ball>,
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Paddle>
	);

	fn run(&mut self, (mut balls, transforms, paddles): Self::SystemData){
		for (ball, transform, paddle) in (&mut balls, &transforms, &paddles).join() {


			/*
			transform.translation.x = (transform.translation.x + ball.velocity.x)
				.min(ARENA_WIDTH - BALL_RADIUS)
				.max(BALL_RADIUS);
			transform.translation.y = (transform.translation.y + ball.velocity.y)
				.min(ARENA_HEIGHT - BALL_RADIUS)
				.max(BALL_RADIUS);
			*/
		}

	}


}