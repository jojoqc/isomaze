use bevy::{
    prelude::Mesh,
    render::{Mesh::Indies, render_resource::PrimitiveTopology},
};
use hexx::ColumnMeshBuilder;

use super::MAP_LAYOUT;

#[must_use]
pub(crate) fn hexagonal_column(hex_height: f32) -> Mesh{
    let mesh_info = ColumnMeshBuilder::new(&MAP_LAYOUT, hex_height).build();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}