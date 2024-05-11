use std::path::{Path, PathBuf};

use crate::images::SourceImagePath;
use crate::input::{Minutes, Section};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OutputImageFilesForConversion<'source> {
    pub source_image_path: &'source Path,
    pub small_image: PathBuf,
    pub large_image: PathBuf,
}

impl<'source> OutputImageFilesForConversion<'source> {
    fn try_from_image_path(
        source_image_path: &'source SourceImagePath,
        output_base_path: &Path,
    ) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            source_image_path: source_image_path.path(),
            large_image: source_image_path.large_image_path(output_base_path)?,
            small_image: source_image_path.small_image_path(output_base_path)?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SectionForConversion<'source> {
    pub name: &'source str,
    pub image_files: Vec<OutputImageFilesForConversion<'source>>,
}

impl<'source> SectionForConversion<'source> {
    pub fn try_from_section(
        section: &'source Section,
        output_base_path: &Path,
    ) -> anyhow::Result<Self> {
        anyhow::Ok(SectionForConversion {
            name: section.name.as_ref(),
            image_files: section
                .image_files
                .iter()
                .map(|i| OutputImageFilesForConversion::try_from_image_path(i, output_base_path))
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[derive(Debug)]
pub struct MinutesForConversion<'source> {
    pub sections: Vec<SectionForConversion<'source>>,
}

pub fn create_minutes_for_conversion<'source>(
    minutes: &'source Minutes,
    output_base_path: &Path,
) -> anyhow::Result<MinutesForConversion<'source>> {
    let sections: Vec<_> = minutes
        .sections
        .iter()
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
            source_image_path: Path::new("/home/images/source/file"),
            large_image: PathBuf::from("/home/images/a/large_file"),
            small_image: PathBuf::from("/home/images/a/small_file"),
        };
        let online_base_path = "http://localhost/documents";
        let files = OutputImageFiles::create(&source, online_base_path).unwrap();

        assert_that!(files.small_image)
            .is_equal_to("http://localhost/documents/a/small_file".to_string());
        assert_that!(files.large_image)
            .is_equal_to("http://localhost/documents/a/large_file".to_string());
    }
}
