use image::{GenericImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Primitive};
use imageproc::integral_image::{column_running_sum, row_running_sum};

pub fn laplacian_filter(image: &GrayImage, x_radius: u32, y_radius: u32) -> GrayImage {
  let (width, height) = image.dimensions();
  let mut out: GrayImage = GrayImage::new(width, height);
  if width == 0 || height == 0 {
      return out;
  }

  let kernel_width = 2 * x_radius + 1;
  let kernel_height = 2 * y_radius + 1;

  let mut row_buffer = vec![0; (width + 2 * x_radius) as usize];
  for y in 0..height {
      row_running_sum(image, y, &mut row_buffer, x_radius);
      let val = row_buffer[(2 * x_radius) as usize] / kernel_width;
      unsafe {
          out.unsafe_put_pixel(0, y, Luma([val as u8]));
      }
      for x in 1..width {
          // TODO: This way we pay rounding errors for each of the
          // TODO: x and y convolutions. Is there a better way?
          let u = (x + 2 * x_radius) as usize;
          let l = (x - 1) as usize;
          let val = (row_buffer[u] - row_buffer[l]) / kernel_width;
          unsafe {
              out.unsafe_put_pixel(x, y, Luma([val as u8]));
          }
      }
  }

  let mut col_buffer = vec![0; (height + 2 * y_radius) as usize];
  for x in 0..width {
      column_running_sum(&out, x, &mut col_buffer, y_radius);
      let val = col_buffer[(2 * y_radius) as usize] / kernel_height;
      unsafe {
          out.unsafe_put_pixel(x, 0, Luma([val as u8]));
      }
      for y in 1..height {
          let u = (y + 2 * y_radius) as usize;
          let l = (y - 1) as usize;
          let val = (col_buffer[u] - col_buffer[l]) / kernel_height;
          unsafe {
              out.unsafe_put_pixel(x, y, Luma([val as u8]));
          }
      }
  }

  out
}