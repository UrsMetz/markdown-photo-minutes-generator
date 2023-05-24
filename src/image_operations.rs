use image::imageops::FilterType;
use std::fs;
use std::io::Write;
use std::path::Path;

fn save_as_resized_image<P: AsRef<Path>>(
    source_image_path: P,
    dest_image_path: P,
    ratio: f32,
) -> anyhow::Result<()> {
    let source_image = image::io::Reader::open(&source_image_path)?.decode()?;
    let source_width = source_image.width();
    let source_height = source_image.height();
    let new_width = calculate_new_dimension(ratio, source_width);
    let new_height = calculate_new_dimension(ratio, source_height);
    let dest_image = source_image.resize(new_width, new_height, FilterType::Gaussian);
    let mut dest_file = fs::File::create(dest_image_path)?;
    dest_image.write_to(&mut dest_file, image::ImageOutputFormat::Jpeg(100))?;
    dest_file.flush()?;
    Ok(())
}

fn calculate_new_dimension(ratio: f32, source_width: u32) -> u32 {
    (ratio.sqrt() * source_width as f32) as u32
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use speculoos::prelude::*;

    use crate::image_operations::save_as_resized_image;

    #[test]
    fn save_as_resized_image_works() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        let dest_path = dir.path().join("dest");
        fs::create_dir(section_path)?;
        let source_image_path = Path::new("./src/empty-100x200.jpg");

        fs::create_dir(&dest_path)?;
        let dest_image_path = dest_path.join("abc.dest.jpg");

        save_as_resized_image(source_image_path, dest_image_path.as_path(), 0.5)?;

        assert_that!(dest_image_path).exists();
        let dynamic_image = image::io::Reader::open(dest_image_path)?.decode()?;
        assert_that!(dynamic_image.width()).is_less_than_or_equal_to(72);
        assert_that!(dynamic_image.width()).is_greater_than_or_equal_to(68);
        assert_that!(dynamic_image.height()).is_less_than_or_equal_to(144);
        assert_that!(dynamic_image.height()).is_greater_than_or_equal_to(140);

        Ok(())
    }
}
