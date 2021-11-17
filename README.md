# photon-effects

photon-effects is an extension of photon-rs to easily apply filter effects on image or image directory.

## Some examples 

```rs
extern crate photon_effects;
 
let input = "/tmp/logo.png";
let output_dir = "/tmp/";

photon_effects::multiple_offsets(
  input, 
  output_dir, 
  "logo_multiple_offsets.png",
  30,
  0,
  2
).expect("Couldn't create multiple_offsets image");
```

Result:

![Logo Output Multiple Offsets](src/assets/out/logo_halftone.png_multiple_offsets.jpg)

## crates.io

## docs.rs