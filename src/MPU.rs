/*
    MAIN PROCCESING UNIT
    CIRCUITUL COMPLET

use rhdl::bits;
use rhdl::prelude::*;
use rhdl_fpga::core::dff::DFF;
use rhdl_fpga::core::ram;
use rhdl_fpga::core::ram::synchronous::SyncBRAM;


#[derive(Synchronous, SynchronousDQ, Debug, Clone)]
pub struct  MPU{
    pub ram : SyncBRAM<Bits<U8>, U128>,
}

impl SynchronousIO for MPU {
    type I = Bits<U8>;
    type O = Bits<U8>;
    type Kernel = mpu_kernel;
}

#[kernel]
pub fn mpu_kernel(_cr : ClockReset, _i : Bits<U8>, _q: Q) -> (Bits<U8>, D)
{

    (bits(0), D{ram: SyncBRAM::default()})
}
*/