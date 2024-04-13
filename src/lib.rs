pub mod conversion;
pub mod image_operations;
pub mod images;
pub mod input;
pub mod markdown_output;
pub mod output;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct OutputImageFiles {
    pub small_image: String,
    pub large_image: String,
}
