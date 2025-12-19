//! Modul pentru testul circuitelor combinationale. Nu are niciun impact asupra\
//! restului proiectului

use rhdl::bits;
use rhdl::prelude::*;
use rhdl_fpga::core::dff::DFF;
use crate::simulation::ImageMatrix;
use rhdl::bits::xmul;

use crate::doc::write_svg;

#[kernel]
pub fn mask(_i: ImageMatrix<32, 32>) -> ImageMatrix<32, 32> {
    let threshold : Bits<8> = bits(128);

    let mut res = ImageMatrix::<32, 32>::default();

    for i in 0..31
    {
        for j in 0..32
        {
            if _i.data[i][j] > threshold{
                res.data[i][j] = bits(1)
            }else {
                res.data[i][j] = bits(0);
            }
        }
    };

    return res;
}

#[kernel]
pub fn find_centroid(_i: ImageMatrix<32, 32>) -> ImageMatrix<2, 2> {

    let mut r: ImageMatrix<2, 2> = ImageMatrix::<2,2>{
        data: [
            [bits(0), bits(0)],
            [bits(0), bits(0)]
        ],
    };

    for i in 0..32{
        for j in 0..32{
            r.data[0][0] += if (_i.data[i as usize][j as usize] == 1){bits(i)}else{bits(0)};
            r.data[0][1] += if (_i.data[i as usize][j as usize] == 1){bits(j)}else{bits(0)};

            r.data[1][0] += _i.data[i as usize][j as usize];
            r.data[1][1] += _i.data[i as usize][j as usize];
        }
    }

    return r;
}

#[derive(Synchronous, SynchronousDQ, Debug, Clone)]
pub struct sim{
    pub memory: DFF<Bits<1>>,
}

impl Default for sim{
    fn default() -> Self {
        return sim{memory: DFF::new(Bits::<1>::default())};
    }
}

impl SynchronousIO for sim{
    type I = [ImageMatrix<32, 32>; 2];
    type O = ImageMatrix<2, 2>;
    type Kernel = sim_kernel;
}

#[kernel]
pub fn sim_kernel(_cr : ClockReset, _i : [ImageMatrix<32, 32>; 2], _q : Q) -> (ImageMatrix<2,2>, D){
    let mut r: ImageMatrix<2, 2> = ImageMatrix::<2,2>{
        data: [
            [bits(0), bits(0)],
            [bits(0), bits(0)]
        ],
    };

    let rgb_mask = mask(_i[0]);
    let depth_mask = mask(_i[1]);

    let c_rgb = find_centroid(rgb_mask);
    let c_depth = find_centroid(depth_mask);

    r.data[0][0] = c_rgb.data[0][0] - c_depth.data[0][0];
    r.data[0][1] = c_rgb.data[0][1] - c_depth.data[0][1];

    (r, D::dont_care())
}

#[kernel]
pub fn sim_kernel_comb(_i : [ImageMatrix<32, 32>; 2]) -> (ImageMatrix<2,2>){
    /*let mut r: ImageMatrix<2, 2> = ImageMatrix::<2,2>{
        data: [
            [bits(8), bits(8)],
            [bits(8), bits(24)]
        ],
    };*/

    //let rgb_mask = mask(_i[0]);
    let depth_mask = mask(_i[1]);

    //let c_rgb = find_centroid(rgb_mask);
    let c_depth = find_centroid(depth_mask);

    //r.data[0][0] = c_rgb.data[0][0] - c_depth.data[0][0];
    //r.data[0][1] = c_rgb.data[0][1] - c_depth.data[0][1];

    c_depth
}

pub fn simulate(matrix_rgb: Vec<Vec<u8>>, matrix_depth: Vec<Vec<u8>>) -> Result<(), RHDLError> {
    // aici folosim 480x480
    let image_rgb_bits: ImageMatrix<32, 32> = ImageMatrix::create(matrix_rgb);
    let image_depth_bits: ImageMatrix<32, 32> = ImageMatrix::create(matrix_depth);
    /*let test_matrix: ImageMatrix<4, 4> = ImageMatrix {
        data: core::array::from_fn(|_| core::array::from_fn(|_| Bits::<8>::from(0u128))),
    };*/

    println!("Avem input");

    /*let input = vec![[image_rgb_bits, image_depth_bits]]
        .into_iter()
        .with_reset(1)
        .clock_pos_edge(2);
    let circ = sim::default();

    println!("Ne pregatim de run");
    let vcd = circ.run(input).take(5).collect::<Vcd>();


    println!("Ajungem la printat");
    //let _ = vcd.dump_to_file("ram.vcd")?;
    let _ = write_svg(vcd, "reg.svg");
    */

    let res = sim_kernel_comb([image_rgb_bits, image_depth_bits]);
    let c_x : u128 = res.data[0][0].0;
    let c_y : u128 = res.data[0][1].0;

    let n_x : u128 = res.data[1][0].0;
    let n_y : u128 = res.data[1][1].0;

    println!("Iesire 1 {}, Iesire 2 {}", (c_x as f32 / n_x as f32 * 32.0) as i32, (c_y as f32 / n_y as f32 * 32.0) as i32);

    return Ok(());
}
