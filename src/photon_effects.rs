extern crate image;
extern crate photon_rs as photon;
extern crate time;

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
