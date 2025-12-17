use rhdl::bits;
use rhdl::prelude::*;
use rhdl_fpga::core::dff::DFF;
use std::ptr::null;

/*
   STAGE 2 din .txt

   Kernel detector de colturi
   Folosim patch-uri direct, ca dam mem overflow daca sunt imagini mari;

*/

#[derive(Synchronous, SynchronousDQ, Clone, Debug)]
pub struct CornerDetect {
    pub memory: DFF<Bits<U16>>,
}

impl Default for CornerDetect {
    fn default() -> Self {
        Self {
            memory: DFF::new(Bits::<U16>::default()),
        }
    }
}

impl SynchronousIO for CornerDetect {
    type I = ImageMatrix<4, 4>;
    type O = Bits<U8>;
    type Kernel = corner_kernel;
}

// Stage 2
// kernel pentru gasire colturi in imagine
#[kernel]
pub fn corner_kernel(_cr: ClockReset, _i: ImageMatrix<4, 4>, _q: Q) -> (Bits<U8>, D) {
    // _i[1] : imaginea RGB
    // _i[2] : imaginea Depth

    // raspuns la colturine RBG
    let a: s8 = _i.data[0][0].as_signed();
    let b: s8 = _i.data[0][1].as_signed();
    let c: s8 = _i.data[1][0].as_signed();
    let d: s8 = _i.data[1][1].as_signed();

    let mut res = (a + d) - (c + b);
    if res < bits(0).as_signed() {
        res = -res;
    }

    (res.as_unsigned(), D { memory: bits(0) })
}

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

// TODO: Poate putem cumva clarifica ce vedem? sa dam plot cumva la pixeli sa clarificam care sunt
