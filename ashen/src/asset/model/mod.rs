mod dat;

use dat::{
    frame::{ModelFrame, ModelSpecs},
    header::ModelHeader,
    sequence::ModelSequence,
    triangle::{ModelTriangle, TextureDimensions},
};

use super::{
    Parser,
    texture::{Texture, TextureSize},
};
use crate::utils::nom::*;

pub struct Model {
    pub texture: Texture,
    pub triangles: Vec<ModelTriangle>,
    pub sequences: Vec<ModelSequence>,
    pub frames: Vec<ModelFrame>,
}

impl Parser for Model {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (_, header) = ModelHeader::parser(())(input)?;

            let (_, triangles) = multi::count!(
                ModelTriangle::parser(TextureDimensions {
                    width: header.texture_width,
                    height: header.texture_height
                }),
                header.triangle_count as usize
            )(&input[header.offset_triangles as usize..])?;

            let (_, texture) = Texture::parser(TextureSize {
                width: header.texture_width as usize,
                height: header.texture_height as usize,
            })(&input[header.offset_texture as usize..])?;

            let (_, sequences) = multi::count!(
                ModelSequence::parser(input),
                header.sequence_count as usize
            )(&input[header.offset_sequences as usize..])?;

            let (_, frames) = multi::count!(
                ModelFrame::parser(ModelSpecs {
                    vertex_count: header.vertex_count,
                    triangle_count: header.triangle_count,
                    frame_size: header.frame_size
                }),
                header.frame_count as usize
            )(&input[header.offset_frames as usize..])?;

            Ok((
                &[],
                Self {
                    texture,
                    triangles,
                    sequences,
                    frames,
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::LazyCell,
        collections::HashMap,
        fmt::{Display, Formatter},
    };

    use itertools::Itertools;

    use super::{
        dat::{frame::ModelVertex, triangle::ModelPoint},
        *,
    };
    use crate::{
        asset::color_map::{Color, ColorMap, PaletteTexture},
        utils::test::*,
    };

    const COLOR_MAPS: LazyCell<HashMap<&'static str, Vec<u8>>> = LazyCell::new(|| {
        HashMap::from([
            ("creature", deflated_file!("01.dat")),
            ("ghost-creature", deflated_file!("03.dat")),
            ("pickup", deflated_file!("04.dat")),
        ])
    });

    const MODELS: LazyCell<HashMap<&'static str, (&'static str, Vec<u8>)>> = LazyCell::new(|| {
        HashMap::from([
            ("aquagore", ("creature", deflated_file!("0A-deflated.dat"))),
            ("broodmaw", ("creature", deflated_file!("0B-deflated.dat"))),
            (
                "cryptcrawler",
                ("creature", deflated_file!("0C-deflated.dat")),
            ),
            (
                "firedeacon",
                ("creature", deflated_file!("0D-deflated.dat")),
            ),
            ("hunter", ("creature", deflated_file!("0E-deflated.dat"))),
            (
                "psistalker",
                ("creature", deflated_file!("0F-deflated.dat")),
            ),
            (
                "stormfluke",
                ("creature", deflated_file!("10-deflated.dat")),
            ),
            ("tentacle", ("creature", deflated_file!("11-deflated.dat"))),
            ("wraith", ("ghost-creature", deflated_file!("12-deflated.dat"))),
            (
                "prime-entity",
                ("pickup", deflated_file!("13-deflated.dat")),
            ),
            // TODO(nenikitov): This model doesn't look good with any palette
            ("player-model", ("creature", deflated_file!("14-deflated.dat"))),
            ("vanessa", ("pickup", deflated_file!("15-deflated.dat"))),
            ("rocket", ("pickup", deflated_file!("16-deflated.dat"))),
            ("grenade", ("pickup", deflated_file!("17-deflated.dat"))),
            ("fx-blast", ("pickup", deflated_file!("18-deflated.dat"))),
            (
                "aquagore-shot",
                ("pickup", deflated_file!("19-deflated.dat")),
            ),
            (
                "broodmaw-shot",
                ("pickup", deflated_file!("1A-deflated.dat")),
            ),
            (
                "cryptcrawler-shot",
                ("pickup", deflated_file!("1B-deflated.dat")),
            ),
            (
                "firedeacon-shot",
                ("pickup", deflated_file!("1C-deflated.dat")),
            ),
            (
                "gib-generic-1",
                ("pickup", deflated_file!("1D-deflated.dat")),
            ),
            (
                "gib-generic-2",
                ("pickup", deflated_file!("1E-deflated.dat")),
            ),
            (
                "gib-generic-3",
                ("pickup", deflated_file!("1F-deflated.dat")),
            ),
            (
                "blood-generic-1",
                ("pickup", deflated_file!("20-deflated.dat")),
            ),
            ("charles", ("pickup", deflated_file!("21-deflated.dat"))),
            (
                "human-gib-generic-1",
                ("pickup", deflated_file!("22-deflated.dat")),
            ),
            (
                "human-gib-generic-2",
                ("pickup", deflated_file!("23-deflated.dat")),
            ),
            (
                "human-gib-generic-3",
                ("pickup", deflated_file!("24-deflated.dat")),
            ),
            (
                "pickup-ammo-pistol",
                ("pickup", deflated_file!("25-deflated.dat")),
            ),
            (
                "pickup-ammo-double-pistol",
                ("pickup", deflated_file!("26-deflated.dat")),
            ),
            (
                "pickup-ammo-shotgun",
                ("pickup", deflated_file!("27-deflated.dat")),
            ),
            (
                "pickup-ammo-machinegun",
                ("pickup", deflated_file!("28-deflated.dat")),
            ),
            (
                "pickup-ammo-sniper",
                ("pickup", deflated_file!("29-deflated.dat")),
            ),
            (
                "pickup-ammo-grenade",
                ("pickup", deflated_file!("2A-deflated.dat")),
            ),
            (
                "pickup-ammo-rocket",
                ("pickup", deflated_file!("2B-deflated.dat")),
            ),
            (
                "pickup-ammo-gatlinggun",
                ("pickup", deflated_file!("2C-deflated.dat")),
            ),
            (
                "pickup-weapon-pistol",
                ("pickup", deflated_file!("2D-deflated.dat")),
            ),
            (
                "pickup-weapon-double-pistol",
                ("pickup", deflated_file!("2E-deflated.dat")),
            ),
            (
                "pickup-weapon-shotgun",
                ("pickup", deflated_file!("2F-deflated.dat")),
            ),
            (
                "pickup-weapon-machinegun",
                ("pickup", deflated_file!("30-deflated.dat")),
            ),
            (
                "pickup-weapon-sniper",
                ("pickup", deflated_file!("31-deflated.dat")),
            ),
            (
                "pickup-weapon-grenade",
                ("pickup", deflated_file!("32-deflated.dat")),
            ),
            (
                "pickup-weapon-gatlinggun",
                ("pickup", deflated_file!("33-deflated.dat")),
            ),
            (
                "pickup-weapon-shockwave",
                ("pickup", deflated_file!("34-deflated.dat")),
            ),
            (
                "pickup-ghost-vision",
                ("pickup", deflated_file!("35-deflated.dat")),
            ),
            (
                "pickup-focitalisman",
                ("pickup", deflated_file!("36-deflated.dat")),
            ),
            (
                "pickup-letter",
                ("pickup", deflated_file!("37-deflated.dat")),
            ),
            // TODO(nenikitov): This model doesn't look good with any palette
            ("pickup-key-1", ("pickup", deflated_file!("38-deflated.dat"))),
            (
                "pickup-flakjacket-25",
                ("pickup", deflated_file!("39-deflated.dat")),
            ),
            (
                "pickup-flakjacket-50",
                ("pickup", deflated_file!("3A-deflated.dat")),
            ),
            (
                "pickup-flakjacket-100",
                ("pickup", deflated_file!("3B-deflated.dat")),
            ),
        ])
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let palettes: HashMap<&str, [Color; 256]> = HashMap::from_iter(
            COLOR_MAPS.iter().map(|(name, data)| {
                let (_, color_map) = ColorMap::parser(())(data).expect("Color map is valid");
                (*name, color_map.shades[15])
            })
        );

        for (name, (palette, data)) in MODELS.iter() {
            let palette = palettes.get(palette).expect("Color map is present");
            let (_, model) = Model::parser(())(data)?;

            output_file(
                PARSED_PATH.join(format!("model/{name}.py")),
                model.to_py(palette),
            )?;
        }

        Ok(())
    }

    // TODO(nenikitov): Move this to `utils::format` module
    pub trait ModelPythonFile {
        fn to_py(&self, palette: &[Color]) -> String;
    }

    impl ModelPythonFile for Model {
        fn to_py(&self, palette: &[Color]) -> String {
            // TODO(nenikitov): Move a common part of the script into a separate `.py` file
            format!(
                r#"import bpy

# Data
texture_width = {}
texture_height = {}
texture = [
{}
]
frames = [
{}
]
triangles = [
{}
]
sequences = [
{}
]

# Clean up
for obj in bpy.data.objects:
    bpy.data.objects.remove(obj, do_unlink=True)

# Mesh
mesh = bpy.data.meshes.new("Mesh")
object = bpy.data.objects.new("Model", mesh)
mesh.from_pydata(
    [
        (v["x"], v["y"], v["z"])
        for v in frames[0]
    ],
    [],
    [
        (t["points"][0]["vertex_index"], t["points"][1]["vertex_index"], t["points"][2]["vertex_index"])
        for t in triangles
    ]
)
# UV
uv = mesh.uv_layers.new(name="UV")
for loop in mesh.loops:
    i = loop.index
    triangle_point = triangles[i // 3]["points"][i % 3]
    uv.data[i].uv = (triangle_point["u"], triangle_point["v"])
# Texture
image = bpy.data.images.new("Texture", texture_width, texture_height)
image.pixels = texture
image.update()
# Material
material = bpy.data.materials.new(name="Material")
material.use_nodes = True
material_bsdf = material.node_tree.nodes["Principled BSDF"]
material_bsdf.inputs["Roughness"].default_value = 1.0
material_texture = material.node_tree.nodes.new("ShaderNodeTexImage")
material_texture.image = image
material_texture.interpolation = "Closest"
material.node_tree.links.new(material_texture.outputs["Color"], material_bsdf.inputs["Base Color"])
mesh.materials.append(material)
# Shape keys
shape_keys = []
for i, f in enumerate(frames):
    shape_keys.append(object.shape_key_add(name=f"Key {{i}}"))
    for v_i, v in enumerate(f):
        shape_keys[i].data[v_i].co = (v["x"], v["y"], v["z"])
# Actions
actions = []
mesh.shape_keys.animation_data_create()
for i, s in enumerate(sequences):
    actions.append(bpy.data.actions.new(f"Action {{i}}"))
    actions[i].use_fake_user = True
    actions[i].frame_end = len(s["frames"])
    actions[i].use_frame_range = True
    mesh.shape_keys.animation_data.action = actions[i]
    for f_i, frame in enumerate(s["frames"]):
        for s_i, shape_key in enumerate(shape_keys):
            shape_key.value = 1.0 if s_i == frame else 0.0
            shape_key.keyframe_insert(data_path="value", frame=f_i + 1)

# Finalize
bpy.context.collection.objects.link(object)
bpy.context.view_layer.objects.active = object
object.select_set(True)
            "#,
                self.texture.width(),
                self.texture.height(),
                self.texture
                    .with_palette(&palette)
                    .into_iter()
                    // In blender, y axis of textures is reversed
                    .rev()
                    .map(|r| format!("    {}", r.into_iter().map(|c| c.to_string()).join(", ")))
                    .join(",\n"),
                self.frames
                    .iter()
                    .map(|f| format!(
                        "    [{}]",
                        f.vertices.iter().map(|v| v.to_string()).join(", ")
                    ))
                    .join(",\n"),
                self.triangles
                    .iter()
                    .map(|v| format!("    {v}"))
                    .join(",\n"),
                self.sequences
                    .iter()
                    .map(|s| format!("    {s}"))
                    .join(",\n")
            )
        }
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{}, {}, {}, 1.0"#,
                self.r as f32 / 255.0,
                self.g as f32 / 255.0,
                self.b as f32 / 255.0
            )
        }
    }

    impl Display for ModelVertex {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{{ "x": {}, "y": {}, "z": {}, "lightmap": {} }}"#,
                self.x, self.y, self.z, self.normal_index
            )
        }
    }

    impl Display for ModelTriangle {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{{ "points": [{}, {}, {}] }}"#,
                self.points[0], self.points[1], self.points[2],
            )
        }
    }

    impl Display for ModelPoint {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{{ "vertex_index": {}, "u": {}, "v": {} }}"#,
                self.vertex_index, self.u, self.v
            )
        }
    }

    impl Display for ModelSequence {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{{ "frames": [{}] }}"#,
                self.frames.iter().map(u32::to_string).join(", ")
            )
        }
    }
}
