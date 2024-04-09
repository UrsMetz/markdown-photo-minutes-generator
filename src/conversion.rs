use std::path::{Path, PathBuf};

use crate::input::{Minutes, Section};
use crate::ImagePath;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OutputImageFilesForConversion {
    pub source_image_path: Box<Path>,
    pub small_image: PathBuf,
    pub large_image: PathBuf,
}

impl OutputImageFilesForConversion {
    fn try_from_image_path(image_path: ImagePath, output_base_path: &Path) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            source_image_path: image_path.source_image_path(),
            large_image: image_path.large_image_path(output_base_path)?,
            small_image: image_path.small_image_path(output_base_path)?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct ConversionImageFiles {
    small_image: PathBuf,
    large_image: PathBuf,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SectionForConversion {
    pub name: String,
    pub image_files: Vec<OutputImageFilesForConversion>,
}

impl SectionForConversion {
    pub fn try_from_section(section: Section, output_base_path: &Path) -> anyhow::Result<Self> {
        anyhow::Ok(SectionForConversion {
            name: section.name,
            image_files: section
                .image_files
                .into_iter()
                .map(|i| OutputImageFilesForConversion::try_from_image_path(i, output_base_path))
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[derive(Debug)]
pub struct MinutesForConversion {
    pub sections: Vec<SectionForConversion>,
}

pub fn create_minutes_for_conversion(
    minutes: Minutes,
    output_base_path: &Path,
) -> anyhow::Result<MinutesForConversion> {
    let sections: Vec<_> = minutes
        .sections
        .into_iter()
        .map(|s| SectionForConversion::try_from_section(s, output_base_path))
        .collect::<anyhow::Result<_>>()?;
    anyhow::Ok(MinutesForConversion { sections })
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use speculoos::prelude::*;

    use crate::conversion::OutputImageFilesForConversion;
    use crate::OutputImageFiles;

    #[test]
    fn create_output_images() {
        let source = OutputImageFilesForConversion {
            large_image: PathBuf::from("/home/images/a/large_file"),
            small_image: PathBuf::from("/home/images/a/small_file"),
            source_image_path: Path::new("/home/images/source/file").into(),
        };
        let online_base_path = "http://localhost/documents";
        let files = OutputImageFiles::create(&source, online_base_path).unwrap();

        assert_that!(files.small_image)
            .is_equal_to("http://localhost/documents/a/small_file".to_string());
        assert_that!(files.large_image)
            .is_equal_to("http://localhost/documents/a/large_file".to_string());
    }
}
