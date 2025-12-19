//! ## Calculul centrului de greutate și al orientării folosind momente de ordinul 2
//!
//! Acest modul implementează calculul centrului de greutate (centroidului) și al
//! orientării principale pentru o imagine discretă, folosind momente spațiale
//! de ordin 0, 1 și 2. Abordarea este potrivită pentru implementare hardware (FPGA),
//! deoarece toate sumele pot fi calculate într-o singură trecere, fara a implementa o arhitectura
//! cu core-uri secventiale si stocarea informatiei imaginii.
//!
//! **Pentru sursa am folosit [Wikipedia](https://en.wikipedia.org/wiki/Image_moment)**
//!
//! Imaginea este considerată ca un set de pixeli cu coordonate `(x, y)` și o greutate
//! `m(x, y)`. În cazul uzual, `m` este o mască binară (0/1), de asemenea precalculata,
//! dar formularea rămâne validă și pentru valori ponderate (ex. intensități 0–255).
//!
//! Un circuit combinational de prelucrare masca se gaseste in mask.rs, dar nu e folosit deoarece
//! e mult mai usor pentru simulare sa nu il adaugam. El ar veni teoretic in fata unui circuit care
//! imparte imaginea in stream-ul nostru de date, pe care il definim mai jos
//!
//!
//! ### 1. Momente brute
//!
//! Pentru toți pixelii imaginii se acumulează următoarele sume:
//!
//! ```text
//! S    = Σ m(x, y)
//! Sx   = Σ x · m(x, y)
//! Sy   = Σ y · m(x, y)
//! Sxx  = Σ x² · m(x, y)
//! Syy  = Σ y² · m(x, y)
//! Sxy  = Σ x·y · m(x, y)
//! ```
//!
//! ### 2. Centroid (centrul de greutate)
//!
//! După finalizarea acumulării, coordonatele centrului de greutate sunt:
//!
//! ```text
//! cx = Sx / S
//! cy = Sy / S
//! ```
//!
//! Impartirile se realizeaza o singura data si de aceea le realizam in afara circuitului sincron.
//!
//! ---
//!
//! ### 3. Momente centrale de ordinul 2
//!
//! Pentru a determina orientarea obiectului, se folosesc momente centrale,
//! care elimină dependența de poziția absolută a centroidului:
//!
//! ```text
//! Mxx = Σ (x − cx)² · m(x, y) = Sxx − (Sx² / S)
//! Myy = Σ (y − cy)² · m(x, y) = Syy − (Sy² / S)
//! Mxy = Σ (x − cx)(y − cy) · m(x, y) = Sxy − (Sx·Sy / S)
//! ```
//!
//!
//! ### 4. Vectorul de orientare (axa principală)
//!
//! O formă eficientă de a obține orientarea principală este utilizarea
//! vectorului:
//!
//! ```text
//! v = [ 2·Mxy , Mxx − Myy ]
//! ```
//!
//! Acest vector este proporțional cu direcția axei principale a obiectului.
//! Dacă `Mxx ≈ Myy` și `Mxy ≈ 0`, orientarea este nedeterminată (simetrie).
//!
//!
//! ### 5. Rotația relativă între două imagini
//!
//! Pentru cele doua imagini RGB si Depth, se calculează:
//!
//! ```text
//! vRGB = [ 2·Mxy_RGB , Mxx_RGB − Myy_RGB ]
//! vDepth = [ 2·Mxy_Depth , Mxx_Depth − Myy_Depth ]
//! ```
//!
//! Se normalizează vectorii:
//!
//! ```text
//! uRGB = vRGB / ||vRGB||
//! uDepth = vDepth / ||vDepth ||
//! ```
//!
//! Apoi se obțin termenii de rotație relativă:
//!
//! ```text
//! cosθ = dot(uRGB, uDepth)
//! sinθ = cross(uRGB, uDepth)
//! ```
//!
//! Matricea de rotație care aliniază imaginea RGB la imaginea Depth este:
//!
//! ```text
//! R = [  cosθ  -sinθ
//!        sinθ   cosθ ]
//! ```
//!
//!
//! ### 6. Translația după rotație
//!
//! După aplicarea rotației, translația necesară este:
//!
//! ```text
//! t = [ cx_Depth , cy_Depth ] − R · [ cx_RGB , cy_RGB ]
//! ```
//!
//! Această translație, împreună cu matricea de rotație, definește complet
//! transformarea 2D dintre cele două imagini.


/*
La cat am stat sa fac tema, imi permit sa las aici un mic playlist de spotify:
    - Korn - Twisted Transistor : https://open.spotify.com/track/05NpeTQWnzXS1d8ZqL4YFZ?si=4ab58e918e4c4e2f
    - Korn - Coming Undone : https://open.spotify.com/track/3o7TMr6RmIusYH7Kkg7ujR?si=e7c4c740714d411b
    - MCR - House of wolves : https://open.spotify.com/track/7j2Bmzpnf6RwEWEQ2sv8Ho?si=e91b3065c0714889
    - MCR - Sharpest Lives : https://open.spotify.com/track/2kMjk14RmYyYhhSbipoa9U?si=52e86f3dd52c4018
    - MCR - Disenchanted : https://open.spotify.com/track/6T7MAQCekVb3UnCykjX3BP?si=32ecb385a0c14bf8
    - The Pretty Reckless - Going to Hell ( Banger album! )
        : https://open.spotify.com/album/5cjJRrzeVRE79YXiTSCbVf?si=h18pEK-SR0KaG_mfzjeOCg

Si un counter de cat a luat, heheheha
Ore de lucru:
16
*/


use rhdl::bits;
use rhdl::bits::mul;
use rhdl::bits::xmul;
use rhdl::prelude::*;
use rhdl_fpga::core::dff::DFF;
use std::num::*;

use crate::doc::write_svg;
use crate::mask::sim_kernel_comb;

/*
   Cum facem translatia cu astea?
   Am simplificat problema, plecand de la imagini grele am ajuns sa facem 2 tinte sintetice. Ideea
   de baza era sa simplificam transformarea imaginilor intr-o translatie si o rotatie 2D. De aici,
   scoate coordonatele centrului de greutate ale culorii albe folosind sum_x/sum_mask pentru
   coordonata in X si sum_y/sum_mask pt coordonata in Y.
*/

#[doc="Structura care se ocupa de starea interna a masinii. Avem in ea toate sumele necesare aflarii\
matricilor de transformare. La final, le si scoate drept output pentru a calcula aceste matrici. Le\
calculam in afara matricii deaorece sunt foarte grele matematic si e mai usor asa. Se poate face si\
cu un circuit combinational, dar RHDL nu ofera o metoda usoara de a face impartiri si de a reprezenta\
numere cu virgula"]
#[derive(Digital, Debug, Clone, PartialEq, Eq, Copy)]
pub struct image_state {
    // set date pentru imagine, ca sa calculam mai usor tot
    pub sum_mask: Bits<10>,
    pub sum_x: Bits<16>,
    pub sum_y: Bits<16>,
    pub sum_x_pow_2: Bits<24>,
    pub sum_y_pow_2: Bits<24>,
    pub sum_x_y: Bits<24>,
}


impl Default for image_state {
    fn default() -> Self {
        Self {
            sum_mask: Bits::<10>::from(0),
            sum_x: Bits::<16>::from(0),
            sum_y: Bits::<16>::from(0),
            sum_x_pow_2: Bits::<24>::from(0),
            sum_y_pow_2: Bits::<24>::from(0),
            sum_x_y: Bits::<24>::from(0),
        }
    }
}

#[doc="Circuitul sincron, care extrage valorile din imaginile date"]
#[derive(Synchronous, SynchronousDQ, Debug, Clone, PartialEq)]
pub struct circ {
    // aparent == e implementat pt tuple-uri <=12 elemente
    // asa ca trebuie sa facem o structura de state
    // YEY :D
    pub rgb_state: DFF<image_state>,
    pub depth_state: DFF<image_state>,
}
impl Default for circ {
    fn default() -> Self {
        return circ {
            rgb_state: DFF::new(image_state::default()),
            depth_state: DFF::new(image_state::default()),
        };
    }
}

impl SynchronousIO for circ {
    // avem i.0 pixelul imaginii rgb, i.1 pixelul imaginii depth, i.2 coordonata liniei si i.3 coordonata coloanei
    type I = (Bits<8>, Bits<8>, Bits<8>, Bits<8>);

    // output-ul o sa aiba valorile de test, sa vedem clar ce iese din el
    // TODO: un circuit combinational sa le dea direct afara din circuit
    type O = (image_state, image_state);
    type Kernel = circ_kernel;
}
#[doc="Kernelul circuitului sincron\
\
\\
\'_i' - 4 biti, in ordinea asta: pixel_rgb, pixel_depth, coordonata x, coordonata y"]
#[kernel]
pub fn circ_kernel(
    _cr: ClockReset,
    _i: (Bits<8>, Bits<8>, Bits<8>, Bits<8>),
    _q: Q,
) -> ((image_state, image_state), D) {
    let mut rgb_state: image_state = _q.rgb_state;
    let mut depth_state: image_state = _q.depth_state;

    let pixel_rgb = _i.0;
    let pixel_depth = _i.1;
    let i = _i.2;
    let j = _i.3;

    rgb_state.sum_mask = rgb_state.sum_mask + bits(pixel_rgb.0);
    rgb_state.sum_x = rgb_state.sum_x + bits(pixel_rgb.0) * bits(i.0);
    rgb_state.sum_y = rgb_state.sum_y + bits(pixel_rgb.0) * bits(j.0);
    rgb_state.sum_x_pow_2 = rgb_state.sum_x_pow_2 + bits(pixel_rgb.0) * bits(i.0) * bits(i.0);
    rgb_state.sum_y_pow_2 = rgb_state.sum_y_pow_2 + bits(pixel_rgb.0) * bits(j.0) * bits(j.0);
    rgb_state.sum_x_y = rgb_state.sum_x_y + bits(pixel_rgb.0) * bits(i.0) * bits(j.0);

    depth_state.sum_mask = depth_state.sum_mask + bits(pixel_depth.0);
    depth_state.sum_x = depth_state.sum_x + bits(pixel_depth.0) * bits(i.0);
    depth_state.sum_y = depth_state.sum_y + bits(pixel_depth.0) * bits(j.0);
    depth_state.sum_x_pow_2 = depth_state.sum_x_pow_2 + bits(pixel_depth.0) * bits(i.0) * bits(i.0);
    depth_state.sum_y_pow_2 = depth_state.sum_y_pow_2 + bits(pixel_depth.0) * bits(j.0) * bits(j.0);
    depth_state.sum_x_y = depth_state.sum_x_y + bits(pixel_depth.0) * bits(j.0) * bits(i.0);

    (
        (rgb_state, depth_state),
        D {
            rgb_state: rgb_state,
            depth_state: depth_state,
        },
    )
}


pub fn simulate(matrix_rgb: Vec<Vec<u8>>, matrix_depth: Vec<Vec<u8>>, graphs : bool, clocks : u64) -> Result<(), RHDLError> {
    // aplatizam matricile si le facem vector de intrare
    let mut input: Vec<(Bits<8>, Bits<8>, Bits<8>, Bits<8>)> = Vec::new();
    for i in 0..matrix_rgb.len() {
        for j in 0..matrix_depth.len() {
            input.push((
                Bits::<8>::from(matrix_rgb[i][j] as u128),
                Bits::<8>::from(matrix_depth[i][j] as u128),
                Bits::<8>::from(i as u128),
                Bits::<8>::from(j as u128),
            ));
        }
    }

    println!("Ne pregatim de run");

    //tip de run
    let select = false;
    if graphs {
        let input = input.into_iter().with_reset(1).clock_pos_edge(clocks);
        let circ = circ::default();

        // print wave-uri
        let vcd = circ.run(input).collect::<Vcd>();
        let _ = write_svg(vcd, "reg.svg");
    } else {
        let input = input.into_iter().with_reset(1).clock_pos_edge(1024);
        let circ = circ::default();

        //Print valori finale
        let out: Vec<_> = circ.run(input).collect();
        let res = out.last().unwrap();
        // println!("{:?}", res);
        println!(
            "t : [ {} {} ]\n",
            (res.value.2.0.sum_x.0 / res.value.2.0.sum_mask.0) as i128 - (res.value.2.1.sum_x.0 / res.value.2.1.sum_mask.0) as i128,
            (res.value.2.0.sum_y.0 / res.value.2.0.sum_mask.0) as i128 - (res.value.2.1.sum_y.0 / res.value.2.1.sum_mask.0) as i128
        );
        let M_xx_rgb = res.value.2.0.sum_x_pow_2.0 as i128 - (res.value.2.0.sum_x.0 * res.value.2.0.sum_x.0 / res.value.2.0.sum_mask.0) as i128;
        let M_yy_rgb = res.value.2.0.sum_y_pow_2.0 as i128 - (res.value.2.0.sum_y.0 * res.value.2.0.sum_y.0 / res.value.2.0.sum_mask.0) as i128;
        let M_xy_rgb = res.value.2.0.sum_x_y.0 as i128 - (res.value.2.0.sum_x.0 * res.value.2.0.sum_y.0 / res.value.2.0.sum_mask.0) as i128;
        println!(
            "M_xx_rgb : {} \nM_yy_rgb : {} \nM_xy_rgb : {}",
            M_xx_rgb,
            M_yy_rgb,
            M_xy_rgb
        );

        println!(
            "Matrice de rotatie relativa RGB : [ {}  {} ]\n",
            2 * M_xy_rgb,
            M_xx_rgb - M_yy_rgb
        );

        let M_xx_depth = res.value.2.1.sum_x_pow_2.0 as i128 - (res.value.2.1.sum_x.0 * res.value.2.1.sum_x.0 / res.value.2.1.sum_mask.0) as i128;
        let M_yy_depth = res.value.2.1.sum_y_pow_2.0 as i128 - (res.value.2.1.sum_y.0 * res.value.2.1.sum_y.0 / res.value.2.1.sum_mask.0) as i128;
        let M_xy_depth = res.value.2.1.sum_x_y.0 as i128 - (res.value.2.1.sum_x.0 * res.value.2.1.sum_y.0 / res.value.2.1.sum_mask.0) as i128;
        println!(
            "M_xx_depth : {} \nM_yy_depth : {} \nM_xy_depth : {}",
            M_xx_depth,
            M_yy_depth,
            M_xy_depth
        );

        println!(
            "Matrice de rotatie relativa depth : [ {}  {} ]\n",
            2 * M_xy_depth,
            M_xx_depth - M_yy_depth
        );

        let norm_rgb : f64 = (((2 * M_xy_rgb) * (2 * M_xy_rgb) + (M_xx_rgb - M_yy_rgb) * (M_xx_rgb - M_yy_rgb)) as f64).sqrt();
        let norm_depth : f64 = (((2 * M_xy_depth) * (2 * M_xy_depth) + (M_xx_depth - M_yy_depth) * (M_xx_depth - M_yy_depth)) as f64).sqrt();

        let u_rgb = [(2 * M_xy_rgb) as f64 / norm_rgb, (M_xx_rgb - M_yy_rgb) as f64 / norm_rgb];
        let u_depth = [(2 * M_xy_depth) as f64 / norm_depth, (M_xx_depth - M_yy_depth) as f64 / norm_depth];

        let c = u_rgb[0] * u_depth[0] + u_rgb[1] * u_depth[1];
        let s = u_rgb[0] * u_depth[1] + u_rgb[1] * u_depth[0];

        println!("R: \n[{} {}\n {} {}]",
            c,
            -s,
            s,
            c
        );

    }
    return Ok(());
}
