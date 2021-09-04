#[macro_use]
extern crate structure;

pub mod types;

use types::*;
use wasm_bindgen::prelude::*;

fn chunk_for_color(obj: &ObjectColor) -> Vec<u8> {
    let mut title = obj.name.clone();
    title.push('\0');
    // Chunk body starts with length of title
    let mut chunk = structure!(">H").pack(title.len() as u16).unwrap();

    // Each character in title is pushed in 16 bits(two 8 bits)
    for character in title.encode_utf16() {
        let be_bytes = character.to_be_bytes();
        chunk.push(be_bytes[0]);
        chunk.push(be_bytes[1]);
    }

    let color_mode = match obj.data.mode {
        ColorMode::Rgb => "RGB",
        ColorMode::Lab => "LAB",
        ColorMode::Cmyk => "CMYK",
        ColorMode::Gray => "Gray",
    };

    // Color mode is adjusted to 4 bytes and pushed
    let adjusted_color_mode = format!("{:<4}", color_mode);
    chunk.append(&mut structure!("!4s").pack(adjusted_color_mode.as_bytes()).unwrap());

    // Append the float values of the color
    for value in &obj.data.values {
        chunk.append(&mut structure!("!f").pack(*value).unwrap());
    }

    // Append object color type("Global", "Spot", "process") value
    chunk.append(&mut structure!(">h").pack(obj.object_type as i16).unwrap());

    // Color chunk starts with [0, 1]
    let mut head_chunk = vec![0, 1];
    // Append size of the chunk body
    head_chunk.append(&mut structure!(">I").pack(chunk.len() as u32).unwrap());
    // Append the chunk body
    head_chunk.append(&mut chunk);

    return head_chunk;
}

fn chunk_for_swatch(obj: &ObjectSwatch) -> Vec<u8> {
    let mut title = obj.name.clone();
    title.push('\0');
    // Chunk body starts with length of title
    let mut chunk_body = structure!(">H").pack(title.len() as u16).unwrap();

    // Each character in title is pushed in 16 bits(two 8 bits)
    for character in title.encode_utf16() {
        let be_bytes = character.to_be_bytes();
        chunk_body.push(be_bytes[0]);
        chunk_body.push(be_bytes[1]);
    }

    // Chunk head starts with [192, 1], possibly an identifier for swatch data
    let mut chunk_head = vec![192, 1];
    // Append length of chunk body (this excludes size of individual color objects in the body)
    chunk_head.append(&mut structure!(">I").pack(chunk_body.len() as u32).unwrap());

    let mut chunk = Vec::new();
    chunk.append(&mut chunk_head);
    chunk.append(&mut chunk_body);

    // Append each color object chunk
    for color in &obj.swatches {
        chunk.append(&mut chunk_for_color(&color));
    }

    // End the swatch with [192, 2, 0, 0, 0, 0]
    chunk.append(&mut vec![192, 2]);
    chunk.append(&mut vec![0, 0, 0, 0]);

    return chunk;
}

/// Creates an ASE binary data from the Swatch and Color objects.
/// 
/// ## Usage:
/// ```
/// use ase_swatch::create_ase;
/// use ase_swatch::types::*;
/// let swatches = vec![
///     ObjectSwatch {
///         name: "Palette 1".to_string(),
///         swatches: vec![
///             ObjectColor {
///                 name: "Red".to_string(),
///                 object_type: ObjectColorType::Global,
///                 data: Color {
///                     mode: ColorMode::Rgb,
///                     values: vec![1.0, 0.0, 0.0],
///                 },
///             },
///             ObjectColor {
///                 name: "Green".to_string(),
///                 object_type: ObjectColorType::Global,
///                 data: Color {
///                     mode: ColorMode::Rgb,
///                     values: vec![0.0, 1.0, 0.0],
///                 },
///             },
///             ObjectColor {
///                 name: "Blue".to_string(),
///                 object_type: ObjectColorType::Global,
///                 data: Color {
///                     mode: ColorMode::Rgb,
///                     values: vec![0.0, 0.0, 1.0],
///                 },
///             },
///         ],
///     }
/// ];
/// let colors = vec![
///     ObjectColor {
///         name: "Blue".to_string(),
///         object_type: ObjectColorType::Global,
///         data: Color {
///             mode: ColorMode::Rgb,
///             values: vec![0.0, 0.0, 1.0],
///         },
///     },
/// ];
/// let result: Vec<u8> = create_ase(&swatches, &colors);
/// ```
pub fn create_ase(swatch_objects: &Vec<ObjectSwatch>, color_objects: &Vec<ObjectColor>) -> Vec<u8> {
    let s = structure!("!4sHHI");
    let header = "ASEF".as_bytes();
    let (v_major, v_minor) = (1, 0);
    
    let mut chunk_count: u32 = 0;
    let mut chunks = Vec::new();
    for object in swatch_objects {
        chunk_count += 2 + object.swatches.len() as u32;
        chunks.append(&mut chunk_for_swatch(&object));
    }

    for object in color_objects {
        chunk_count += 1;
        chunks.append(&mut chunk_for_color(&object));
    }

    let mut head_chunk = s.pack(header, v_major, v_minor, chunk_count).unwrap();
    head_chunk.append(&mut chunks);

    return head_chunk;
}

/// `create_ase` that can be called from JavaScript.
#[wasm_bindgen]
pub fn create_ase_js(swatches: &JsValue, colors: &JsValue) -> Vec<u8> {
    let swatch_objects: Vec<ObjectSwatch> = swatches.into_serde().unwrap();
    let color_objects: Vec<ObjectColor> = colors.into_serde().unwrap();

    return create_ase(&swatch_objects, &color_objects);
}