
use image::{GrayImage, Luma};

/*
 * @brief: Save GrayImage to file   
 */
pub fn save_gray_image_to_file(image: &GrayImage, output_path: &str) 
-> Result<(), image::ImageError> 
{
  image.save(output_path)
}