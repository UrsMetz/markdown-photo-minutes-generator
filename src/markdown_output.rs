use crate::{MinutesForOutput, SectionForOutput};

fn create_markdown(minutes: MinutesForOutput) -> anyhow::Result<String> {
    let sections = minutes
        .sections
        .into_iter()
        .map(format_section)
        .collect::<String>();

    Ok(sections)
}

fn format_section(s: SectionForOutput) -> String {
    let images = s
        .image_files
        .into_iter()
        .map(|f| f.small_image)
        .map(|uri| format!("![{}]()", uri))
        .reduce(|acc, cur| format!("{}\n\n{}", acc, cur))
        .unwrap_or("\n".to_string());
    format!("# {}\n\n{}", s.name, images)
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use crate::markdown_output::create_markdown;
    use crate::{MinutesForOutput, OutputImageFiles, Section, SectionForOutput};

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

        let minutes = MinutesForOutput {
            sections: vec![SectionForOutput {
                name: "section 1".to_string(),
                image_files: vec![
                    OutputImageFiles {
                        small_image: image_path_1_small.clone(),
                        large_image: image_path_1_large,
                    },
                    OutputImageFiles {
                        small_image: image_path_2_small.clone(),
                        large_image: image_path_2_large,
                    },
                ],
            }],
        };

        let markdown = assert_that!(create_markdown(minutes))
            .is_ok()
            .subject
            .to_string();

        assert_that!(markdown).contains(format!(
            "# section 1\n\n![{}]()\n\n![{}]()",
            image_path_1_small, image_path_2_small
        ));
    }
}
