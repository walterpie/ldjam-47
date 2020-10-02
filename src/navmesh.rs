use std::convert::TryFrom;

use bevy::render::{mesh::*, pipeline::PrimitiveTopology};
use itertools::Itertools;
use rapier3d::math::Point;

#[derive(Debug, Clone)]
pub struct Navmesh {
    pub vertices: Vec<Point<f32>>,
    pub indices: Vec<Point<u32>>,
}

impl<'a> From<&'a Mesh> for Navmesh {
    fn from(mesh: &'a Mesh) -> Self {
        debug_assert!(matches!(
            mesh.primitive_topology,
            PrimitiveTopology::TriangleList | PrimitiveTopology::TriangleStrip
        ));

        let indices;
        let mut vertices = Vec::new();
        for attr in &mesh.attributes {
            if attr.name.as_ref() == VertexAttribute::POSITION {
                match &attr.values {
                    VertexAttributeValues::Float3(vec) => {
                        vertices = vec.iter().copied().map(From::from).collect()
                    }
                    _ => panic!("{} is not a Float3", VertexAttribute::POSITION),
                }
                break;
            }
        }

        match &mesh.indices {
            Some(ind) => match mesh.primitive_topology {
                PrimitiveTopology::TriangleList => {
                    indices = ind
                        .chunks_exact(3)
                        .map(|chunk| From::from(<[u32; 3]>::try_from(chunk).unwrap()))
                        .collect();
                }
                PrimitiveTopology::TriangleStrip => {
                    indices = ind
                        .iter()
                        .zip(ind.iter().skip(1))
                        .zip(ind.iter().skip(2))
                        .map(|((i0, i1), i2)| From::from([*i0, *i1, *i2]))
                        .collect();
                }
                _ => unreachable!(),
            },
            None => {
                let ind = 0..vertices.len() as u32;
                indices = ind
                    .chunks(3)
                    .into_iter()
                    .flat_map(|mut chunk| {
                        let i0 = chunk.next()?;
                        let i1 = chunk.next()?;
                        let i2 = chunk.next()?;
                        Some(From::from([i0, i1, i2]))
                    })
                    .collect();
            }
        }

        Navmesh { vertices, indices }
    }
}
