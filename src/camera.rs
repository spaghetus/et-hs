use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::PhysicsTime;

pub fn camera_fit_inside_current_level(
	mut camera_query: Query<
		(
			&mut bevy::render::camera::OrthographicProjection,
			&mut Transform,
		),
		Without<Player>,
	>,
	player_query: Query<(&Transform, &GlobalTransform), With<Player>>,
	level_query: Query<
		(&Transform, &Handle<LdtkLevel>),
		(Without<OrthographicProjection>, Without<Player>),
	>,
	level_selection: Res<LevelSelection>,
	ldtk_levels: Res<Assets<LdtkLevel>>,
	window: Res<Windows>,
) {
	let aspect_ratio = window.get_primary().unwrap().width() as f32
		/ window.get_primary().unwrap().height() as f32;
	let padding_x = window.get_primary().unwrap().width() as f32 * 0.1 / 100.;
	let padding_y = window.get_primary().unwrap().height() as f32 * 0.1 / 100.;
	if let Ok((
		Transform {
			translation: player_translation,
			..
		},
		GlobalTransform {
			translation: player_global_translation,
			..
		},
	)) = player_query.get_single()
	{
		let player_translation = player_translation.clone();

		let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

		if std::env::var("FOLLOW_PLAYER_OOB").is_ok() {
			camera_transform.translation = player_global_translation.clone();
			return;
		}

		for (level_transform, level_handle) in level_query.iter() {
			if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
				let level = &ldtk_level.level;
				if level_selection.is_match(&0, &level) {
					let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;
					orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
					orthographic_projection.bottom = 0.;
					orthographic_projection.left = 0.;
					if level_ratio > aspect_ratio {
						// level is wider than the screen
						orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
						orthographic_projection.right = orthographic_projection.top * aspect_ratio;
						camera_transform.translation.x = (player_translation.x
							- level_transform.translation.x
							- orthographic_projection.right / 2.)
							.clamp(0., level.px_wid as f32 - orthographic_projection.right);
						camera_transform.translation.y = 0.;
					} else {
						// level is taller than the screen
						orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
						orthographic_projection.top = orthographic_projection.right / aspect_ratio;
						camera_transform.translation.y = (player_translation.y
							- level_transform.translation.y
							- orthographic_projection.top / 2.)
							.clamp(0., level.px_hei as f32 - orthographic_projection.top);
						camera_transform.translation.x = 0.;
					}

					camera_transform.translation.x += level_transform.translation.x;
					camera_transform.translation.y += level_transform.translation.y;
				}
			}
			orthographic_projection.right += padding_x * 2.;
			camera_transform.translation.x -= padding_x;
			orthographic_projection.top += padding_y * 2.;
			camera_transform.translation.y -= padding_y;
		}
	}
}

pub fn update_level_selection(
	level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
	player_query: Query<&Transform, With<Player>>,
	mut level_selection: ResMut<LevelSelection>,
	ldtk_levels: Res<Assets<LdtkLevel>>,
) {
	for (level_handle, level_transform) in level_query.iter() {
		if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
			let level_bounds = Rect {
				bottom: level_transform.translation.y,
				top: level_transform.translation.y + ldtk_level.level.px_hei as f32,
				left: level_transform.translation.x,
				right: level_transform.translation.x + ldtk_level.level.px_wid as f32,
			};

			for player_transform in player_query.iter() {
				if player_transform.translation.x < level_bounds.right
					&& player_transform.translation.x > level_bounds.left
					&& player_transform.translation.y < level_bounds.top
					&& player_transform.translation.y > level_bounds.bottom
				{
					if !level_selection.is_match(&0, &ldtk_level.level) {
						println!("level selection: {}", ldtk_level.level.identifier);
						*level_selection = LevelSelection::Uid(ldtk_level.level.uid);
					}
				}
			}
		}
	}
}

pub fn black_when_loading(
	mut level_events: EventReader<LevelEvent>,
	mut physics_time: ResMut<PhysicsTime>,
	mut camera: Query<&mut Transform, With<Camera>>,
	mut input: ResMut<Input<KeyCode>>,
) {
	for event in level_events.iter() {
		match event {
			LevelEvent::SpawnTriggered(_) => {
				physics_time.set_scale(0.);
				// for i in [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right].iter() {
				// 	input.reset(*i);
				// }
				camera.single_mut().translation.z = -1.;
			}
			LevelEvent::Transformed(_) => {
				physics_time.set_scale(1.);
				// for i in [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right].iter() {
				// 	input.reset(*i);
				// }
				camera.single_mut().translation.z = 1.;
			}
			_ => (),
		}
	}
}
