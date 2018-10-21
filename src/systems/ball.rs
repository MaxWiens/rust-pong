use amethyst::core::transform::components::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use pong::{Paddle, Ball, ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS, PADDLE_WIDTH, PADDLE_HEIGHT, BALL_SPEED};
use amethyst::core::cgmath::prelude::*;
use amethyst::core::cgmath::Rad;
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
		for (ball, ball_transform) in (&mut balls, &transforms).join() {
			if ball_transform.translation.y <= BALL_RADIUS || ball_transform.translation.y >= ARENA_HEIGHT - BALL_RADIUS {
				ball.velocity.y = - ball.velocity.y;
			}

			for (paddle, paddle_transform) in (&paddles, &transforms).join() {
				if ball_transform.translation.x <= paddle_transform.translation.x + PADDLE_WIDTH &&
				   ball_transform.translation.x >= paddle_transform.translation.x &&
				   ball_transform.translation.y >= paddle_transform.translation.y &&
				   ball_transform.translation.y <= paddle_transform.translation.y + PADDLE_HEIGHT {

				   	let new_angle = Rad::atan(ball.velocity.y+paddle.velocity.y/-ball.velocity.x);
				   	println!("paddle touched! {:?}", paddle.velocity.y);
				   	let (y,x) = new_angle.sin_cos();
				   	ball.velocity.y = -y*BALL_SPEED;
				   	ball.velocity.x = x*BALL_SPEED;
				}
			}

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