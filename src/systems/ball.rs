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
			let mut contacted = false;
			for (paddle, paddle_transform) in (&paddles, &transforms).join() {
				if !ball.in_contact &&
				   ball_transform.translation.x <= paddle_transform.translation.x+PADDLE_WIDTH*0.5 &&
				   ball_transform.translation.x >= paddle_transform.translation.x-PADDLE_WIDTH &&
				   ball_transform.translation.y >= paddle_transform.translation.y-PADDLE_HEIGHT*0.5 &&
				   ball_transform.translation.y <= paddle_transform.translation.y+PADDLE_HEIGHT*0.5 {
				   	contacted = true;
				   	ball.in_contact = true;
				   	let mut angle = Rad::atan2(ball.velocity.y+paddle.velocity.y, -ball.velocity.x);
				   	println!("paddle touched! {:?} at x:{}, y:{}", angle*(360.0/(2.0*3.14159)), ball_transform.translation.x, ball_transform.translation.y);
				   	const PI: f32 = 3.141593;
				   	const PI_4 : f32 = 0.785398;
				   	const PI_2: f32 = 1.570796;
				   	const PI3_4 : f32 = 2.356194;

				   	if angle.0 > PI_4 &&angle.0 <= PI_2 {
				   		angle.0 = PI_4;
				   	}else if angle.0 < PI && angle.0 >= PI_2 {
				   		angle.0 = PI;
				   	}else if angle.0 > -PI3_4 && angle.0 <= -PI_2 {
				   		angle.0 = -PI3_4;
				   	}else if angle.0 < -PI_4 && angle.0 >= -PI_2 {
				   		angle.0 = -PI_4;
				   	}

				   	let (y,x) = angle.sin_cos();
				   	ball.velocity.y = y*BALL_SPEED;
				   	ball.velocity.x = x*BALL_SPEED;
				}
			}
			if !contacted{
				ball.in_contact = false
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

pub struct BallScore;
impl<'s> System<'s> for BallScore {
	type SystemData = (
		ReadStorage<'s, Ball>,
		WriteStorage<'s, Transform>,
	);
	fn run(&mut self, (balls, mut transforms): Self::SystemData){
		for (_, transform) in (&balls, &mut transforms).join() {
			if transform.translation.x <= 0.0 {

			}else if transform.translation.x >= ARENA_WIDTH {

			}
		}
	}
}