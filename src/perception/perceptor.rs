/* External imports */
use image::{self, GrayImage, ImageBuffer, RgbImage, Luma, GenericImageView, Pixel};
use ndarray::{Array1, Array2};

/* Internal imports */
use crate::constants::constants::*;

/*
IMAGE row - column convention
----------------> [ii]
|
|
|
v
[jj]
*/

/*
 * @brief: perform pixel correlation search between left and right image for 
 *         a single pixel coordinate.
 * 
 * @out: Return pixel distance between left and right image for best correlation
 */
fn get_disparity_match(img_l_in: &GrayImage, img_r_in: &GrayImage, pix_x: u32, pix_y: u32, 
x_max: u32, y_max: u32) 
-> u8 
{
  /* Initialise temporary variables 
   */
  let mut disparity_distance: u8 = 0;
  let mut xmin = 0;
  let mut xmax = 0;
  let mut corr_y_max = 0;
  let mut corr_y_min = 0;
  let mut corr_x_max = 0;
  let mut corr_x_min = 0;
  /* TODO might be worth to store the correlation window differences ? */
  // let mut array = Array2::zeros((2 * CORR_CROSS_HEIGHT,
  //                                (2 * CORR_CROSS_WIDTH)));
  
  /* Check if image boundaries in y direction are violated by any search window
  * setting 
  */
  if pix_y - CORR_CROSS_HEIGHT < 0
  {
    corr_y_min = 0;
    corr_y_max = pix_y + 2 * CORR_CROSS_HEIGHT;
  }
  else if  pix_y + CORR_CROSS_HEIGHT > y_max {
    corr_y_min = pix_y - ( 2 * CORR_CROSS_HEIGHT );
    corr_y_max = 0;
  }
  else 
  {
    corr_y_min = pix_y - CORR_CROSS_HEIGHT;
    corr_y_max = pix_y + CORR_CROSS_HEIGHT;   
  }
  
  if pix_x - CORR_MAX_VALID_DISPARITY > 0
  {
    xmin = pix_x - CORR_MAX_VALID_DISPARITY;
  } 
  
  /* If this condition is true we are too close to the left edge of the image 
  * to compute a valid disparity  */
  if pix_x - CORR_MIN_VALID_DISPARITY < 0
  {
    return disparity_distance;
  }
  else 
  {
    xmax = pix_x - CORR_MIN_VALID_DISPARITY ;
  }
  /* Number of disparity values to explore */
  let num_disparity_vals = xmax - xmin;
  /* Array to store cost (sum of abs differences for each pixel in the search 
  *  window) for each explored disparity value */
  let mut list_sum_local_abs_diff_per_disp: Array1<u32> = Array1::zeros(num_disparity_vals as usize);
  let mut list_of_explored_disparities: Array1<u32> = Array1::zeros(num_disparity_vals as usize);
  /* Index tracker */
  let mut disp_counter = 0 ;
  /* Loop over all valid disparities in search window */
  for ii in xmin..xmax
  {
    /* Set search window boundaries */
    if ii - CORR_CROSS_WIDTH < 0
    {
      corr_x_min = 0;
      corr_x_max = ii + 2 * CORR_CROSS_WIDTH;
    }
    else if  ii + CORR_CROSS_WIDTH > x_max {
      corr_x_min = ii - ( 2 * CORR_CROSS_WIDTH );
      corr_x_max = 0;
    }
    else 
    {
      corr_x_min = ii - CORR_CROSS_WIDTH;
      corr_x_max = ii + CORR_CROSS_WIDTH;
    }
    let mut sum_of_differences = 0;
    /* Loop over all pixel in search window */
    for kk in corr_x_min..corr_x_max
    {
      for ll in corr_y_min..corr_y_max
      {
        let pix_left = img_l_in.get_pixel(pix_x, pix_y);
        let pix_right = img_r_in.get_pixel(kk, ll);
        let abs_local_diff: u32 = (pix_left.to_luma().0[0] as i32 
                                - pix_right.to_luma().0[0] as i32).abs() as u32;
        sum_of_differences += abs_local_diff;
      } /* ll */
    } /* kk */
    list_sum_local_abs_diff_per_disp[disp_counter] = sum_of_differences;
    list_of_explored_disparities[disp_counter] = ii;
    disp_counter += 1;
  } /* ii */

  /* Find minimum */
  for ii in xmin..xmax
  {

  }
  let (min_value, min_index) = find_minimum_value_and_index(&list_sum_local_abs_diff_per_disp);

  disparity_distance = min_value as u8;

  disparity_distance
}

/*
 * @brief: Helper function to get the minimum value and associated index of a 
 *         Array1<u32> 
 */
fn find_minimum_value_and_index(arr: &Array1<u32>) -> (u32, usize) {
  let (index, min_value) = arr
      .iter()
      .enumerate()
      .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
      .unwrap();
  (*min_value, index)
}

pub fn create_disparity_map(img_left_in: &GrayImage, img_right_in: &GrayImage)
-> GrayImage
{
  let mut disparity_value: u8 = 0;
  let (xl, yl) = img_left_in.dimensions();
  let (xr, yr) = img_right_in.dimensions();
  // TODO: Add check left aqnd rifght image dimesion match

  if xl != xr 
  {
    panic!("Number of pixels in x are not identical between left and right image! {:?} vs {:?}", xl, xr);
  }
  else if yl != yr 
  {
    panic!("Number of pixels in y are not identical between left and right image! {:?} vs {:?}", xl, xr);
  } 

  let mut disparity_map: GrayImage = ImageBuffer::new(xl, yl); 

  for ii in 0..xl
  {
    /* Loop through row pixel */
    for jj in 0..yl
    {
      /* TODO: Perform perception on current pixel -> return disparity distance */
      disparity_value = get_disparity_match(&img_left_in, 
                                            &img_right_in,
                                            ii, jj,
                                            xl, yl);
      disparity_map.put_pixel(ii, jj, Luma([disparity_value]))

      /* TODO: Fill disparity map  */
    }
    }
  disparity_map
}