struct Camera {
    vp: mat4x4<f32>,
    cam_pos: vec3<f32>,
    _pad: f32, // alignment fix
};

@group(0) @binding(0)
var<uniform> camera: Camera;

// ----------------------------

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct InstanceInput {
    @location(3) instance_pos: vec3<f32>,
    @location(4) scale: f32,
    @location(5) rotation: vec4<f32>,
};

struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

// ----------------------------
// Quaternion rotation
fn quat_rotate(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let t = 2.0 * cross(q.xyz, v);
    return v + q.w * t + cross(q.xyz, t);
}

// ----------------------------

@vertex
fn vs_main(v: VertexInput, i: InstanceInput) -> VSOut {
    let rotated = quat_rotate(i.rotation, v.position);
    let world_pos = rotated * i.scale + i.instance_pos;

    var out: VSOut;
    out.pos = camera.vp * vec4<f32>(world_pos, 1.0);

    out.world_pos = world_pos;
    out.normal = normalize(quat_rotate(i.rotation, v.normal));

    return out;
}

// ----------------------------

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    let light_pos = vec3(20.0, 30.0, 20.0);
    let light_color = vec3(1.0, 1.0, 1.0);

    let normal = normalize(in.normal);
    let light_dir = normalize(light_pos - in.world_pos);
    let view_dir = normalize(camera.cam_pos - in.world_pos);

    // -------- Diffuse --------
    let diff = max(dot(normal, light_dir), 0.0);

    // -------- Specular (Blinn-Phong) --------
    let halfway = normalize(light_dir + view_dir);
    let spec = pow(max(dot(normal, halfway), 0.0), 64.0); // sharper highlight

    // -------- Ambient --------
    let ambient = 0.15;

    // -------- Material --------
    let base_color = vec3(0.4, 0.7, 1.0);

    let lighting = ambient + diff;
    var color = base_color * lighting + spec * light_color;

    // -------- Gamma correction --------
    color = pow(color, vec3(1.0 / 2.2));

    return vec4(color, 1.0);
}
