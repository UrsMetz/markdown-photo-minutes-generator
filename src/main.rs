use std::path::PathBuf;

use bpaf::{OptionParser, Parser};

use lib::OutputImageFiles;
use markdown_photo_minutes_generator as lib;
use markdown_photo_minutes_generator::input::Minutes;
use markdown_photo_minutes_generator::markdown_output;
use markdown_photo_minutes_generator::output::{MinutesForOutput, SectionForOutput};

#[derive(Clone, Debug)]
struct ImageConversionOptions {
    input_root_path: PathBuf,
    output_root_path: PathBuf,
    online_base_path: String,
    skip_image_conversion: bool,
}

fn main() -> anyhow::Result<()> {
    let options = options().run();

    println!("input: {}", options.input_root_path.to_string_lossy());
    println!("output: {}", options.output_root_path.to_string_lossy());

    let minutes = Minutes::try_from(options.input_root_path.as_path())?;

    let for_conversion =
        markdown_photo_minutes_generator::conversion::create_minutes_for_conversion(
            &minutes,
            options.output_root_path.as_path(),
        )?;

    if !options.skip_image_conversion {
        let vec = for_conversion.sections.clone();
        vec.into_iter()
            .flat_map(|s| s.image_files)
            .try_for_each(|f| {
                lib::image_operations::save_as_resized_image(
                    f.source_image_path,
                    f.large_image,
                    1.0,
                )?;
                lib::image_operations::save_as_resized_image(
                    f.source_image_path,
                    f.small_image,
                    0.3,
                )?;
                anyhow::Ok(())
            })?;
    }

    let for_output = into_minutes_for_outputs(for_conversion, options.online_base_path)?;

    let markdown = markdown_output::create_markdown(for_output)?;

    println!("{}", markdown);

    Ok(())
}

fn into_minutes_for_outputs(
    for_conversion: markdown_photo_minutes_generator::conversion::MinutesForConversion,
    online_base_path: String,
) -> anyhow::Result<MinutesForOutput> {
    Ok(MinutesForOutput {
        sections: for_conversion
            .sections
            .iter()
            .map(|s| {
                let image_files: Vec<OutputImageFiles> = s
                    .image_files
                    .iter()
                    .map(|f| OutputImageFiles::create(f, &online_base_path))
                    .collect::<anyhow::Result<_>>()?;
                Ok(SectionForOutput {
                    name: s.name.clone(),
                    image_files,
                })
            })
            .collect::<anyhow::Result<_>>()?,
    })
}

fn options() -> OptionParser<ImageConversionOptions> {
    let input_root_path = bpaf::positional("INPUT");
    let output_root_path = bpaf::positional::<PathBuf>("OUTPUT");
    let online_base_path = bpaf::positional::<String>("ONLINE_BASE_PATH");
    let skip_image_conversion = bpaf::long("skip-image-conversion")
        .argument::<bool>("SKIP_IMAGE_CONVERSION")
        .fallback(false);

    bpaf::construct!(ImageConversionOptions {
        input_root_path,
        output_root_path,
        online_base_path,
        skip_image_conversion,
    })
    .to_options()
}
