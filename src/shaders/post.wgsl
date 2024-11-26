struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput{
    let uv = vec2<f32>(f32(index >> 1u), f32(index & 1u)) * 2.0;
    let pos = vec3<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0);

    return VertexOutput (
        vec4<f32>(pos.x, pos.y, pos.z, 1.0),
        vec2<f32>(uv.x, uv.y),
    );
}

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
// @group(0) @binding(2) var<uniform> threshold: f32;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var kernel_x = array<i32, 9>(
        -1, 0, 1,
        -2, 0, 2,
        -1, 0, 1
    );

    var kernel_y = array<i32, 9>(
        -1, -2, -1,
        0,  0,  0,
        1,  2,  1
    );

    let texSize = vec2<f32>(textureDimensions(texture, 0));
    let step = vec2<f32>(1.0 / texSize.x, 1.0 / texSize.y);

    var sum_x = 0.0;
    var sum_y = 0.0;

    for (var y: i32 = -1; y <= 1; y = y + 1) {
        for (var x: i32 = -1; x <= 1; x = x + 1) {
            let texOffset = in.uv + vec2<f32>(f32(x) * step.x, f32(y) * step.y);
            let sample_rgb = textureSample(texture, texture_sampler, texOffset);
            let sample = sample_rgb.r + sample_rgb.g + sample_rgb.b;
            let index = (y + 1) * 3 + (x + 1);
            sum_x = sum_x + sample * f32(kernel_x[index]);
            sum_y = sum_y + sample * f32(kernel_y[index]);
        }
    }

    let edge = sqrt(sum_x * sum_x + sum_y * sum_y);

    // if (edge < 0.15) {
        return textureSample(texture, texture_sampler, in.uv);
    // }
    // return vec4<f32>(vec3<f32>(edge), 1.0);
}
