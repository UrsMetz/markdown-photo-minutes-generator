use std::path::Path;

use fs_err;

use crate::ImagePath;

#[derive(Debug)]
pub struct Minutes {
    pub sections: Vec<Section>,
}

impl TryFrom<&Path> for Minutes {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let dir = fs_err::read_dir(value)?;
        let names = dir.map(|e| Section::try_from(e?));
        anyhow::Ok(Self {
            sections: names.collect::<anyhow::Result<Vec<_>>>()?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Section {
    pub name: String,
    pub image_files: Vec<ImagePath>,
}

impl TryFrom<fs_err::DirEntry> for Section {
    type Error = anyhow::Error;

    fn try_from(value: fs_err::DirEntry) -> Result<Self, Self::Error> {
        let dir = fs_err::read_dir(value.path())?;
        let image_files = dir
            .map(|e| anyhow::Ok(ImagePath(e?.path())))
            .collect::<anyhow::Result<Vec<_>>>()?;
        anyhow::Ok(Section {
            name: value.file_name().to_string_lossy().to_string(),
            image_files,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::create_dir;

    use speculoos::prelude::*;

    use crate::input::{Minutes, Section};
    use crate::ImagePath;

    #[test]
    fn minutes_from_non_existing_parent_dir_is_err() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;

        let non_existing_path = dir.path().join("does_not_exist");
        let result = Minutes::try_from(non_existing_path.as_path());
        assert_that!(result).is_err();
        Ok(())
    }

    #[test]
    fn minutes_from_no_session_directory() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;

        let minutes = Minutes::try_from(dir.path())?;

        assert_that!(minutes.sections).is_empty();
        Ok(())
    }

    #[test]
    fn minutes_from_single_session_directory_without_images() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        create_dir(dir.path().join("abc"))?;

        let path = dir.path();
        let minutes = Minutes::try_from(path)?;

        assert_that!(minutes.sections).contains_all_of(&vec![&Section {
            name: "abc".to_string(),
            image_files: vec![],
        }]);
        Ok(())
    }

    #[test]
    fn minutes_from_single_session_directory_with_images() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        create_dir(&section_path)?;
        let image_path = ImagePath(section_path.join("abc.jpg"));
        fs::File::create(&image_path.0)?;

        let path = dir.path();
        let minutes = Minutes::try_from(path)?;

        assert_that!(minutes.sections).contains_all_of(&vec![&Section {
            name: "abc".to_string(),
            image_files: vec![image_path],
        }]);
        Ok(())
    }

    #[test]
    fn image_paths_are_absolute() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let section_path = dir.path().join("abc");
        create_dir(&section_path)?;
        let image_path = section_path.join("abc.jpg");
        fs::File::create(image_path)?;

        let minutes = Minutes::try_from(dir.path())?;

        let mut paths = minutes.sections.into_iter().flat_map(|s| s.image_files);
        assert_that!(paths.all(|p| p.0.is_absolute())).is_true();
        Ok(())
    }
}
