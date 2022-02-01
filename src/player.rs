use crate::physics::ColliderBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
	#[sprite_bundle("boye.png")]
	#[bundle]
	pub sprite_bundle: SpriteBundle,
	#[from_entity_instance]
	#[bundle]
	pub collider_bundle: ColliderBundle,
	pub player: Player,
	#[worldly]
	pub worldly: Worldly,
}

pub fn player_move(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
	for mut velocity in query.iter_mut() {
		let mut desired = Vec3::default();
		if input.pressed(KeyCode::Up) {
			desired.y += 1.0;
		}
		if input.pressed(KeyCode::Down) {
			desired.y += -1.0;
		}
		if input.pressed(KeyCode::Left) {
			desired.x += -1.0;
		}
		if input.pressed(KeyCode::Right) {
			desired.x += 1.0;
		}
		velocity.linear = velocity.linear.lerp(desired * 100.0, 0.2);
	}
}

// For whatever reason when I give the player physics, it teleports the player 4096 units upwards?
pub fn oob_failsafe(
	mut character: Query<&mut Transform, Without<Handle<LdtkLevel>>>,
	levels: Query<(&Transform, &Handle<LdtkLevel>)>,
	ldtk_levels: Res<Assets<LdtkLevel>>,
) {
	let max_height = levels
		.iter()
		.flat_map(|(transform, level_handle)| {
			if let Some(level) = ldtk_levels.get(level_handle) {
				Some(transform.translation.y + level.level.px_hei as f32)
			} else {
				None
			}
		})
		.reduce(f32::max)
		.unwrap_or(0.);
	for mut transform in character.iter_mut() {
		if transform.translation.y > max_height {
			transform.translation.y -= 4096.;
		}
	}
}
