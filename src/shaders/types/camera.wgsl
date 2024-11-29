struct Camera {
    view: mat4x4<f32>,
    position: vec4<f32>,
    direction: vec4<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
}
