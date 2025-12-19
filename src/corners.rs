use rhdl::bits;
use rhdl::prelude::*;
use rhdl_fpga::core::dff::DFF;
use std::ptr::null;
use crate::simulation::ImageMatrix;

/*
   STAGE 2 din .txt

   Kernel detector de colturi
   Folosim patch-uri direct, ca dam mem overflow daca sunt imagini mari;

*/

// Stage 2
// kernel pentru gasire colturi in imagine
// circuit combinational
//#[kernel]
/*
pub fn mem_mapper(
    i: Bits<U8>,  // pasul (index liniar)
    w: Bits<U8>,  // width
    h: Bits<U8>,  // height (nefolosit aici, dar îl păstrăm)
) -> (Bits<U8>, Bits<U8>, Bits<U8>, Bits<U8>) {

    // Evită împărțirea la 0 când w == 0 sau w == 1
    if w <= bits(1) {
        return (bits(0), bits(0), bits(0), bits(0));
    }

    let w_minus_1 = w - bits(1);
    let skip = i / w_minus_1;          // i / (w-1)
    let base = i + skip;               // i + i/(w-1)

    let b1 = base;
    let b2 = base + bits(1);
    let b3 = base + w;
    let b4 = base + w + bits(1);

    (b1, b2, b3, b4)
}
*/

/*
#[kernel]
pub fn corner_finder(p1 : Bits<U8>, p2 : Bits<U8>, p3 : Bits<U8>, p4:Bits<U8>) -> Bits<U8>{
    // _i[1] : imaginea RGB
    // _i[2] : imaginea Depth

    // raspuns la colturine RBG
    let mut res = (p1.as_signed() + p4.as_signed()) - (p2.as_signed() + p3.as_signed());
    if res < bits(0).as_signed() {
        res = -res;
    }

    res.as_unsigned()
}



// Cod INUTIL !!!
// fost cod de test, nu folosi

use crate::doc::write_svg;
use crate::simulation::ImageMatrix;

pub fn simulate(matrix_rgb: Vec<Vec<u8>>, matrix_depth: Vec<Vec<u8>>) -> Result<(), RHDLError> {
    // aici folosim 480x480
    //let image_rgb_bits: ImageMatrix<480, 480> = ImageMatrix::create(matrix_rgb);
    //let image_rgb_depth: ImageMatrix<480, 480> = ImageMatrix::create(matrix_depth);
    let test_matrix: ImageMatrix<4, 4> = ImageMatrix {
        data: core::array::from_fn(|_| core::array::from_fn(|_| Bits::<U8>::from(0u128))),
    };

    println!("Avem input");

    let input = vec![test_matrix]
        .into_iter()
        .with_reset(1)
        .clock_pos_edge(100);
    let circ = CornerDetect::default();

    println!("Ne pregatim de run");
    let vcd = circ.run(input)?.collect::<Vcd>();

    println!("Ajungem la printat");
    //let _ = vcd.dump_to_file("ram.vcd")?;
    let _ = write_svg(vcd, "reg.svg");

    return Ok(());
}
*/

// TODO: Poate putem cumva clarifica ce vedem? sa dam plot cumva la pixeli sa clarificam care sunt
