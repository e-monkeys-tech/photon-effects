extern crate photon_effects;

#[cfg(test)]
mod tests {
    #[test]
    fn test_solarize() {
        photon_effects::solarize(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_solarize.png",
        )
        .expect("Couldn't create solarize image");
    }

    #[test]
    fn test_colorize() {
        photon_effects::colorize(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_colorize.png",
        )
        .expect("Couldn't create colorize image");
    }

    #[test]
    fn test_halftone() {
        photon_effects::halftone(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
        )
        .expect("Couldn't create halftone image");
    }

    #[test]
    fn test_inc_brightness() {
        photon_effects::inc_brightness(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            10,
        )
        .expect("Couldn't create inc_brightness image");
    }

    #[test]
    fn test_vertical_strips() {
        photon_effects::vertical_strips(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            5,
        )
        .expect("Couldn't create vertical_strips image");
    }

    #[test]
    fn test_horizontal_strips() {
        photon_effects::horizontal_strips(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            7,
        )
        .expect("Couldn't create horizontal_strips image");
    }

    #[test]
    fn test_tint() {
        photon_effects::tint(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            10,
            20,
            15,
        )
        .expect("Couldn't create tint image");
    }

    #[test]
    fn test_offset() {
        photon_effects::offset(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            0,
            30,
        )
        .expect("Couldn't create offset image");
    }

    #[test]
    fn test_offset_blue() {
        photon_effects::offset_blue(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            30,
        )
        .expect("Couldn't create offset_blue image");
    }

    #[test]
    fn test_offset_red() {
        photon_effects::offset_red(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            30,
        )
        .expect("Couldn't create offset_red image");
    }

    #[test]
    fn test_offset_green() {
        photon_effects::offset_green(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            30,
        )
        .expect("Couldn't create offset_green image");
    }

    #[test]
    fn test_multiple_offsets() {
        photon_effects::multiple_offsets(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
            30,
            0,
            2,
        )
        .expect("Couldn't create multiple_offsets image");
    }

    #[test]
    fn test_primary() {
        photon_effects::primary(
            "src/assets/logo.png",
            "src/assets/out/",
            "logo_halftone.png",
        )
        .expect("Couldn't create multiple_offsets image");
    }
}
