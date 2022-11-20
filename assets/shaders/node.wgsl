struct NodeMaterial {
    highlight: u32,
    color: vec4<f32>,
    color_border: vec4<f32>,
    color_title: vec4<f32>,
    size: vec2<f32>,
    border_thickness: f32,
    title_height: f32,
};

@group(1) @binding(0)
var<uniform> material: NodeMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let pos = uv * material.size;

    if bool(material.highlight) && (
        pos.x < material.border_thickness
        || pos.y <= material.border_thickness
        || pos.x >= material.size.x - material.border_thickness
        || pos.y >= material.size.y - material.border_thickness
    ) {
        return material.color_border;
    }

    if pos.y < material.title_height {
        return material.color_title;
    }

    return material.color;
}
