use std::arch::aarch64::int32x2x2_t;

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, Hyperslab, SliceOrIndex, Result};
use ndarray::{Array2, s};

fn read_hdf5() -> Result<()> 
{
    let file = File::open("/Users/aglowacki/data/2023-1_Twining/img.dat/2xfm_0029.mda.h50")?; 
    let ds_chan_names = file.dataset("/MAPS/XRF_Analyzed/NNLS/Channel_Names")?; 
    let ds_counts = file.dataset("/MAPS/XRF_Analyzed/NNLS/Counts_Per_Sec")?; 
    let chan_names = ds_chan_names.read_1d::<hdf5::types::FixedAscii<256>>().expect("Error reading channel names.");
    let counts_shape = ds_counts.shape();
    for i in 0..counts_shape[0] 
    {
        let slice = s![i, .., ..];
        let data: Array2<f32> = ds_counts.read_slice(slice)?;
        println!("{}", chan_names[i]);
        println!("{data}");
    }
    
    Ok(())
}



fn main() 
{
    read_hdf5();
}
