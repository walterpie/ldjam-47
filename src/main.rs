use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::physics::*;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::ColliderBuilder;
use rapier3d::na::{Translation3, UnitQuaternion};

use navmesh::Navmesh;
use room::*;

pub mod navmesh;
pub mod room;

fn main() {
    App::build()
        .add_plugin(RapierPhysicsPlugin)
        .add_default_plugins()
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .add_system(room_system.system())
        .run();
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
    let mut rm_a = None;
    let mut rm_b = None;
    commands
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents::default())
        .with(FlyCamera::default())
        .spawn(RoomBundle {
            name: Name::new("Test room".to_string()),
            body: RigidBodyBuilder::new_static(),
            collider: ColliderBuilder::trimesh(navmesh.vertices, navmesh.indices),
        })
        .with_bundle(PbrComponents {
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            mesh: handle,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .for_current_entity(|e| rm_a = Some(e));

    let navmesh = Navmesh::from(mesh);
    commands
        .spawn(RoomBundle {
            name: Name::new("Test room*".to_string()),
            body: RigidBodyBuilder::new_static(),
            collider: ColliderBuilder::trimesh(navmesh.vertices, navmesh.indices),
        })
        .with_bundle(PbrComponents {
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            mesh: handle,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .for_current_entity(|e| rm_b = Some(e));

    let rm_a = rm_a.unwrap();
    let rm_b = rm_b.unwrap();

    commands.insert_one(
        rm_a,
        Edges::new().add(
            Room::new(rm_b)
                .origin(Translation3::new(-8.0, 0.0, 0.0))
                .rotation(UnitQuaternion::from_euler_angles(
                    0.0,
                    90.0_f32.to_radians(),
                    0.0,
                )),
        ),
    );
    commands.insert_one(
        rm_b,
        Edges::new().add(
            Room::new(rm_a)
                .origin(Translation3::new(-8.0, 0.0, 0.0))
                .rotation(UnitQuaternion::from_euler_angles(
                    0.0,
                    90.0_f32.to_radians(),
                    0.0,
                )),
        ),
    );

    commands.insert_resource(CurrentRoom { entity: rm_a });
}

pub fn room_system(
    mut commands: Commands,
    current: Res<CurrentRoom>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<Without<ActiveRoom, (&Edges, &Name)>>,
    connected: Query<(&RigidBodyHandleComponent, Mut<Draw>)>,
) {
    let current = current.entity;
    if let Ok(name) = query.get::<Name>(current) {
        let mut draw = connected.get_mut::<Draw>(current).unwrap();
        draw.is_visible = true;
        commands.insert_one(current, ActiveRoom);
        eprintln!("* Entering {:?}", name.get());
    }

    if let Ok(edges) = query.get::<Edges>(current) {
        for edge in edges.iter() {
            let body = connected
                .get::<RigidBodyHandleComponent>(edge.entity())
                .unwrap();
            bodies
                .get_mut(body.handle())
                .unwrap()
                .set_position(edge.isometry());
            let mut draw = connected.get_mut::<Draw>(edge.entity()).unwrap();
            draw.is_visible = true;
        }
    }
}
