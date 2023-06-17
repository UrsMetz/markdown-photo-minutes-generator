use bpaf::{OptionParser, Parser};

use std::path::PathBuf;

#[derive(Clone, Debug)]
struct ImageConversionOptions {
    input_root_path: PathBuf,
    output_root_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let options = options().run();

    println!("input: {}", options.input_root_path.to_string_lossy());
    println!("output: {}", options.output_root_path.to_string_lossy());

    let minutes =
        markdown_photo_minutes_generator::create_minutes(options.input_root_path.as_path())?;

    let for_output = markdown_photo_minutes_generator::create_minutes_for_output(
        minutes,
        options.output_root_path.as_path(),
    )?;

    for_output
        .sections
        .into_iter()
        .flat_map(|s| s.image_files)
        .try_for_each(|f| {
            markdown_photo_minutes_generator::image_operations::save_as_resized_image(
                f.source_image_path,
                f.small_image,
                0.4,
            )
        })?;

    // for f in for_output.sections.into_iter().flat_map(|s| s.image_files) {
    //     println!("source image: {}", f.source_image_path.to_string_lossy());
    //     println!("small image: {}", f.small_image.to_string_lossy());
    // }
    Ok(())
}

fn options() -> OptionParser<ImageConversionOptions> {
    let input_root_path = bpaf::positional("INPUT");
    let output_root_path = bpaf::positional::<PathBuf>("OUTPUT");

    bpaf::construct!(ImageConversionOptions {
        input_root_path,
        output_root_path
    })
    .to_options()
}
