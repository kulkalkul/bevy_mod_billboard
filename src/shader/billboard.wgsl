struct View {
    view_proj: mat4x4<f32>,
    inverse_view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    world_position: vec3<f32>,
    viewport: vec4<f32>,
};

struct Billboard {
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> view: View;

@group(1) @binding(0)
var<uniform> billboard: Billboard;

@group(2) @binding(0)
#ifdef VERTEX_TEXTURE_ARRAY
var billboard_texture: texture_2d_array<f32>;
#else
var billboard_texture: texture_2d<f32>;
#endif
@group(2) @binding(1)
var billboard_sampler: sampler;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
#ifdef VERTEX_COLOR
    @location(2) color: vec4<f32>,
#endif
#ifdef VERTEX_TEXTURE_ARRAY
    @location(3) array_index: i32,
#endif
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
#ifdef VERTEX_COLOR
    @location(1) color: vec4<f32>,
#endif
#ifdef VERTEX_TEXTURE_ARRAY
    @location(2) array_index: i32,
#endif
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let camera_right = normalize(vec3<f32>(view.view_proj.x.x, view.view_proj.y.x, view.view_proj.z.x));
    let camera_up = normalize(vec3<f32>(view.view_proj.x.y, view.view_proj.y.y, view.view_proj.z.y));

    let world_space = camera_right * vertex.position.x + camera_up * vertex.position.y;
    let position = view.view_proj * billboard.model * vec4<f32>(world_space, 1.0);

    var out: VertexOutput;
    out.position = position;
    out.uv = vertex.uv;
#ifdef VERTEX_COLOR
    out.color = vertex.color;
#endif
#ifdef VERTEX_TEXTURE_ARRAY
    out.array_index = vertex.array_index;
#endif

    return out;
}

struct Fragment {
    @location(0) uv: vec2<f32>,
#ifdef VERTEX_COLOR
    @location(1) color: vec4<f32>,
#endif
#ifdef VERTEX_TEXTURE_ARRAY
    @location(2) array_index: i32,
#endif
};

@fragment
fn fragment(fragment: Fragment) -> @location(0) vec4<f32> {
#ifdef VERTEX_TEXTURE_ARRAY
    let color = textureSample(billboard_texture, billboard_sampler, fragment.uv, fragment.array_index);
#else
    let color = textureSample(billboard_texture, billboard_sampler, fragment.uv);
#endif
#ifdef VERTEX_COLOR
    return color * fragment.color;
#else
    return color;
#endif
}