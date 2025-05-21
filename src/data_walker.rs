use ndarray::{Array2};
use walkdir::WalkDir;
use image::{GrayImage};


pub mod dataset;

pub struct Config
{
    pub recursive: bool,
    pub export_counts_png: bool,
    pub directory: String,
}

impl Config
{
    pub fn new() -> Config
    {
        Config
        {
            recursive: false,
            export_counts_png: false,
            directory: String::new(),
        }
    }
}
/*
fn array_to_image(arr: Array2<f32>) -> GrayImage 
{
    assert!(arr.is_standard_layout());

    let (height, width) = arr.dim();
    let raw_f32 = arr.into_raw_vec();
    let max_val  = raw_f32.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_val = raw_f32.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let f_range = max_val - min_val;
    let raw_1d = raw_f32.iter().map(|&x| (255.0 * (x - min_val) / f_range) as u8).collect::<Vec<u8>>();
    GrayImage::from_raw(width as u32, height as u32, raw_1d).expect("ERROR: container should have the right size for the image dimensions")
}
*

pub fn saerch_hdf5(config:Config) -> Result<(), hdf5::Error> 
{
    for entry in WalkDir::new(config.directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        //let sec = entry.metadata()?.modified()?;

        //if f_name.ends_with(".h50") && sec.elapsed()?.as_secs() < 86400 
        if f_name.ends_with(".h50")
        {
            let mut dataset = dataset::XrfDataset::new();
            dataset.load_from_hdf5(entry.path().to_str().unwrap()).unwrap();
        }
        
        //else if f_name.ends_with(".h51")
        //{
        //    let _ = read_hdf5(entry.path().to_str().unwrap());
        //}
         
    }

    Ok(())
}
*/