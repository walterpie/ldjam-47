use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::physics::*;
use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::geometry::ColliderBuilder;

use navmesh::Navmesh;

pub mod navmesh;

fn main() {
    App::build()
        .add_plugin(RapierPhysicsPlugin)
        .add_default_plugins()
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

#[derive(Bundle)]
struct RoomBundle {
    body: RigidBodyBuilder,
    collider: ColliderBuilder,
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handle: Handle<Mesh> = assets
        .load_sync(&mut meshes, "assets/navmesh/rm_init.gltf")
        .unwrap();
    let mesh = meshes.get(&handle).unwrap();
    let navmesh = Navmesh::from(mesh);
    commands
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents::default())
        .with(FlyCamera::default())
        .spawn(RoomBundle {
            body: RigidBodyBuilder::new_static(),
            collider: ColliderBuilder::trimesh(navmesh.vertices, navmesh.indices),
        })
        .with_bundle(PbrComponents {
            mesh: handle,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        });
}
