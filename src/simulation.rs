use std::ptr::null;
use rhdl::prelude::*;
use rhdl::bits;

#[derive(Digital, PartialEq, Eq)]
pub struct ImageMatrix<const WIDTH: usize, const HEIGHT: usize>{
    pub data : [[Bits<U8>; WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> ImageMatrix<WIDTH, HEIGHT>
{
    pub fn create(data: Vec<Vec<u8>>) -> Self{
        
    }
}

pub fn simulate() -> Result<(), RHDLError>
{


    return Ok(());
}