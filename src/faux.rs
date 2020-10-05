use bevy::prelude::*;

pub fn spawn(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let faux_street = assets
        .load_sync(meshes, "assets/mesh/faux_street.gltf")
        .unwrap();
    let faux_soy_milk_store = assets
        .load_sync(meshes, "assets/mesh/faux_soy_milk_store.gltf")
        .unwrap();
    commands
        .spawn(PbrComponents {
            mesh: faux_street,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .spawn(PbrComponents {
            transform: Transform::from_translation(Vec3::new(-15.0, 0.0, 0.0)),
            mesh: faux_soy_milk_store,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        });
}
