use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Section {
    name: String,
    image_files: Vec<PathBuf>,
}

struct Minutes {
    sections: Vec<Section>,
}

fn create_minutes(path: &Path) -> Result<Minutes, String> {
    let dir = fs::read_dir(path).expect("should be a ReadDir");
    let names = dir
        .map(|d| d.expect("should be a dir entry"))
        .map(create_section);
    Ok(Minutes {
        sections: names.collect::<Vec<_>>(),
    })
}

fn create_section(dir_entry: DirEntry) -> Section {
    let dir = fs::read_dir(dir_entry.path()).expect("should be a sub ReadDir");
    let image_files = dir
        .map(|d| d.expect("should be a dir entry"))
        .map(|e| e.path())
        .collect::<Vec<_>>();
    Section {
        name: dir_entry.file_name().to_string_lossy().to_string(),
        image_files,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::create_dir;

    use speculoos::prelude::*;

    use crate::{create_minutes, Section};

    #[test]
    fn minutes_from_no_session_directory() {
        let dir = tempfile::tempdir().expect("temp dir should be created");

        let minutes = create_minutes(dir.path()).expect("minutes should be created");

        assert_that!(minutes.sections).is_empty();
    }

    #[test]
    fn minutes_from_single_session_directory_without_images() {
        let dir = tempfile::tempdir().expect("temp dir should be created");
        create_dir(dir.path().join("abc")).expect("sub dir should be created");

        let minutes = create_minutes(dir.path()).expect("minutes should be created");

        assert_that!(minutes.sections).contains_all_of(&vec![&Section {
            name: "abc".to_string(),
            image_files: vec![],
        }]);
    }

    #[test]
    fn minutes_from_single_session_directory_with_images() {
        let dir = tempfile::tempdir().expect("temp dir should be created");
        let section_path = dir.path().join("abc");
        create_dir(&section_path).expect("sub dir should be created");
        let image_path = section_path.join("abc.jpg");
        fs::File::create(&image_path).expect("file should be created");

        let minutes = create_minutes(dir.path()).expect("minutes should be created");

        assert_that!(minutes.sections).contains_all_of(&vec![&Section {
            name: "abc".to_string(),
            image_files: vec![image_path],
        }]);
    }
}
