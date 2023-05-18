use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Section {
    name: String,
    image_files: Vec<PathBuf>,
}

#[derive(Debug)]
struct Minutes {
    sections: Vec<Section>,
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
        .map(|e| Ok(e?.path()))
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Section {
        name: dir_entry.file_name().to_string_lossy().to_string(),
        image_files,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::create_dir;

    use speculoos::prelude::*;

    use crate::{create_minutes, Section};

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
        let image_path = section_path.join("abc.jpg");
        fs::File::create(&image_path)?;

        let minutes = create_minutes(dir.path())?;

        assert_that!(minutes.sections).contains_all_of(&vec![&Section {
            name: "abc".to_string(),
            image_files: vec![image_path],
        }]);
        Ok(())
    }
}
