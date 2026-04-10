#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2], // new
}

pub fn load_glb(path: &str) -> (Vec<Vertex>, Vec<u32>) {
    let (doc, buffers, _) = gltf::import(path).expect("Failed to load glb");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for mesh in doc.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let positions = reader.read_positions().unwrap();
            let normals = reader.read_normals().unwrap();
            let uvs = reader.read_tex_coords(0).map(|tc| tc.into_f32()).unwrap();

            let base_index = vertices.len() as u32;

            for ((p, n), uv) in positions.zip(normals).zip(uvs) {
                vertices.push(Vertex {
                    position: p,
                    normal: n,
                    uv,
                });
            }

            if let Some(read_indices) = reader.read_indices() {
                indices.extend(read_indices.into_u32().map(|i| i + base_index));
            }
        }
    }

    (vertices, indices)
}
