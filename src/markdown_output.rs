use crate::output::{MinutesForOutput, SectionForOutput};

pub fn create_markdown(minutes: MinutesForOutput) -> anyhow::Result<String> {
    let sections = minutes
        .sections
        .into_iter()
        .map(format_section)
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(sections)
}

fn format_section(s: SectionForOutput) -> String {
    let images = s
        .image_files
        .into_iter()
        .map(|f| (f.small_image, f.large_image))
        .map(|image_paths| format!("[![{}]({})]({})", s.name, image_paths.0, image_paths.1))
        .reduce(|acc, cur| format!("{}\n\n{}", acc, cur))
        .unwrap_or("\n".to_string());
    format!("# {}\n\n{}", s.name, images)
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::markdown_output::create_markdown;
    use crate::output::{MinutesForOutput, SectionForOutput};
    use crate::OutputImageFiles;

    #[test]
    fn creates_heading_for_each_section() {
        let minutes = MinutesForOutput {
            sections: vec![
                SectionForOutput {
                    name: "section 1".to_string(),
                    image_files: vec![],
                },
                SectionForOutput {
                    name: "section 2".to_string(),
                    image_files: vec![],
                },
            ],
        };

        let markdown = assert_that!(create_markdown(minutes))
            .is_ok()
            .subject
            .to_string();

        assert_that!(markdown).contains("# section 1\n");
        assert_that!(markdown).contains("# section 2\n");
    }

    #[test]
    fn creates_image_elements_for_each_section() {
        let image_path_1_small = "/a/section_1/1_small.jpg".to_string();
        let image_path_1_large = "/a/section_1/1_large.jpg".to_string();
        let image_path_2_small = "/a/section_1/2_small.jpg".to_string();
        let image_path_2_large = "/a/section_1/2_large.jpg".to_string();
        let image_path_3_small = "/a/section_2/1_small.jpg".to_string();
        let image_path_3_large = "/a/section_2/1_large.jpg".to_string();
        let image_path_4_small = "/a/section_2/2_small.jpg".to_string();
        let image_path_4_large = "/a/section_2/2_large.jpg".to_string();

        let minutes = MinutesForOutput {
            sections: vec![
                SectionForOutput {
                    name: "section 1".to_string(),
                    image_files: vec![
                        OutputImageFiles {
                            small_image: image_path_1_small.clone(),
                            large_image: image_path_1_large.clone(),
                        },
                        OutputImageFiles {
                            small_image: image_path_2_small.clone(),
                            large_image: image_path_2_large.clone(),
                        },
                    ],
                },
                SectionForOutput {
                    name: "section 2".to_string(),
                    image_files: vec![
                        OutputImageFiles {
                            small_image: image_path_3_small.clone(),
                            large_image: image_path_3_large.clone(),
                        },
                        OutputImageFiles {
                            small_image: image_path_4_small.clone(),
                            large_image: image_path_4_large.clone(),
                        },
                    ],
                },
            ],
        };

        let markdown = assert_that!(create_markdown(minutes))
            .is_ok()
            .subject
            .to_string();

        assert_that!(markdown).is_equal_to(format!(
            "# section 1\n\n[![{}]({})]({})\n\n[![{}]({})]({})\n\n# section 2\n\n[![{}]({})]({})\n\n[![{}]({})]({})",
            "section 1",
            image_path_1_small,
            image_path_1_large,
            "section 1",
            image_path_2_small,
            image_path_2_large,
            "section 2",
            image_path_3_small,
            image_path_3_large,
            "section 2",
            image_path_4_small,
            image_path_4_large
        ));
    }
}
