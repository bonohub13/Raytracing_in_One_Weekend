struct Camera {
    resolution: vec2<f32>,
    _pad: vec2<f32>,

    center: vec4<f32>,
    pixel00_loc: vec4<f32>,
    pixel_delta_u: vec4<f32>,
    pixel_delta_v: vec4<f32>,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

@group(0) @binding(2)
var<uniform> cam : Camera;

@group(0) @binding(1)
var ray_image : texture_storage_2d<rgba16float, write>;

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) gid: vec3<u32>,
) {
    if ((gid.x >= u32(cam.resolution.x)) || (gid.y >= u32(cam.resolution.y))) {
        return;
    }

    var r: Ray;

    let x = f32(gid.x);
    let y = f32(gid.y);
    let pixel_center = cam.pixel00_loc.xyz + (x * cam.pixel_delta_u.xyz) + (y * cam.pixel_delta_v.xyz);

    r.origin = cam.center.xyz;
    r.direction = pixel_center - cam.center.xyz;

    textureStore(
        ray_image,
        vec2<i32>(gid.xy),
        vec4<f32>(x / (cam.resolution.x - 1), y / (cam.resolution.y - 1), 0, 1)
    );
}

/* Vec3 functions */
fn length(v: vec3<f32>) -> f32 {
    return sqrt(length_squared(v));
}

fn length_squared(v: vec3<f32>) -> f32 {
    return dot(v, v);
}

fn unit_vector(v: vec3<f32>) -> vec3<f32> {
    return v / length(v);
}

/* Ray functions */
fn ray_color(r: Ray) -> vec4<f32> {
    return vec4<f32>(0, 0, 0, 1);
}

fn ray_at(r: Ray, t: f32) -> vec3<f32> {
    return r.origin + (t * r.direction);
}
