use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Ok};

mod image_operations;
mod markdown_output;

#[derive(Eq, PartialEq, Debug, Clone)]
struct ImagePath(PathBuf);

impl ImagePath {
    fn small_image_path(&self, output_root: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        Ok(output_root
            .as_ref()
            .join(self.create_output_file_name("small")?))
    }

    fn large_image_path(&self, output_root: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        Ok(output_root
            .as_ref()
            .join(self.create_output_file_name("large")?))
    }

    fn create_output_file_name(&self, suffix: &str) -> anyhow::Result<PathBuf> {
        let path = &self.0;
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

        Ok(PathBuf::from(parent_file_name)
            .join(stem)
            .with_extension(extension))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Section {
    name: String,
    image_files: Vec<ImagePath>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct OutputImageFiles {
    small_image: String,
    large_image: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct SectionForOutput {
    name: String,
    image_files: Vec<OutputImageFiles>,
}

#[derive(Debug)]
struct Minutes {
    sections: Vec<Section>,
}

#[derive(Debug)]
struct MinutesForOutput {
    sections: Vec<SectionForOutput>,
}

fn create_minutes(path: &Path) -> anyhow::Result<Minutes> {
    let dir = fs::read_dir(path)?;
    let names = dir.map(|e| create_section(e?));
    Ok(Minutes {
        sections: names.collect::<anyhow::Result<Vec<_>>>()?,
    })
}

fn create_section(dir_entry: DirEntry) -> anyhow::Result<Section> {
    let dir = fs::read_dir(dir_entry.path())?;
    let image_files = dir
        .map(|e| Ok(ImagePath(e?.path())))
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Section {
        name: dir_entry.file_name().to_string_lossy().to_string(),
        image_files,
    })
}

fn create_dest_image_path(
    source_image_path: impl AsRef<Path>,
    dest_image_root_path: impl AsRef<Path>,
) -> anyhow::Result<PathBuf> {
    if source_image_path.as_ref().is_relative() {
        return Err(anyhow!(
            "source path must be absolute but is: {}",
            source_image_path.as_ref().to_string_lossy()
        ));
    }

    let file_name = source_image_path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("no file"))?;
    Ok(dest_image_root_path.as_ref().join(file_name))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::create_dir;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use speculoos::prelude::*;

    use crate::{create_dest_image_path, create_minutes, ImagePath, Section};

    #[test]
    fn minutes_from_non_existing_parent_dir_is_err() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;

        assert_that!(create_minutes(&dir.path().join("does_not_exist"))).is_err();
        Ok(())
    }

    #[test]
    fn minutes_from_no_session_directory() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;

        let minutes = create_minutes(dir.path())?;

        assert_that!(minutes.sections).is_empty();
        Ok(())
    }

    #[test]
    fn minutes_from_single_session_directory_without_images() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        create_dir(dir.path().join("abc"))?;

        let minutes = create_minutes(dir.path())?;

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

        let minutes = create_minutes(dir.path())?;

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

        let minutes = create_minutes(dir.path())?;

        let mut paths = minutes.sections.into_iter().flat_map(|s| s.image_files);
        assert_that!(paths.all(|p| p.0.is_absolute())).is_true();
        Ok(())
    }

    #[test]
    fn create_dest_image_path_absolute() {
        let source_image_path = Path::new("/a/b/c.jpg");
        let dest_image_root_path = Path::new("/a/x");

        let dest_image_path = create_dest_image_path(source_image_path, dest_image_root_path);
        assert_that!(dest_image_path).is_ok_containing(&PathBuf::from("/a/x/c.jpg"));
    }

    #[test]
    fn create_dest_image_path_relative_source_path_results_in_err() {
        let source_image_path = Path::new("a/b/c.jpg");
        let dest_image_root_path = Path::new("/a/x");

        let dest_image_path = create_dest_image_path(source_image_path, dest_image_root_path);
        assert_that!(dest_image_path)
            .is_err()
            .matches(|e| e.to_string().contains("source path must be absolute"));
    }

    #[test]
    fn image_path_create_small_image_path() {
        let image_path = ImagePath(PathBuf::from("/input/section-1/1.jpg"));

        assert_that!(image_path.small_image_path(Path::new("/output")))
            .is_ok_containing(PathBuf::from("/output/section-1/1_small.jpg"));
    }

    #[test]
    fn image_path_create_large_image_path() {
        let image_path = ImagePath(PathBuf::from("/input/section-1/1.jpg"));

        assert_that!(image_path.large_image_path(Path::new("/output")))
            .is_ok_containing(PathBuf::from("/output/section-1/1_large.jpg"));
    }
}
