extern crate photon_effects;

#[cfg(test)]
mod tests {
    #[test]
    fn test_solarize() {
      photon_effects::solarize("src/assets/logo.png", "src/assets/out/", "logo_solarize.png")
      .expect("Couldn't create solarize image");
    }

    #[test]
    fn test_colorize() {
      photon_effects::colorize("src/assets/logo.png", "src/assets/out/", "logo_colorize.png")
      .expect("Couldn't create colorize image");
    }

    #[test]
    fn test_halftone() {
      photon_effects::halftone("src/assets/logo.png", "src/assets/out/", "logo_halftone.png")
      .expect("Couldn't create halftone image");
    }

    /* #[test]
    inc_brightness(image_path, image_output_directory, &parse_filename, 10)
    .expect("Couldn't create inc_brightness image");
    
    #[test]
    vertical_strips(image_path, image_output_directory, &parse_filename, 5)
    .expect("Couldn't create vertical_strips image");

    #[test]
    horizontal_strips(image_path, image_output_directory, &parse_filename, 7)
    .expect("Couldn't create horizontal_strips image");

    #[test]
    tint(image_path, image_output_directory, &parse_filename, 10, 20, 15)
    .expect("Couldn't create tint image");

    #[test]
    offset(image_path, image_output_directory, &parse_filename, 0, 30)
    .expect("Couldn't create offset image");

    #[test]
    offset_blue(image_path, image_output_directory, &parse_filename, 30)
    .expect("Couldn't create offset_blue image");

    #[test]
    offset_red(image_path, image_output_directory, &parse_filename, 30)    
    .expect("Couldn't create offset_red image");

    #[test]
    offset_green(image_path, image_output_directory, &parse_filename, 30)
    .expect("Couldn't create offset_green image");

    #[test]
    multiple_offsets(image_path, image_output_directory, &parse_filename, 30, 0, 2)
    .expect("Couldn't create multiple_offsets image");

    #[test]
    primary(image_path, image_output_directory, &parse_filename)
    .expect("Couldn't create primary image"); */

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}