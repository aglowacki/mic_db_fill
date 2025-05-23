use ndarray::{Array2};
use walkdir::WalkDir;
use std::fs;
use image::{GrayImage};

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
*/

pub fn get_dirs(directory:&str) -> Result<Vec<Option<String>>, std::io::Error>
{
    let mut dir_vec: Vec<Option<String>> = Vec::new();
    
    for entry in fs::read_dir(directory).unwrap() 
    {
        if entry.is_ok()
        {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() 
            {
                dir_vec.push(entry.path().to_str().unwrap().to_string().into());
            }
        }
    }
    Ok(dir_vec)
}

pub fn saerch_hdf5(directory:&str, extentions: &Vec<String>) -> Result<Vec<String>, hdf5::Error> 
{
    let mut hdf5_files: Vec<String> = Vec::new();
    for entry in WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) 
    {
        let f_name = entry.file_name().to_string_lossy();
        for ext in extentions
        {
            if f_name.ends_with(ext)
            {
                hdf5_files.push(entry.path().to_str().unwrap().to_string());
            }
        } 
    }
    Ok(hdf5_files)
}
