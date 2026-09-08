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
var billboard_texture: texture_2d<f32>;
@group(2) @binding(1)
var billboard_sampler: sampler;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
#ifdef VERTEX_COLOR
    @location(2) color: vec4<f32>,
#endif
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
#ifdef VERTEX_COLOR
    @location(1) color: vec4<f32>,
#endif
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
#ifdef LOCK_ROTATION
    let vertex_position = vec4<f32>(-vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let position = view.view_proj * billboard.model * vertex_position;
#else
    let camera_right = normalize(vec3<f32>(view.view_proj[0][0], view.view_proj[1][0], view.view_proj[2][0]));
#ifdef LOCK_Y
    let camera_up = vec3<f32>(0.0, 1.0, 0.0);
#else
    let camera_up = normalize(vec3<f32>(view.view_proj[0][1], view.view_proj[1][1], view.view_proj[2][1]));
#endif

    let world_space = camera_right * vertex.position.x + camera_up * vertex.position.y;
    let position = view.view_proj * billboard.model * vec4<f32>(world_space, 1.0);
#endif

    var out: VertexOutput;
    out.position = position;
    out.uv = vertex.uv;
#ifdef VERTEX_COLOR
    out.color = vertex.color;
#endif

    return out;
}

struct Fragment {
    @location(0) uv: vec2<f32>,
#ifdef VERTEX_COLOR
    @location(1) color: vec4<f32>,
#endif
};

@fragment
fn fragment(fragment: Fragment) -> @location(0) vec4<f32> {
    let color = textureSample(billboard_texture, billboard_sampler, fragment.uv);
#ifdef VERTEX_COLOR
    return color * fragment.color;
#else
    return color;
#endif
}
