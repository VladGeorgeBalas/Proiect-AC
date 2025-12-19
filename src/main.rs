mod simulation;
mod doc;
mod mask;
mod circ_seq;

use crate::circ_seq::simulate;
use image::ImageReader;

fn main() {

    // CONFIG!!

    let graphs = false;
    let clocks : u64 = 10;



    let file_rgb = "Test_3/square_1.png";
    let file_depth = "Test_3/square_3.png";

    let img_rgb_grayscale = ImageReader::open(file_rgb)
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();
    let img_depth_grayscale = ImageReader::open(file_depth)
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    // da momentan 640 x 480
    let (width_rgb, height_rgb) = img_rgb_grayscale.dimensions();
    println!("RGB -> width: {}, height: {}", width_rgb, height_rgb);
    let (width_depth, height_depth) = img_depth_grayscale.dimensions();
    println!(
        "Depth -> width depth: {}, height depth: {}",
        width_depth, height_depth
    );

    //matrice pentru a o da simularii
    let mut matrix_rgb: Vec<Vec<u8>> = vec![vec![0; width_rgb as usize]; height_rgb as usize];
    let mut matrix_depth: Vec<Vec<u8>> = vec![vec![0; width_depth as usize]; height_depth as usize];

    // trim-ul este dat din topor
    // TODO: la final DE TOT, de facut dinamic asta

    //mutam datele in imagine
    for y in 0..height_rgb {
        for x in 0..width_rgb {
            matrix_rgb[y as usize][x as usize] = if ( img_rgb_grayscale.get_pixel(x, y)[0] > 128) {1} else {0} ;
        }
    }
    for y in 0..height_depth {
        for x in 0..width_depth {
            matrix_depth[y as usize][x as usize] = if ( img_depth_grayscale.get_pixel(x, y)[0] > 128) {1} else {0};
        }
    }

    img_rgb_grayscale.save("out_rgb_gray.png").unwrap();
    img_depth_grayscale.save("out_depth_gray.png").unwrap();

    let _ = simulate(matrix_rgb, matrix_depth, graphs, clocks);
}
