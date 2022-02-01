use bevy::prelude::Bundle;
use bevy_ecs_ldtk::LdtkIntCell;

use crate::physics::ColliderBundle;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
	#[from_int_grid_cell]
	#[bundle]
	pub collider_bundle: ColliderBundle,
}
