struct Camera {
    resolution: vec2<f32>,
    samples_per_pixel: u32,
    max_depth: u32,
    center: vec4<f32>,
    pixel00_loc: vec4<f32>,
    pixel_delta_u: vec4<f32>,
    pixel_delta_v: vec4<f32>,
    pixel_samples_scale: f32,
    reserved: vec3<f32>,
}

struct Scene {
    objects: array<HitObject>,
}

struct Material {
    albedo: vec3<f32>,
    fuzz: f32,
    _pad: vec3<f32>,
    id: u32,
}

struct HitObject {
    center: vec3<f32>,
    radius: f32,
    _pad: vec3<f32>,
    id: u32,
    mat: Material,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
    pixel: u32,
}

struct Interval {
    min: f32,
    max: f32,
}

struct HitRecord {
    p: vec3<f32>,
    t: f32,
    normal: vec3<f32>,
    front_face: bool,
    mat: Material,
    hit: bool,
}

struct ScatterResult {
    attenuation: vec3<f32>,
    is_scatter: bool,
    scattered: Ray,
}

// Hittable object ID
const SPHERE_ID: u32 = 0x0001;

// Material object ID
const LAMBTERTIAN_ID: u32 = 0x1000;
const METAL_ID: u32 = 0x2000;

// Constant parameters
const INFINITY: f32 = 1e30;
const PI: f32 = 3.1415926535897932385;
const EMPTY: Interval = Interval(INFINITY, -INFINITY);
const UNIVERSE: Interval = Interval(-INFINITY, INFINITY);

@group(0) @binding(0)
var<uniform> cam : Camera;

@group(0) @binding(1)
var<storage, read> scene: Scene;

@group(0) @binding(2)
var<storage, read> rand: array<vec3<f32>>;

@group(0) @binding(3)
var ray_image : texture_storage_2d<rgba32float, write>;

@compute
@workgroup_size(8, 8, 1)
fn main(
    @builtin(global_invocation_id) gid: vec3<u32>,
) {
    if (gid.x >= u32(cam.resolution.x)) || (gid.y >= u32(cam.resolution.y)) {
        return;
    }

    var r: Ray;
    var pixel_color: vec3<f32> = vec3<f32>(0);
    let x = f32(gid.x);
    let y = f32(gid.y);
    let pixel_id = gid.x * gid.y + gid.x;

    for (var sample: u32 = 0; sample < u32(cam.samples_per_pixel); sample++) {
        r = get_ray(x, y, sample_square(sample).xy);
        r.pixel = pixel_id;
        pixel_color += ray_color(r);
    }

    textureStore(
        ray_image,
        vec2<i32>(gid.xy),
        write_color(cam.pixel_samples_scale * pixel_color)
    );
}

/* Vec3 functions */
fn length(v: vec3<f32>) -> f32 {
    return sqrt(length_squared(v));
}

fn length_squared(v: vec3<f32>) -> f32 {
    return dot(v, v);
}

fn near_zero(v: vec3<f32>) -> bool {
    const DELTA: f32 = 1e-8;
    let abs_v = abs(v);

    return (abs_v.x < DELTA) && (abs_v.y < DELTA) && (abs_v.z < DELTA);
}

fn random_unit_vector(id: u32) -> vec3<f32> {
    var p: vec3<f32>;
    var lensq: f32;
    var i: u32 = id;

    loop {
        p = random_in_range(rand[i%arrayLength(&rand)], -1, 1);
        lensq = length_squared(p);
        if (1e-160 < lensq) && (lensq <= 1) {
            return normalize(p);
        }
        i++;
    }

    return vec3<f32>(0);
}

fn random_on_hemisphere(normal: vec3<f32>, id: u32) -> vec3<f32> {
    var on_unit_sphere: vec3<f32> = random_unit_vector(id);

    if dot(on_unit_sphere, normal) <= 0 {
        on_unit_sphere = -on_unit_sphere;
    }

    return on_unit_sphere;
}

fn reflect(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2 * dot(v, n) * n;
}

fn random_in_range(v: vec3<f32>, min: f32, max: f32) -> vec3<f32> {
    let offset = vec3<f32>(min);

    return offset + (max - min) * v;
}

fn sample_square(sample: u32) -> vec3<f32> {
    const OFFSET: vec2<f32> = vec2<f32>(0.5, 0.5);

    return vec3<f32>(rand[sample % arrayLength(&rand)].xy - OFFSET, 0);
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    var out: f32 = 0;

    if linear_component > 0 {
        out = sqrt(linear_component);
    }

    return out;
}

fn write_color(color: vec3<f32>) -> vec4<f32> {
    const intensity: Interval = Interval(0.000, 0.999);

    var out: vec4<f32>;

    out.x = interval_clamp(intensity, linear_to_gamma(color.x));
    out.y = interval_clamp(intensity, linear_to_gamma(color.y));
    out.z = interval_clamp(intensity, linear_to_gamma(color.z));
    out.w = 1;

    return out;
}

/* Ray functions */
fn get_ray(i: f32, j: f32, offset: vec2<f32>) -> Ray {
    var out: Ray;
    let pixel_center = cam.pixel00_loc.xyz
        + ((i + offset.x) * cam.pixel_delta_u.xyz)
        + ((j + offset.y) * cam.pixel_delta_v.xyz);

    out.origin = cam.center.xyz;
    out.direction = pixel_center - cam.center.xyz;
    out.pixel = 0;

    return out;
}

fn ray_color(r: Ray) -> vec3<f32> {
    const WHITE: vec3<f32> = vec3<f32>(1);
    const BLACK: vec3<f32> = vec3<f32>(0);
    const BLUE: vec3<f32> = vec3<f32>(0.5, 0.7, 1);
    const RANGE: Interval = Interval(1e-3, INFINITY);

    var rec: HitRecord;
    var ray: Ray = r;
    var depth: u32 = 0;
    var color: vec3<f32> = WHITE;
    let max_depth: u32 = cam.max_depth;

    for (; depth < max_depth; depth++) {
        rec = hit(ray, RANGE);
        if rec.hit {
            let result = scatter(rec.mat, ray, rec);
            if result.is_scatter {
                ray = result.scattered;
                color *= result.attenuation;
                continue;
            } else {
                return BLACK;
            }
        }

        let unit_direction = normalize(ray.direction);
        let a = 0.5 * (unit_direction.y + 1);
        let sky_color = (1 - a) * WHITE + a * BLUE;

        return color * sky_color;
    }

    return BLACK;
}

fn ray_at(r: Ray, t: f32) -> vec3<f32> {
    return r.origin + (t * r.direction);
}

/* Interval functions */
fn interval_size(interval: Interval) -> f32 {
    return interval.max - interval.min;
}

fn interval_contains(interval: Interval, x: f32) -> bool {
    return (interval.min <= x) && (x <= interval.max);
}

fn interval_surrounds(interval: Interval, x: f32) -> bool {
    return (interval.min < x) && (x < interval.max);
}

fn interval_clamp(interval: Interval, x: f32) -> f32 {
    var clamp: f32 = x;

    if x < interval.min {
        clamp = interval.min;
    } else if interval.max < x {
        clamp = interval.max;
    }

    return clamp;
}

/* HitRecord functions */
fn hit_record_set_face_normal(
    rec: HitRecord,
    r: Ray,
    outward_normal: vec3<f32>,
) -> HitRecord {
    var out: HitRecord = rec;

    out.front_face = dot(r.direction, outward_normal) < 0;
    if out.front_face {
        out.normal = outward_normal;
    } else {
        out.normal = -outward_normal;
    }

    return out;
}

/* Hittable */
fn hit(r: Ray, ray_t: Interval) -> HitRecord {
    var rec: HitRecord;
    var tmp: HitRecord;
    var obj: HitObject;
    var closest_so_far: Interval = ray_t;

    rec.hit = false;
    for (var i: u32 = 0; i < arrayLength(&scene.objects); i++) {
        obj = scene.objects[i];
        tmp = hit_sphere(obj, r, closest_so_far);
        if tmp.hit {
            closest_so_far.max = tmp.t;
            rec = tmp;
        }
    }


    return rec;
}

fn hit_sphere(sphere: HitObject, r: Ray, ray_t: Interval) -> HitRecord {
    var out: HitRecord;

    let oc = sphere.center - r.origin;
    let a = dot(r.direction, r.direction);
    let h = dot(r.direction, oc);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = h * h - a * c;

    if discriminant < 0 {
        out.hit = false;
    } else {
        var root: f32;

        out.hit = true;
        let sqrtd = sqrt(discriminant);
        root = (h - sqrtd) / a;
        if !interval_surrounds(ray_t, root) {
            root = (h + sqrtd) / a;
            if !interval_surrounds(ray_t, root) {
                out.hit = false;
            }
        }

        if out.hit {
            out.t = root;
            out.p = ray_at(r, out.t);
            let outward_normal = (out.p - sphere.center) / sphere.radius;
            out = hit_record_set_face_normal(out, r, outward_normal);
            out.mat = sphere.mat;
        }
    }

    return out;
}

/* Material */
fn scatter(mat: Material, r_in: Ray, rec: HitRecord) -> ScatterResult {
    var out: ScatterResult = ScatterResult(vec3<f32>(0), false, Ray(vec3<f32>(0), vec3<f32>(0), r_in.pixel));

    switch mat.id {
        case LAMBTERTIAN_ID: {
            let scatter_direction = rec.normal + random_unit_vector(r_in.pixel);

            out.scattered.direction = scatter_direction;
            if near_zero(scatter_direction) {
                out.scattered.direction = rec.normal;
            }
            out.scattered.origin = rec.p;
            out.attenuation = mat.albedo;
            out.is_scatter = true;
        }
        case METAL_ID: {
            var reflected: vec3<f32> = reflect(r_in.direction, rec.normal);

            reflected = normalize(reflected) + (mat.fuzz * random_unit_vector(r_in.pixel));
            out.scattered.origin = rec.p;
            out.scattered.direction = reflected;
            out.attenuation = mat.albedo;
            out.is_scatter = dot(out.scattered.direction, rec.normal) > 0;
        }
        default: {
            // Do nothing
        }
    }

    return out;
}

/* utility function */
fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180;
}
