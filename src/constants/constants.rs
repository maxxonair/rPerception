
/* 
 *
 * [ CONSTANT ] 
 * 
 * ---------------------------------------------------------------------------*/

/* Pixel correlation window half-width
 * @unit: pixel
 */
pub const CORR_CROSS_WIDTH: u32        = 5; 

/* Pixel correlation window half-width
 * @unit: pixel
 */
pub const CORR_CROSS_HEIGHT: u32       = 5; 

/*
 * Minimum valid disparity in pixel
 * @unit: pixel
 */
pub const CORR_MIN_VALID_DISPARITY: u32 = 10;

/*
 * Maximum valid disparity in pixel
 * NOTE: Currently disparity maps are saved to u8 GrayImage. So keep in mind that 
 *       the maximum disparity value that can be saved is 255!
 * @unit: pixel
 */
pub const CORR_MAX_VALID_DISPARITY: u32 = 200;

/*
 * Threshold defining the maximum ratio 
 * min_sum_of_abs_differences / max_sum_of_abs_differences to declare the 
 * disparity for the respective pixel valid
 * @unit: N/A
 */
pub const THR_MAX_RATIO_MIN_TO_MAX_SUM_DIFF: f64 = 0.35;

/*
 * Laplacian image filter kernel size 
 * @unit: pixel
 */
pub const LAPLACIAN_FILTER_KERNEL_PX: u32 = 5;