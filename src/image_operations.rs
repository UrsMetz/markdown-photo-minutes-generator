use anyhow::Context;
use image::imageops::FilterType;
use std::io::Write;
use std::path::Path;

pub fn save_as_resized_image<S: AsRef<Path>, D: AsRef<Path>>(
    source_image_path: S,
    dest_image_path: D,
    ratio: f32,
) -> anyhow::Result<()> {
    let source_image = image::io::Reader::open(&source_image_path)
        .with_context(|| "source file does not exist")?
        .decode()?;
    let source_width = source_image.width();
    let source_height = source_image.height();
    let new_width = calculate_new_dimension(ratio, source_width);
    let new_height = calculate_new_dimension(ratio, source_height);
    let dest_image = source_image.resize(new_width, new_height, FilterType::Triangle);
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

#[cfg(test)]
mod tests {
    use fs_err;
    use std::path::Path;

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
        let dynamic_image = image::io::Reader::open(dest_image_path)?.decode()?;
        assert_that!(dynamic_image.width()).is_less_than_or_equal_to(72);
        assert_that!(dynamic_image.width()).is_greater_than_or_equal_to(68);
        assert_that!(dynamic_image.height()).is_less_than_or_equal_to(144);
        assert_that!(dynamic_image.height()).is_greater_than_or_equal_to(140);

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
        let dynamic_image = image::io::Reader::open(dest_image_path)?.decode()?;
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
