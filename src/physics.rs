use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use heron::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
	pub collider: CollisionShape,
	pub rigid_body: RigidBody,
	pub velocity: Velocity,
	pub rotation_constraints: RotationConstraints,
}

impl From<EntityInstance> for ColliderBundle {
	fn from(entity_instance: EntityInstance) -> ColliderBundle {
		let rotation_constraints = RotationConstraints::lock();

		match entity_instance.identifier.as_ref() {
			"Player" => ColliderBundle {
				collider: CollisionShape::Sphere { radius: 8. },
				rigid_body: RigidBody::Dynamic,
				rotation_constraints,
				..Default::default()
			},
			"Alien" => ColliderBundle {
				collider: CollisionShape::Sphere { radius: 8. },
				rigid_body: RigidBody::KinematicVelocityBased,
				rotation_constraints,
				..Default::default()
			},
			"Pickup" => ColliderBundle {
				collider: CollisionShape::Sphere { radius: 16. },
				rigid_body: RigidBody::Static,
				rotation_constraints,
				..Default::default()
			},
			_ => ColliderBundle::default(),
		}
	}
}

impl From<IntGridCell> for ColliderBundle {
	fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
		let rotation_constraints = RotationConstraints::lock();

		match int_grid_cell.value {
			1 => ColliderBundle {
				collider: CollisionShape::Cuboid {
					half_extends: Vec3::new(8., 8., 0.),
					border_radius: None,
				},
				rigid_body: RigidBody::Static,
				rotation_constraints,
				..Default::default()
			},
			_ => ColliderBundle::default(),
		}
	}
}
