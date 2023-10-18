
pub mod perception;
pub mod constants;
pub mod io;

use image::{self, GrayImage, ImageBuffer, RgbImage};
use image::GenericImageView; 
use std::{thread, time};
use std::time::{Duration, Instant};

use crate::perception::perceptor::{create_disparity_map};
use crate::io::save::save_gray_image_to_file;




/*
IMAGE row - column convention
----------------> [ii]
|
|
|
v
[jj]
*/


fn main() {
    println!("-----------------------------");
    println!("--  PERCEPTION MODEL mk.0  --");
    println!("-----------------------------");
    /* ---------------------------------------------------------------------- */
    /*                  [load stereo image from file]                         */
    /* ---------------------------------------------------------------------- */
    let start_image_loading = Instant::now();
    /* Load image from assets  */
    let stereo_img_gray = 
        image::open("assets/30_rectified.png")
            .expect("File not found!")
            .into_luma8();
    println!("[MSG] Image loaded successfully.");
    /*
    IMAGE row - column convention
    ----------------> [X]/ii
    |
    |
    |
    v
    [Y]/jj
    */
    /* Obtain the image's width and height. */ 
    let (xstereo, yimg) = stereo_img_gray.dimensions();

    /* Create expected width of left & right image */
    let ximg: u32 = xstereo / 2;

    println!("[MSG] Loaded stereo image width/height : {:?}/{:?}", xstereo,yimg);
    println!("[MSG] Left  image width/height         : {:?}/{:?}", ximg,yimg);
    println!("[MSG] Right image width/height         : {:?}/{:?}", ximg,yimg);
    println!("[MSG] Time to load image               : {:?} [ms] ", start_image_loading.elapsed().as_millis());
    /* ---------------------------------------------------------------------- */
    /*          [split stereo image into left & right image]                  */
    /* ---------------------------------------------------------------------- */
    /* Initialize left and right images */
    let mut img_left: GrayImage = ImageBuffer::new(ximg, yimg);
    let mut img_right: GrayImage = ImageBuffer::new(ximg, yimg);

    /* Assign pixel values from stereo image */
    for ii in 0..ximg
    {
        for jj in 0..yimg
        {
            img_left.put_pixel(ii, jj, *stereo_img_gray.get_pixel(ii, jj));
            img_right.put_pixel(ii, jj, *stereo_img_gray.get_pixel(ii+ximg, jj));
        }
    }
    /* ---------------------------------------------------------------------- */
    /*                  [create disparity map]                                */
    /* ---------------------------------------------------------------------- */
    /* Loop over image rows */
    let start_correlation = Instant::now();

    let disparity_map: GrayImage = create_disparity_map(&img_left,
                                                       &img_right);

    println!(" Time to create disparity map: {:?} [ms] ", start_correlation.elapsed().as_millis());
    
    /* Save results to file  */
    println!("[x] Saving disparity map to: ./out/disparity.png");
    save_gray_image_to_file(&disparity_map, 
                            &"./out/disparity.png").unwrap();

    /* ---------------------------------------------------------------------- */
}
