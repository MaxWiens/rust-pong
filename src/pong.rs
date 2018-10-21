use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};

pub struct Pong;

const SPRITESHEET_SIZE:  (f32, f32) = (8.0, 16.0);

impl<'a, 'b> State<GameData<'a,'b>> for Pong {
	fn on_start(&mut self, data: StateData<GameData>) {
		let world = data.world;

		// registers components in the world
		world.register::<Paddle>();
		world.register::<Ball>();

		// load spritesheet
		let spritesheet = {
			let loader = world.read_resource::<Loader>();
			let texture_storage = world.read_resource::<AssetStorage<Texture>>();
			loader.load(
				"texture/pong_spritesheet.png",
				PngFormat,
				Default::default(),
				(),
				&texture_storage,
			)
		};
		initialise_ball(world, spritesheet.clone());
		initialise_paddles(world, spritesheet.clone());

		initialise_camera(world);

	}


	fn handle_event(&mut self, _: StateData<GameData>, event:Event) -> Trans<GameData<'a,'b>> {
		if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
			Trans::Quit
		} else {
			Trans::None
		}
	}

	fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
		data.data.update(&data.world);
		Trans::None
	}
}




//
// camera
//
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
	world.create_entity()
		.with(Camera::from(Projection::orthographic(
			0.0,
			ARENA_WIDTH,
			ARENA_HEIGHT,
			0.0,
		)))
		.with(GlobalTransform(
			Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
		))
		.build();
}



//
// ball game component
//
pub const BALL_RADIUS: f32 = 2.0;
pub const BALL_SPEED: f32 = 1.0;
pub struct Ball {
	pub radius : f32,
	pub restitution: f32,
	pub velocity: Vector3<f32>,
}
impl Ball {
	fn new() -> Ball {
		Ball {
			radius: BALL_RADIUS,
			restitution: 1.0,
			velocity: Vector3::new(BALL_SPEED,0.0,0.0),
		}
	}
}

impl Component for Ball {
	type Storage = DenseVecStorage<Self>;
}

fn initialise_ball(world: &mut World, spritesheet: TextureHandle) {
	let mut ball_transform = Transform::default();
	ball_transform.translation = Vector3::new(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

	let sprite = Sprite {
		left: 4.0,
		right: 8.0,
		top: 0.0,
		bottom: 4.0,
	};

	// create ball
	world.create_entity()
		.with_sprite(&sprite, spritesheet, SPRITESHEET_SIZE)
			.expect("Failed to add sprite render on ball")
		.with(Ball::new())
		.with(GlobalTransform::default())
		.with(ball_transform)
		.build();
}


//
// paddle game component
//
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_SPEED: f32 = 1.2;
#[derive(PartialEq, Eq)]
pub enum Side {
	Left,
	Right,
}

pub struct Paddle {
	pub side: Side,
	pub width: f32,
	pub height: f32,
	pub velocity : Vector3<f32>
}

impl Paddle {
	fn new(side: Side) -> Paddle {
		Paddle {
			side: side,
			width: 1.0,
			height: 1.0,
			velocity: Vector3::new(0.0,0.0,0.0),
		}
	}
}

impl Component for Paddle {
	type Storage = DenseVecStorage<Self>;
}

fn initialise_paddles(world: &mut World, spritesheet: TextureHandle) {
	let mut left_transform = Transform::default();
	let mut right_transform = Transform::default();
	// paddle position
	let y = ARENA_HEIGHT / 2.0;
	left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, y, 0.0);
	right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

	//builds sprite for paddles
	let sprite = Sprite {
		left: 0.0,
		right: PADDLE_WIDTH,
		top: 0.0,
		bottom: PADDLE_HEIGHT,
	};

	// create left paddle
	world.create_entity()
		.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
			.expect("Failed to add sprite render on left paddle")
		.with(Paddle::new(Side::Left))
		.with(GlobalTransform::default())
		.with(left_transform)
		.build();
	// create right paddle
	world.create_entity()
		.with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE) // inefficent useing the same sprite should use SpriteRenderData instead
			.expect("Failed to add sprite render on right paddle")
		.with(Paddle::new(Side::Right))
		.with(GlobalTransform::default())
		.with(right_transform)
		.build();
}