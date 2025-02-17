#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, Hyperslab, SliceOrIndex, Result};
use ndarray::{Array2, s};

struct analyzed_counts
{
    analysis_type :String,
    channel_names: Vec<String>,
    counts_data: Vec<Array2<f32>>,
}

pub struct XRF_Dataset
{
    filename: String,
    path: String,
    analyzed_data: Vec<analyzed_counts>,
}

impl XRF_Dataset
{
    pub fn new() -> XRF_Dataset
    {
        XRF_Dataset
        {
            filename : String::new(),
            path: String::new(),
            analyzed_data: Vec::new(),
        }
    }
    pub fn load_from_hdf5(mut self, filePath: &str) -> Result<()>
    {
        self.path = filePath.to_owned();
        println!("loading {}", filePath);
        let file = File::open(filePath)?; 
        // v10
        let ds_chan_names = file.dataset("/MAPS/XRF_Analyzed/NNLS/Channel_Names")?; 
        let ds_counts = file.dataset("/MAPS/XRF_Analyzed/NNLS/Counts_Per_Sec")?; 
        if ds_chan_names.id() > 0 && ds_counts.id() > 0
        {
            let chan_names = ds_chan_names.read_1d::<hdf5::types::FixedAscii<256>>().expect("Error reading channel names.");
            let counts_shape = ds_counts.shape();
            for i in 0..counts_shape[0] 
            {
                let slice = s![i, .., ..];
                let data: Array2<f32> = ds_counts.read_slice(slice)?;
                println!("{}", chan_names[i]);
                //let img = array_to_image(data);
                //img.save(format!("/Users/aglowacki/data/tmp/{}.png", chan_names[i])).unwrap();
            }
        }
        Ok(())
    }
}

