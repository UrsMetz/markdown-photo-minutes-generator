use std::ffi::OsString;
use std::path::{Path, PathBuf};

use anyhow::anyhow;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ImagePath(PathBuf);

impl ImagePath {
    pub fn new(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }

    pub fn source_image_path(&self) -> Box<Path> {
        Box::from(self.0.as_path())
    }

    pub fn small_image_path(&self, output_root: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        anyhow::Ok(
            output_root
                .as_ref()
                .join(self.create_output_file_name("small")?),
        )
    }

    pub fn large_image_path(&self, output_root: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        anyhow::Ok(
            output_root
                .as_ref()
                .join(self.create_output_file_name("large")?),
        )
    }

    fn create_output_file_name(&self, suffix: &str) -> anyhow::Result<PathBuf> {
        let path = self.0.as_path();
        let path_str = path.to_string_lossy();

        let parent_file_name = path
            .parent()
            .ok_or(anyhow!("path <{}> has no parent", path_str))?
            .file_name()
            .ok_or(anyhow!("cannot find direct parent for {}", path_str))?;

        let mut stem: OsString = path
            .file_stem()
            .ok_or(anyhow!("path has no file stem {}", path_str))?
            .into();
        stem.push("_");
        stem.push(suffix);
        let extension = path
            .extension()
            .ok_or(anyhow!("path <{}> has no extension", path_str))?;

        anyhow::Ok(
            PathBuf::from(parent_file_name)
                .join(stem)
                .with_extension(extension),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use speculoos::prelude::*;

    use crate::images::ImagePath;

    #[test]
    fn image_path_create_small_image_path() {
        let image_path = ImagePath::new(PathBuf::from("/input/section-1/1.jpg"));

        assert_that!(image_path.small_image_path(Path::new("/output")))
            .is_ok_containing(PathBuf::from("/output/section-1/1_small.jpg"));
    }

    #[test]
    fn image_path_create_large_image_path() {
        let image_path = ImagePath::new(PathBuf::from("/input/section-1/1.jpg"));

        assert_that!(image_path.large_image_path(Path::new("/output")))
            .is_ok_containing(PathBuf::from("/output/section-1/1_large.jpg"));
    }
}
