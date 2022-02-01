pub mod camera;
pub mod physics;
pub mod player;
pub mod setup;
pub mod wall;

use bevy::prelude::*;
use bevy::render::{options::WgpuOptions, render_resource::WgpuLimits};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use heron::prelude::*;

fn main() {
	App::new()
		.insert_resource(WgpuOptions {
			limits: WgpuLimits {
				max_texture_array_layers: 2048,
				..Default::default()
			},
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(WorldInspectorPlugin::default())
		.add_plugin(LdtkPlugin)
		.add_plugin(PhysicsPlugin::default())
		.insert_resource(LevelSelection::Uid(0))
		.insert_resource(LdtkSettings {
			load_level_neighbors: true,
			use_level_world_translations: true,
		})
		.add_startup_system(setup::setup.system())
		.add_system(setup::pause_physics_during_load.system())
		.add_system(camera::camera_fit_inside_current_level.system())
		.add_system(camera::update_level_selection.system())
		// .add_system(camera::black_when_loading.system())
		.add_system(player::player_move.system())
		.add_system(player::oob_failsafe.system())
		.register_ldtk_int_cell::<wall::WallBundle>(1)
		.register_ldtk_entity::<player::PlayerBundle>("Player")
		.run();
}
