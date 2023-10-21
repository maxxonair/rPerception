
/* External imports */
use image::{self, GrayImage, ImageBuffer, RgbImage, Luma, GenericImageView, Pixel};

/* Internal imports */
use crate::constants::constants::*;

/*
IMAGE row - column convention
----------------> X [ii]
|
|
|
v
Y [jj]
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
  let mut xmin: u32 = 0;
  let mut xmax: u32 = 0;
  let mut corr_y_max: u32 = 0;
  let mut corr_y_min: u32 = 0;
  let mut corr_x_max: i32 = 0;
  let mut corr_x_min: i32 = 0;
  
  /* Check if image boundaries in y direction are violated by any search window
  * setting 
  */
  if (pix_y as i32 - CORR_CROSS_HEIGHT as i32) < 0
  {
    corr_y_min = 0;
    // TODO: Handle this properly
    corr_y_max = pix_y  +  CORR_CROSS_HEIGHT ;
  }
  else if  pix_y + CORR_CROSS_HEIGHT > y_max {
    // TODO: Handle this properly
    corr_y_min = pix_y - CORR_CROSS_HEIGHT;
    corr_y_max = y_max;
  }
  else 
  {
    corr_y_min = pix_y - CORR_CROSS_HEIGHT;
    corr_y_max = pix_y + CORR_CROSS_HEIGHT;   
  }
  
  if (pix_x as i32 - CORR_MAX_VALID_DISPARITY as i32) > 0
  {
    xmin = pix_x - CORR_MAX_VALID_DISPARITY;
  } /* else leave xmin to initialised 0 */
  
  /* If this condition is true we are too close to the left edge of the image 
  * to compute a valid disparity  */
  if (pix_x as i32 - CORR_MIN_VALID_DISPARITY as i32)  < 0
  {
    return disparity_distance;
  }
  else 
  {
    xmax = pix_x - CORR_MIN_VALID_DISPARITY ;
  }
  /* Number of disparity values to explore */
  let num_disparity_vals = xmax - xmin;

  /* For some reason the range  of valid disparities is equal to zero we can 
   * call it a day here */
  if num_disparity_vals == 0
  {
    return disparity_distance;
  }

  /* Initialise variables to track min/max SAD and disparity associated to 
   * minimum SAD */
  let mut min_sum_abs_diff: u32 = 0;
  let mut disp_at_min_abs_diff: u32 = 0;
  let mut max_sum_abs_diff: u32 = 0;

  /* Loop over all valid disparities in search window */
  for ii in xmin..xmax
  {
    /* Update x boundaries for search window with disparity ii */
    /* Set search window boundaries */
    if ((ii as i32 - CORR_CROSS_WIDTH as i32) as i32) < 0
    {
      corr_x_min = 0;
      corr_x_max = (2 * CORR_CROSS_WIDTH) as i32;
    }
    else if pix_x + CORR_CROSS_WIDTH > x_max  {
      corr_x_min =  - ( 2 * CORR_CROSS_WIDTH as i32 );
      corr_x_max = 0;
    }
    else 
    {
      corr_x_min =  - (CORR_CROSS_WIDTH as i32);
      corr_x_max =  CORR_CROSS_WIDTH as i32;
    }

    /* Initialise variable to store the absolute sum of all differences for each
     * search window and associated disparity value */
    let mut sum_of_differences: u32 = 0;

    /* Loop over all pixel in search window */
    for kk in corr_x_min..corr_x_max
    {
      for ll in corr_y_min..corr_y_max
      {
        let pix_left = img_l_in.get_pixel((pix_x as i32 + kk) as u32, ll as u32);
        let pix_right = img_r_in.get_pixel((ii as i32 + kk) as u32, ll as u32);
        let abs_local_diff: u32 = (pix_left.to_luma().0[0] as i32 
                                - pix_right.to_luma().0[0] as i32).abs() as u32;
        sum_of_differences += abs_local_diff;
      } /* ll */
    } /* kk */

    /* [ Update local maximum and minimum SAD ] */
    /* If first disparity step of current sum is smaller than tracked minimum 
     * sum */
    if ii == xmin || sum_of_differences < min_sum_abs_diff
    {
      min_sum_abs_diff = sum_of_differences;
      disp_at_min_abs_diff = ii;
    }
    /* If first disparity step or current sum is larger than track maximum
     * sum */
    if ii == xmin || sum_of_differences > max_sum_abs_diff
    {
      max_sum_abs_diff = sum_of_differences;
    }

  } /* ii */

  if max_sum_abs_diff == 0 
  {
    return disparity_distance;
  }
  
  /* Check if that's a valid detection -> Otherwise reject disparity minimum */
  if ((min_sum_abs_diff / max_sum_abs_diff) as f64) 
        < THR_MAX_RATIO_MIN_TO_MAX_SUM_DIFF
  {
    disparity_distance = disp_at_min_abs_diff as u8;
  }

  disparity_distance
}


pub fn create_disparity_map(img_left_in: &GrayImage, img_right_in: &GrayImage)
-> GrayImage
{
  let mut disparity_value: u8 = 0;
  let (xl, yl) = img_left_in.dimensions();
  let (xr, yr) = img_right_in.dimensions();
  
  /* Check that left and rifght image dimension match */
  if xl != xr 
  {
    panic!("Number of pixels in x are not identical between left and right image! {:?} vs {:?}", xl, xr);
  }
  else if yl != yr 
  {
    panic!("Number of pixels in y are not identical between left and right image! {:?} vs {:?}", xl, xr);
  } 

  /* Create empty disparity map as GrayImage */
  let mut disparity_map: GrayImage = ImageBuffer::new(xl, yl); 

  for ii in 0..xl-1
  {
    for jj in 0..yl-1
    {
      /* Perform perception on current pixel -> return disparity distance */
      disparity_value = get_disparity_match(&img_left_in, 
                                            &img_right_in,
                                            ii, jj,
                                            xl-1, yl-1);

      /* Assign value to disparity map pixel */
      disparity_map.put_pixel(ii, jj, Luma([disparity_value]))


    } /* jj */
    /* Print progess */
    // println!("Progress {:000.1?} percent", ii as f64/(xl as f64 - 1.0) * 100.0);
  } /* ii */
  disparity_map
}