use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::PhysicsTime;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let camera = OrthographicCameraBundle::new_2d();
	commands.spawn_bundle(camera);

	asset_server.watch_for_changes().unwrap();

	let ldtk_handle = asset_server.load("et-hs.ldtk");
	commands.spawn_bundle(LdtkWorldBundle {
		ldtk_handle,
		..Default::default()
	});
}

pub fn pause_physics_during_load(
	mut level_events: EventReader<LevelEvent>,
	mut physics_time: ResMut<PhysicsTime>,
) {
	for event in level_events.iter() {
		match event {
			LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
			LevelEvent::Transformed(_) => physics_time.set_scale(1.),
			_ => (),
		}
	}
}
