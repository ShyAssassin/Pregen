struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) world_position: vec3<f32>,
}

struct Camera {
    view: mat4x4<f32>,
    position: vec3<f32>,
    direction: vec3<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
}

struct LightingUniform {
    color: vec3<f32>,
    position: vec3<f32>,
    intensity: f32,
}

@group(0) @binding(0)
var<uniform> uTime: f32;

@group(0) @binding(7)
var<uniform> uLights: array<LightingUniform, 32>;

@group(1) @binding(0)
var<uniform> uCamera: Camera;

@group(2) @binding(0)
var<uniform> uModel: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let world_position = uModel * vec4<f32>(in.position, 1.0);
    let clip_position = uCamera.projection * uCamera.view * world_position;

    let world_normal = normalize(mat3x3<f32>(uModel[0].xyz, uModel[1].xyz, uModel[2].xyz) * in.normal);

    return VertexOutput(
        clip_position,
        world_normal,
        in.color,
        in.uv,
        world_position.xyz
    );
}

@group(3) @binding(0)
var tAlbedo: texture_2d<f32>;
@group(3) @binding(1)
var sAlbedo: sampler;

@group(3) @binding(2)
var tNormal: texture_2d<f32>;
@group(3) @binding(3)
var sNormal: sampler;

@group(3) @binding(4)
var tAmbient: texture_2d<f32>;
@group(3) @binding(5)
var sAmbient: sampler;

fn compute_tbn(world_pos: vec3<f32>, normal: vec3<f32>, uv: vec2<f32>) -> mat3x3<f32> {
    let dp_dx = dpdx(world_pos);
    let dp_dy = dpdy(world_pos);
    let duv_dx = dpdx(uv);
    let duv_dy = dpdy(uv);

    let denom = duv_dx.x * duv_dy.y - duv_dy.x * duv_dx.y;
    let inv_det = 1.0 / max(denom, 1e-8);

    let tangent = normalize((dp_dx * duv_dy.y - dp_dy * duv_dx.y) * inv_det);
    let orthogonal_tangent = normalize(tangent - normal * dot(normal, tangent));
    let bitangent = cross(normal, orthogonal_tangent);

    return mat3x3<f32>(orthogonal_tangent, bitangent, normal);
}

fn aces_tonemap(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(in.uv.x, 1.0 - in.uv.y);

    var albedo = textureSample(tAlbedo, sAlbedo, uv);
    if (albedo.a < 0.1) {
        discard;
    }

    let ambient_occlusion = textureSample(tAmbient, sAmbient, uv).rgb;

    let normal_sample = textureSample(tNormal, sNormal, uv).rgb * 2.0 - 1.0;
    let geo_normal = normalize(in.normal);
    let tbn = compute_tbn(in.world_position, geo_normal, uv);
    let world_normal = normalize(tbn * normal_sample);

    let view_dir = normalize(uCamera.position - in.world_position);

    // Small constant so unlit areas aren't pure black, modulated by the
    // ambient/occlusion texture for a bit of surface detail in shadow
    var color = albedo.rgb * ambient_occlusion * 0.08;

    let shininess = 27.25;
    let specular_strength = 0.5;

    for (var i: u32 = 0; i < 32; i = i + 1) {
        let light = uLights[i];
        let to_light = light.position - in.world_position;
        let distance = length(to_light) / 1.65;
        let light_dir = to_light / distance;

        // Inverse-square falloff, clamped so very close lights don't
        // divide-by-near-zero and blow out to infinity
        let attenuation = 1.0 / max(distance * distance, 0.01);
        let radiance = light.color * light.intensity * attenuation;

        let n_dot_l = max(dot(world_normal, light_dir), 0.0);
        let diffuse = albedo.rgb * n_dot_l;

        let half_vec = normalize(light_dir + view_dir);
        let spec_angle = max(dot(world_normal, half_vec), 0.0);
        let specular = specular_strength * pow(spec_angle, shininess) * ambient_occlusion;

        color += (diffuse + specular) * radiance;
    }

    let final_color = aces_tonemap(color);
    return vec4<f32>(final_color, albedo.a);
}
