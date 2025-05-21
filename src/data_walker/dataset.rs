//#[cfg(feature = "blosc")]
//use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, Result};
use ndarray::{Array2, s};

enum AnalysisType
{
    NNLS,
    FITTED,
}

struct AnalyzedCounts
{
    analysis_type :AnalysisType,
    channel_names: Vec<String>,
    counts_data: Vec<Array2<f32>>,
}

impl AnalyzedCounts
{
    pub fn new(atype: AnalysisType) -> AnalyzedCounts
    {
        AnalyzedCounts
        {
            analysis_type : atype,
            channel_names: Vec::new(),
            counts_data: Vec::new(),
        }
    }
}

pub struct XrfDataset
{
    //filename: String,
    path: String,
    analyzed_data: Vec<AnalyzedCounts>,
}

impl XrfDataset
{
    pub fn new() -> XrfDataset
    {
        XrfDataset
        {
            //filename : String::new(),
            path: String::new(),
            analyzed_data: Vec::new(),
        }
    }
    pub fn load_from_hdf5(&mut self, file_path: &str) -> Result<()>
    {
        self.path = file_path.to_string();
        println!("loading {}", file_path);
        let file = File::open(file_path)?; 
        // v10
        let ds_chan_names = file.dataset("/MAPS/XRF_Analyzed/NNLS/Channel_Names")?; 
        let ds_counts = file.dataset("/MAPS/XRF_Analyzed/NNLS/Counts_Per_Sec")?; 
        if ds_chan_names.id() > 0 && ds_counts.id() > 0
        {
            let mut analyzed_counts = AnalyzedCounts::new(AnalysisType::NNLS);
            analyzed_counts.channel_names = ds_chan_names.read_1d::<hdf5::types::FixedAscii<256>>().expect("Error reading channel names.").iter().map(|x| x.to_string()).collect();
            let counts_shape = ds_counts.shape();
            for i in 0..counts_shape[0] 
            {
                let slice = s![i, .., ..];
                let data: Array2<f32> = ds_counts.read_slice(slice)?;
                analyzed_counts.counts_data.push(data);
                //println!("{}", chan_names[i]);
                //let img = array_to_image(data);
                //img.save(format!("/Users/aglowacki/data/tmp/{}.png", chan_names[i])).unwrap_or();
            }
            self.analyzed_data.push(analyzed_counts);
        }
        Ok(())
    }
}

