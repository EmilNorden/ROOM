struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

[[group(0), binding(0)]]
var t_diffuse: texture_2d<u32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;


[[group(1), binding(0)]]
var t_palette: texture_1d<u32>;

[[group(1), binding(1)]]
var s_palette: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let coords = vec2<i32>(i32(320.0 * in.tex_coords.x), i32(200.0 * in.tex_coords.y));
    let index = textureLoad(t_diffuse, coords, 0);

    let palette = textureLoad(t_palette, i32(index.r), 0);
    return vec4<f32>(f32(palette.r) / 255.0, f32(palette.g) / 255.0, f32(palette.b) / 255.0, 255.0);
}