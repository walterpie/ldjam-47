use bevy::prelude::*;
use bevy::render::{mesh::*, pipeline::PrimitiveTopology};
use hashbrown::HashSet;

use super::*;

pub fn generate(w: f32, h: f32, d: f32, doors: &HashSet<Door>) -> Mesh {
    let w = w / 2.0;
    let d = d / 2.0;
    let mut attributes = Vec::new();
    let mut indices = Vec::new();

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    let mut n = 0;

    // floor
    positions.extend(&[[w, 0.0, -d], [-w, 0.0, -d], [-w, 0.0, d], [w, 0.0, d]]);
    normals.extend(&[
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ]);
    uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
    indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
    n = positions.len() as u32;

    // north
    if doors.contains(&Door::North) {
        positions.extend(&[[-w, 0.0, -d], [-0.5, 0.0, -d], [-0.5, h, -d], [-w, h, -d]]);
        normals.extend(&[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[0.5, 0.0, -d], [w, 0.0, -d], [w, h, -d], [0.5, h, -d]]);
        normals.extend(&[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[-0.5, 1.6, -d], [0.5, 1.6, -d], [0.5, h, -d], [-0.5, h, -d]]);
        normals.extend(&[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;
    } else {
        positions.extend(&[[-w, 0.0, -d], [w, 0.0, -d], [w, h, -d], [-w, h, -d]]);
        normals.extend(&[
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;
    }

    // south
    if doors.contains(&Door::South) {
        positions.extend(&[[-0.5, 0.0, d], [-w, 0.0, d], [-w, h, d], [-0.5, h, d]]);
        normals.extend(&[
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[w, 0.0, d], [0.5, 0.0, d], [0.5, h, d], [w, h, d]]);
        normals.extend(&[
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[0.5, 1.6, d], [-0.5, 1.6, d], [-0.5, h, d], [0.5, h, d]]);
        normals.extend(&[
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;
    } else {
        positions.extend(&[[w, 0.0, d], [-w, 0.0, d], [-w, h, d], [w, h, d]]);
        normals.extend(&[
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;
    }

    // west
    if doors.contains(&Door::West) {
        positions.extend(&[[w, 0.0, -d], [w, 0.0, -0.5], [w, h, -0.5], [w, h, -d]]);
        normals.extend(&[
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[w, 0.0, 0.5], [w, 0.0, d], [w, h, d], [w, h, 0.5]]);
        normals.extend(&[
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[w, 1.6, -0.5], [w, 1.6, 0.5], [w, h, 0.5], [w, h, -0.5]]);
        normals.extend(&[
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;
    } else {
        positions.extend(&[[w, 0.0, -d], [w, 0.0, d], [w, h, d], [w, h, -d]]);
        normals.extend(&[
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;
    }

    // east
    if doors.contains(&Door::East) {
        positions.extend(&[[-w, 0.0, -0.5], [-w, 0.0, -d], [-w, h, -d], [-w, h, -0.5]]);
        normals.extend(&[
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[-w, 0.0, d], [-w, 0.0, 0.5], [-w, h, 0.5], [-w, h, d]]);
        normals.extend(&[
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
        n = positions.len() as u32;

        positions.extend(&[[-w, 1.6, 0.5], [-w, 1.6, -0.5], [-w, h, -0.5], [-w, h, 0.5]]);
        normals.extend(&[
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
    } else {
        positions.extend(&[[-w, 0.0, d], [-w, 0.0, -d], [-w, h, -d], [-w, h, d]]);
        normals.extend(&[
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ]);
        uvs.extend(&[[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);
        indices.extend(&[n + 0, n + 1, n + 2, n + 2, n + 3, n + 0]);
    }

    attributes.push(VertexAttribute::position(positions));
    attributes.push(VertexAttribute::normal(normals));
    attributes.push(VertexAttribute::uv(uvs));

    Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes,
        indices: Some(indices),
    }
}
