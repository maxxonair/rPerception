
/* 
 *
 * [ CONSTANT ] 
 * 
 * */
/* Pixel correlation search window dimension in row 
 * [<-------|-------->]
 * imin   Pixel     imax
 */
pub const CORR_SEARCH_WINDOW_INROW: u32 = 30;

/* Pixel correlation window half-width
 * @unit: pixel
 */
pub const CORR_CROSS_WIDTH: u32        = 5; 

/* Pixel correlation window half-width
 * @unit: pixel
 */
pub const CORR_CROSS_HEIGHT: u32        = 5; 

/*
 * Minimum valid disparity in pixel
 * @unit: pixel
 */
pub const CORR_MIN_VALID_DISPARITY: u32 = 10;

/*
 * Maximum valid disparity in pixel
 * @unit: pixel
 */
pub const CORR_MAX_VALID_DISPARITY: u32 = 200;
