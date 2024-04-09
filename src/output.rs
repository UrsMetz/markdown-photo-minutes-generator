use std::path::Path;

use anyhow::Context;

use crate::conversion::OutputImageFilesForConversion;
use crate::OutputImageFiles;

impl OutputImageFiles {
    pub fn create(
        source: &OutputImageFilesForConversion,
        online_base_path: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            small_image: Self::create_online_path(&source.small_image, online_base_path.as_ref())?,
            large_image: Self::create_online_path(&source.large_image, online_base_path.as_ref())?,
        })
    }

    fn create_online_path(
        image_path: impl AsRef<Path>,
        online_base_path: &str,
    ) -> anyhow::Result<String> {
        let image_path = image_path.as_ref();
        let base_path = image_path
            .parent()
            .and_then(Path::parent)
            .with_context(|| {
                format!(
                    "Could not derive base path of image {}",
                    image_path.to_string_lossy()
                )
            })?;

        let image_online_path = image_path.strip_prefix(base_path)?;
        anyhow::Ok(format!(
            "{}/{}",
            online_base_path,
            image_online_path.to_string_lossy()
        ))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SectionForOutput {
    pub name: String,
    pub image_files: Vec<OutputImageFiles>,
}

#[derive(Debug)]
pub struct MinutesForOutput {
    pub sections: Vec<SectionForOutput>,
}
