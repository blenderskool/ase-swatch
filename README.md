# ase-swatch

Rust and WebAssembly library to encode and create Adobe Swatch Exchange (ASE) files for colors and palettes.

This project is based on the [swatch](https://github.com/nsfmc/swatch) library by Marcos A Ojeda written in Python.

## Usage

### In Rust

```rust
use ase_swatch::create_ase;
use ase_swatch::types::*;

let swatches = vec![
    ObjectSwatch {
        name: "Palette 1".to_string(),
        swatches: vec![
            ObjectColor {
                name: "Red".to_string(),
                object_type: ObjectColorType::Global,
                data: Color {
                    mode: ColorMode::Rgb,
                    values: vec![1.0, 0.0, 0.0],
                },
            },
            ObjectColor {
                name: "Green".to_string(),
                object_type: ObjectColorType::Global,
                data: Color {
                    mode: ColorMode::Rgb,
                    values: vec![0.0, 1.0, 0.0],
                },
            },
            ObjectColor {
                name: "Blue".to_string(),
                object_type: ObjectColorType::Global,
                data: Color {
                    mode: ColorMode::Rgb,
                    values: vec![0.0, 0.0, 1.0],
                },
            },
        ],
    }
];
let colors = vec![
    ObjectColor {
        name: "Blue".to_string(),
        object_type: ObjectColorType::Global,
        data: Color {
            mode: ColorMode::Rgb,
            values: vec![0.0, 0.0, 1.0],
        },
    },
];
let result: Vec<u8> = create_ase(&swatches, &colors);
// resulting vector of bytes can be written as a binary file
```

### In JavaScript

**Prerequisite steps:**

- Clone the repo
- Build a WebAssembly binary using [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) with appropriate build target.

The library exposes a `create_ase_js()` function that can be called from JavaScript with a similar signature to `create_ase` function.

```javascript
const swatches = [
  {
    name: "Palette 1",
    swatches: [
      {
        name: "Red",
        object_type: "Global",
        data: {
          mode: "Rgb",
          values: [1.0, 0.0, 0.0],
        },
      },
      {
        name: "Green",
        object_type: "Global",
        data: {
          mode: "Rgb",
          values: [0.0, 1.0, 0.0],
        },
      },
      {
        name: "Blue",
        object_type: "Global",
        data: {
          mode: "Rgb",
          values: [0.0, 0.0, 1.0],
        },
      },
    ],
  },
];
let colors = [
  {
    name: "Blue",
    object_type: "Global",
    data: {
      mode: "Rgb",
      values: [0.0, 0.0, 1.0],
    },
  },
];
const result = create_ase_js(swatches, colors);
// resulting array is a `Uint8Array` of the created ASE file
```

## License
ase-swatch is [MIT Licensed](./LICENSE)