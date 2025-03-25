use std::fs;
use std::path::{Path, PathBuf};

fn main() 
{
    let custom_out_dir = "bin";
    if !Path::new(custom_out_dir).exists() 
    {
        fs::create_dir_all(custom_out_dir).expect("Failed to create bin directory");
    }
    let bin_name = "lema";     
    let target_bin = PathBuf::from(format!("target/release/{}", bin_name));
    let dest_bin = PathBuf::from(format!("{}/{}", custom_out_dir, bin_name));
    if target_bin.exists() 
    {
        fs::copy(&target_bin, &dest_bin).expect("Failed to copy binary to bin directory");
        println!("Binary copied to {}/{}", custom_out_dir, bin_name);
    }
}
