use image::ImageReader;

fn main() {
    let img_rgb_grayscale = ImageReader::open("test_2/1341834649.259514.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();
    let img_depth_grayscale = ImageReader::open("test_2/1341834649.259514.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    // debug pt viitor
    let (width_rgb, height_rgb) = img_rgb_grayscale.dimensions();
    println!("RGB -> width: {}, height: {}", width_rgb, height_rgb);
    let ( width_depth, height_depth) = img_depth_grayscale.dimensions();
    println!("Depth -> width depth: {}, height depth: {}", width_depth, height_depth);

    //matrice pentru a o da simularii
    let mut matrix_rgb: Vec<Vec<u8>> = vec![vec![0; width_rgb as usize]; height_rgb as usize];
    let mut matrix_depth: Vec<Vec<u8>> = vec![vec![0; width_depth as usize]; height_depth as usize];


    //mutam datele in imagine
    for y in 0..height_rgb {
        for x in 0..width_rgb {
            matrix_rgb[y as usize][x as usize] = img_rgb_grayscale.get_pixel(x, y)[0];
        }
    }
    for y in 0..height_depth {
        for x in 0..width_depth {
            matrix_depth[y as usize][x as usize] = img_depth_grayscale.get_pixel(x, y)[0];
        }
    }

    println!("\nDaca nu avem erori, e de bine!");
}
