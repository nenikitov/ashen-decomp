#[cfg(any(test, feature = "conv"))]
use std::ops::Deref;

#[cfg(any(test, feature = "conv"))]
use image::{
    Frame, ImageEncoder, RgbaImage,
    codecs::{
        gif::{GifEncoder, Repeat},
        png::PngEncoder,
    },
};
use itertools::Itertools;

use crate::asset::{
    color_map::Color,
    model::Model,
    sound::sample::{AudioBuffer, AudioSamplePoint},
};

#[cfg(any(test, feature = "conv"))]
pub trait PngFile {
    fn to_png(&self) -> Vec<u8>;
}

// impl for any 2D array like data structure.
#[cfg(any(test, feature = "conv"))]
impl<Outer: ?Sized, Inner> PngFile for Outer
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[Color]>,
{
    fn to_png(&self) -> Vec<u8> {
        let width = self[0].as_ref().len() as u32;
        let height = self.len() as u32;

        let mut data = vec![];
        let mut encoder = PngEncoder::new(&mut data);

        encoder
            .write_image(
                &self
                    .iter()
                    .flat_map(|slice| {
                        slice
                            .as_ref()
                            .iter()
                            .flat_map(|color| [color.r, color.g, color.b])
                    })
                    .collect::<Vec<_>>(),
                width,
                height,
                image::ColorType::Rgb8,
            )
            .expect("Generated image data must be valid");

        data
    }
}

#[cfg(any(test, feature = "conv"))]
pub trait GifFile {
    fn to_gif(&self) -> Vec<u8>;
}

#[cfg(any(test, feature = "conv"))]
impl<Outer: ?Sized, Inner1, Inner2> GifFile for Outer
where
    Outer: Deref<Target = [Inner1]>,
    Inner1: Deref<Target = [Inner2]>,
    Inner2: AsRef<[Color]>,
{
    fn to_gif(&self) -> Vec<u8> {
        let width = self[0][0].as_ref().len() as u32;
        let height = self[0].len() as u32;

        let mut data = vec![];
        let mut encoder = GifEncoder::new_with_speed(&mut data, 10);

        encoder
            .encode_frames(self.iter().map(|f| {
                Frame::new(
                    RgbaImage::from_vec(
                        width,
                        height,
                        f.iter()
                            .flat_map(|slice| {
                                slice
                                    .as_ref()
                                    .iter()
                                    .flat_map(|color| [color.r, color.g, color.b, 255])
                            })
                            .collect(),
                    )
                    .expect("Generated image data must be valid"),
                )
            }))
            .expect("Generated image frames must be valid");

        encoder
            .set_repeat(Repeat::Infinite)
            .expect("Generated image frames must loop");

        drop(encoder);

        data
    }
}

pub trait PaletteTexture {
    // TODO(Unavailable): `&[Color; 256]`
    fn with_palette(&self, palette: &[Color]) -> Vec<Vec<Color>>;
}

// impl for any 2D array like data structure.
impl<Outer: ?Sized, Inner> PaletteTexture for Outer
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[u8]>,
{
    fn with_palette(&self, palette: &[Color]) -> Vec<Vec<Color>> {
        self.iter()
            .map(|c| c.as_ref().iter().map(|c| palette[*c as usize]).collect())
            .collect()
    }
}

pub trait WaveFile<S: AudioSamplePoint> {
    fn to_wave(&self) -> Vec<u8>
    where
        [(); S::SIZE_BYTES]:;
}

impl<S: AudioSamplePoint> WaveFile<S> for AudioBuffer<S> {
    fn to_wave(&self) -> Vec<u8>
    where
        [(); S::SIZE_BYTES]:,
    {
        const CHANNELS: usize = 1;

        let bytes_per_sample = S::SIZE_BYTES;
        let bits_per_sample = bytes_per_sample * 8;

        let size = self.len_samples() * CHANNELS * bytes_per_sample;

        "RIFF"
            .bytes()
            .chain(u32::to_le_bytes((36 + size) as u32))
            .chain("WAVE".bytes())
            .chain("fmt ".bytes())
            .chain(u32::to_le_bytes(16))
            .chain(u16::to_le_bytes(S::wave_format().signature()))
            .chain(u16::to_le_bytes(CHANNELS as u16))
            .chain(u32::to_le_bytes(self.sample_rate as u32))
            .chain(u32::to_le_bytes(
                (self.sample_rate * CHANNELS * bytes_per_sample) as u32,
            ))
            .chain(u16::to_le_bytes((CHANNELS * bytes_per_sample) as u16))
            .chain(u16::to_le_bytes(bits_per_sample as u16))
            .chain("data".bytes())
            .chain(u32::to_le_bytes(size as u32))
            .chain(self.data.iter().flat_map(|s| s.wave_le_bytes()))
            .collect()
    }
}

// TODO(Unavailable): We don't need this trait at all once the `parse_rom_asset`
// are removed. The `to_py` method could be inlined into `Model::to_blender_script`.
pub(crate) trait ModelPythonFile {
    fn to_py(&self, palette: &[Color]) -> String;
}

impl ModelPythonFile for Model {
    fn to_py(&self, palette: &[Color]) -> String {
        macro_rules! display_color {
            ($c:expr) => {
                format!(
                    r#"{}, {}, {}, 1.0"#,
                    $c.r as f32 / 255.0,
                    $c.g as f32 / 255.0,
                    $c.b as f32 / 255.0
                )
            };
        }

        macro_rules! display_vertex {
            ($v:expr) => {
                format!(
                    r#"{{ "x": {}, "y": {}, "z": {}, "lightmap": {} }}"#,
                    $v.x, $v.y, $v.z, $v.normal_index
                )
            };
        };

        macro_rules! display_point {
            ($p:expr) => {
                format!(
                    r#"{{ "vertex_index": {}, "u": {}, "v": {} }}"#,
                    $p.vertex_index, $p.u, $p.v
                )
            };
        };

        macro_rules! display_triangle {
            ($t:expr) => {
                format!(
                    r#"{{ "points": [{}, {}, {}] }}"#,
                    display_point!($t.points[0]),
                    display_point!($t.points[1]),
                    display_point!($t.points[2]),
                )
            };
        };

        macro_rules! display_sequence {
            ($s:expr) => {
                format!(
                    r#"{{ "frames": [{}] }}"#,
                    $s.frames.iter().map(u32::to_string).join(", ")
                )
            };
        }

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
                .map(|r| format!(
                    "    {}",
                    r.into_iter().map(|c| display_color!(c)).join(", ")
                ))
                .join(",\n"),
            self.frames
                .iter()
                .map(|f| format!(
                    "    [{}]",
                    f.vertices.iter().map(|v| display_vertex!(v)).join(", ")
                ))
                .join(",\n"),
            self.triangles
                .iter()
                .map(|t| format!("    {}", display_triangle!(t)))
                .join(",\n"),
            self.sequences
                .iter()
                .map(|s| format!("    {}", display_sequence!(s)))
                .join(",\n")
        )
    }
}
