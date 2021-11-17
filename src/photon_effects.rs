extern crate image;
extern crate photon_rs as photon;
extern crate time;

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::solarize(
///   input,
///   output_dir,
///   "logo_solarize.png",
/// ).expect("Couldn't create solarize image");
/// ```
pub fn solarize(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::solarize(&mut img);
    photon::native::save_image(
        img,
        &format!("{}/{}_solarize.jpg", image_output_directory, parse_filename)[..],
    );
    println!(
        "Generate {}/{}_solarize.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::colorize(
///   input,
///   output_dir,
///   "logo_colorize.png",
/// ).expect("Couldn't create colorize image");
/// ```
pub fn colorize(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::colorize(&mut img);
    photon::native::save_image(
        img,
        &format!("{}/{}_colorize.jpg", image_output_directory, parse_filename)[..],
    );
    println!(
        "Generate {}/{}_colorize.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::halftone(
///   input,
///   output_dir,
///   "logo_halftone.png",
/// ).expect("Couldn't create halftone image");
/// ```
pub fn halftone(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::halftone(&mut img);
    photon::native::save_image(
        img,
        &format!("{}/{}_halftone.jpg", image_output_directory, parse_filename)[..],
    );
    println!(
        "Generate {}/{}_halftone.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::horizontal_strips(
///   input,
///   output_dir,
///   "logo_horizontal_strips.png",
///   5
/// ).expect("Couldn't create horizontal_strips image");
/// ```
pub fn horizontal_strips(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    num_strips: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::horizontal_strips(&mut img, num_strips);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_horizontal_strips.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_horizontal_strips.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::vertical_strips(
///   input,
///   output_dir,
///   "logo_vertical_strips.png",
///   3
/// ).expect("Couldn't create vertical_strips image");
/// ```
pub fn vertical_strips(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    num_strips: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::vertical_strips(&mut img, num_strips);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_horizontal_strips.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_horizontal_strips.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::inc_brightness(
///   input,
///   output_dir,
///   "logo_inc_brightness.png",
///   10
/// ).expect("Couldn't create inc_brightness image");
/// ```
pub fn inc_brightness(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    brightness: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::inc_brightness(&mut img, brightness);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_inc_brightness.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_inc_brightness.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::multiple_offsets(
///   input,
///   output_dir,
///   "logo_multiple_offsets.png",
///   30,
///   0,
///   2
/// ).expect("Couldn't create multiple_offsets image");
/// ```
pub fn multiple_offsets(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    offset: u32,
    channel_index: usize,
    channel_index2: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::multiple_offsets(&mut img, offset, channel_index, channel_index2);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_multiple_offsets.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_multiple_offsets.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::offset(
///   input,
///   output_dir,
///   "logo_offset.png",
///   0,
///   30
/// ).expect("Couldn't create offset image");
/// ```
pub fn offset(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    channel_index: usize,
    offset: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::offset(&mut img, channel_index, offset);
    photon::native::save_image(
        img,
        &format!("{}/{}_offset.jpg", image_output_directory, parse_filename)[..],
    );
    println!(
        "Generate {}/{}_offset.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::offset_blue(
///   input,
///   output_dir,
///   "logo_offset_blue.png",
///   20
/// ).expect("Couldn't create offset_blue image");
/// ```
pub fn offset_blue(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    offset_amt: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::offset_blue(&mut img, offset_amt);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_offset_blue.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_offset_blue.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::offset_green(
///   input,
///   output_dir,
///   "logo_offset_green.png",
///   35
/// ).expect("Couldn't create offset_green image");
/// ```
pub fn offset_green(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    offset_amt: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::offset_green(&mut img, offset_amt);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_offset_green.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_offset_green.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::offset_red(
///   input,
///   output_dir,
///   "logo_offset_red.png",
///   25
/// ).expect("Couldn't create offset_red image");
/// ```
pub fn offset_red(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    offset_amt: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::offset_red(&mut img, offset_amt);
    photon::native::save_image(
        img,
        &format!(
            "{}/{}_offset_red.jpg",
            image_output_directory, parse_filename
        )[..],
    );
    println!(
        "Generate {}/{}_offset_red.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::primary(
///   input,
///   output_dir,
///   "logo_primary.png"
/// ).expect("Couldn't create primary image");
/// ```
pub fn primary(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::primary(&mut img);
    photon::native::save_image(
        img,
        &format!("{}/{}_primary.jpg", image_output_directory, parse_filename)[..],
    );
    println!(
        "Generate {}/{}_primary.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}

/// Example
///
/// ```no_run
/// extern crate photon_effects;
///
/// let input = "/tmp/logo.png";
/// let output_dir = "/tmp/";
///
/// photon_effects::tint(
///   input,
///   output_dir,
///   "logo_tint.png",
///   18,
///   22,
///   26,
/// ).expect("Couldn't create tint image");
/// ```
pub fn tint(
    image_path: &str,
    image_output_directory: &str,
    parse_filename: &str,
    r_offset: u32,
    g_offset: u32,
    b_offset: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = photon::native::open_image(image_path)?;
    photon::effects::tint(&mut img, r_offset, g_offset, b_offset);
    photon::native::save_image(
        img,
        &format!("{}/{}_tint.jpg", image_output_directory, parse_filename)[..],
    );
    println!(
        "Generate {}/{}_tint.jpg image.",
        image_output_directory, parse_filename
    );
    Ok(())
}
