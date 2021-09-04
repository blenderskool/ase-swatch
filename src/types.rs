use serde::{Deserialize};
use wasm_bindgen::prelude::*;

/// Format of the color
/// - Rgb - 3 floats each with range [0-1]
/// - Lab - 3 floats with L in range [0-1] and A, B in range [-128.0, 127.0]
/// - Cmyk - 4 floats each with range [0-1]
/// - Gray - 1 float with range [0-1]
#[wasm_bindgen]
#[derive(Deserialize)]
pub enum ColorMode {
    Rgb,
    Lab,
    Cmyk,
    Gray,
}

/// Type of the Color Object
/// 
/// _From <https://pypi.org/project/swatch/>_
/// > Process colors are standard colors, this is the default if you define a new color in illustrator. As the name implies, they’re mixed from either RGB or CMYK depending on the document color mode.
/// 
/// > Global colors are the same thing as process colors, but they have one neat property which is that when you update them, they are updated all throughout your artwork. This makes them something like “color references” and quite useful if you’re doing something like reskinning some extant document.
/// 
/// > Spot colors are implicitly global but have the nifty property that you can create new swatches from them based on “tints” or, effectively some screened value of that color. The only hitch is that tints, even though they can be part of your file, can’t be stored/exchanged as swatches. I’m on the fence as to how problematic this is, but that’s just how it goes. Even illustrator won’t save them out, it’s just not supported in the app (almost certainly due to the nature of the file forma
#[wasm_bindgen]
#[derive(Deserialize, Clone, Copy)]
pub enum ObjectColorType {
    Global = 0,
    Spot = 1,
    Process = 2,
}

/// Color data
/// 
/// Stores individual color information.
#[derive(Deserialize)]
pub struct Color {
    pub mode: ColorMode,
    pub values: Vec<f32>,
}

/// A Color object
/// 
/// Stores data associated with a color object.
#[derive(Deserialize)]
pub struct ObjectColor {
    pub name: String,
    pub object_type: ObjectColorType,
    pub data: Color,
}

/// Swatch object
/// 
/// Stores a group of `ObjectColor` under a name.
#[derive(Deserialize)]
pub struct ObjectSwatch {
    pub name: String,
    pub swatches: Vec<ObjectColor>,
}
