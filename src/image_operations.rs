use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use exif::{In, Tag};
use image::imageops::FilterType;
use image::{imageops, DynamicImage};

pub fn save_as_resized_image<S: AsRef<Path>, D: AsRef<Path>>(
    source_image_path: S,
    dest_image_path: D,
    ratio: f32,
) -> anyhow::Result<()> {
    if ratio == 1.0 {
        fs_err::copy(source_image_path, dest_image_path)?;
        return Ok(());
    }

    let source_image = image::ImageReader::open(&source_image_path)
        .with_context(|| "source file does not exist")?
        .decode()?;

    let source_width = source_image.width();
    let source_height = source_image.height();
    let new_width = calculate_new_dimension(ratio, source_width);
    let new_height = calculate_new_dimension(ratio, source_height);
    let dest_image = source_image.resize(new_width, new_height, FilterType::Triangle);
    let dest_image = rotate(
        dest_image,
        get_jpeg_orientation(PathBuf::from(source_image_path.as_ref()))?,
    );
    let dest_image_path = dest_image_path.as_ref();
    dest_image_path.parent().map(fs_err::create_dir_all);
    let mut dest_file = fs_err::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dest_image_path)
        .context("destination file exists")?;
    dest_image.write_to(&mut dest_file, image::ImageFormat::Jpeg)?;
    dest_file.flush()?;
    Ok(())
}

fn calculate_new_dimension(ratio: f32, source_width: u32) -> u32 {
    (ratio.sqrt() * source_width as f32) as u32
}

fn rotate(mut img: DynamicImage, orientation: u8) -> DynamicImage {
    let rgba = img.color().has_alpha();
    img = match orientation {
        2 => DynamicImage::ImageRgba8(imageops::flip_horizontal(&img)),
        3 => DynamicImage::ImageRgba8(imageops::rotate180(&img)),
        4 => DynamicImage::ImageRgba8(imageops::flip_vertical(&img)),
        5 => DynamicImage::ImageRgba8(imageops::flip_horizontal(&imageops::rotate90(&img))),
        6 => DynamicImage::ImageRgba8(imageops::rotate90(&img)),
        7 => DynamicImage::ImageRgba8(imageops::flip_horizontal(&imageops::rotate270(&img))),
        8 => DynamicImage::ImageRgba8(imageops::rotate270(&img)),
        _ => img,
    };
    if !rgba {
        img = DynamicImage::ImageRgb8(img.into_rgb8());
    }
    img
}
fn get_jpeg_orientation(file_path: PathBuf) -> anyhow::Result<u8> {
    let file = std::fs::File::open(file_path).context("problem opening the file")?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader);

    match exif {
        Ok(exif) => {
            let orientation: u8 = match exif.get_field(Tag::Orientation, In::PRIMARY) {
                Some(orientation) => match orientation.value.get_uint(0) {
                    Some(v @ 1..=8) => v as u8,
                    _ => 1,
                },
                None => 1,
            };

            Ok(orientation)
        }
        Err(_) => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fs_err;
    use speculoos::prelude::*;

    use crate::image_operations::save_as_resized_image;

    #[test]
    fn save_as_resized_image_works() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        let dest_path = dir.path().join("dest");
        fs_err::create_dir(section_path)?;
        let source_image_path = Path::new("./src/empty-100x200.jpg");

        fs_err::create_dir(&dest_path)?;
        let dest_image_path = dest_path.join("abc.dest.jpg");

        save_as_resized_image(source_image_path, dest_image_path.as_path(), 0.5)?;

        assert_that!(dest_image_path).exists();
        let dynamic_image = image::ImageReader::open(dest_image_path)?.decode()?;
        assert_that!(dynamic_image.width()).is_less_than_or_equal_to(72);
        assert_that!(dynamic_image.width()).is_greater_than_or_equal_to(68);
        assert_that!(dynamic_image.height()).is_less_than_or_equal_to(144);
        assert_that!(dynamic_image.height()).is_greater_than_or_equal_to(140);

        Ok(())
    }

    #[test]
    fn copies_input_image_when_ratio_is_1() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        let dest_path = dir.path().join("dest");
        fs_err::create_dir(section_path)?;
        let source_image_path = Path::new("./src/empty-100x200.jpg");

        fs_err::create_dir(&dest_path)?;
        let dest_image_path = dest_path.join("abc.dest.jpg");

        save_as_resized_image(source_image_path, dest_image_path.as_path(), 1.0)?;

        assert_that!(dest_image_path).exists();
        let input_image = image::ImageReader::open(source_image_path)?.decode()?;
        let output_image = image::ImageReader::open(dest_image_path)?.decode()?;

        assert_that!(output_image).is_equal_to(input_image);

        Ok(())
    }

    #[test]
    fn creates_subdirectories_when_they_do_not_exist() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        let dest_root_path = dir.path().join("dest");
        fs_err::create_dir(section_path)?;
        let source_image_path = Path::new("./src/empty-100x200.jpg");

        fs_err::create_dir(&dest_root_path)?;
        let dest_image_path = dest_root_path
            .join("sub-dir")
            .join("sub-sub-dir")
            .join("abc.dest.jpg");

        save_as_resized_image(source_image_path, dest_image_path.as_path(), 0.5)?;

        assert_that!(dest_image_path).exists();
        let dynamic_image = image::ImageReader::open(dest_image_path)?.decode()?;
        assert_that!(dynamic_image.width()).is_less_than_or_equal_to(72);
        assert_that!(dynamic_image.width()).is_greater_than_or_equal_to(68);
        assert_that!(dynamic_image.height()).is_less_than_or_equal_to(144);
        assert_that!(dynamic_image.height()).is_greater_than_or_equal_to(140);

        Ok(())
    }

    #[test]
    fn fails_when_source_file_does_not_exist() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        let dest_path = dir.path().join("dest");
        fs_err::create_dir(section_path)?;
        let source_image_path = Path::new("./src/does-not-exist.jpg");

        fs_err::create_dir(&dest_path)?;
        let dest_image_path = dest_path.join("abc.dest.jpg");

        let res = save_as_resized_image(source_image_path, dest_image_path.as_path(), 0.5);
        let err_desc = assert_that!(res).is_err().subject.to_string();
        assert_that!(err_desc).contains("source file");

        Ok(())
    }

    #[test]
    fn fails_when_destination_file_already_exists() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        let dest_path = dir.path().join("dest");
        fs_err::create_dir(section_path)?;
        let source_image_path = Path::new("./src/empty-100x200.jpg");

        fs_err::create_dir(&dest_path)?;
        let dest_image_path = dest_path.join("abc.dest.jpg");
        fs_err::File::create(&dest_image_path)?;

        let res = save_as_resized_image(source_image_path, dest_image_path.as_path(), 0.5);
        let err_desc = assert_that!(res).is_err().subject.to_string();
        assert_that!(err_desc).contains("destination file");

        Ok(())
    }
}
