use rhdl::prelude::*;
use rhdl_fpga::core::dff::DFF;
use rhdl::bits;
use std::ptr::null;

/*

   Prolog fisier:

   Tipuri de baza pentru lucrul cu fisiere.
   ImageMatrix<h, w> -> un patch de imagine
   TransformMatrix -> 2x2 matrice de transformare

*/

#[derive(Digital, PartialEq, Eq)]
pub struct ImageMatrix<const WIDTH: usize, const HEIGHT: usize> {
    pub data: [[Bits<8>; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for ImageMatrix<WIDTH, HEIGHT> {
    fn default() -> Self {
        let data = core::array::from_fn(|y| core::array::from_fn(|x| Bits::<8>::from(0 as u128)));
        Self { data }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Clone for ImageMatrix<WIDTH, HEIGHT> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Copy for ImageMatrix<WIDTH, HEIGHT> {}

impl<const WIDTH: usize, const HEIGHT: usize> ImageMatrix<WIDTH, HEIGHT> {
    pub fn create(matrix: Vec<Vec<u8>>) -> Self {
        let data = core::array::from_fn(|y| {
            core::array::from_fn(|x| Bits::<8>::from(matrix[y][x] as u128))
        });
        Self { data }
    }
}
/* TODO: trebuie cumva separata imaginea in patch-uri de 2x2 si tinuta minte. Nu faceti iesirea Sequential
ca se buleste de la marime.
*/
