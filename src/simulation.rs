use std::ptr::null;
use rhdl::prelude::*;
use rhdl::bits;

#[derive(Digital, PartialEq, Eq)]
pub struct ImageMatrix<const WIDTH: usize, const HEIGHT: usize>{
    pub data : [[Bits<U8>; WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> ImageMatrix<WIDTH, HEIGHT>
{
    pub fn create(matrix: Vec<Vec<u8>>) -> Self{
        let data = core::array::from_fn(|y| {
            core::array::from_fn(|x| Bits::<U8>::from(matrix[y][x] as u128))
        });
        Self{data}
    }
}

pub fn simulate(matrix_rgb: Vec<Vec<u8>>, matrix_depth: Vec<Vec<u8>>) -> Result<(), RHDLError>
{
    let image_rgb_bits: ImageMatrix<480, 480> = ImageMatrix::create(matrix_rgb);
    let image_rgb_depth: ImageMatrix<480, 480> = ImageMatrix::create(matrix_depth);

    return Ok(());
}