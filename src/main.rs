use std::fs;
use std::path::PathBuf;

use clap::Parser;
use image::imageops::FilterType;
use wax::Glob;

#[derive(Parser, Debug)]
struct Parameter {
    #[arg(long, default_value_t = 1280)]
    max_width: u32,
    #[arg(long, default_value_t = 720)]
    max_height: u32,
    #[arg(short, long)]
    input_path: String,
    #[arg(short, long)]
    output_dir: Option<String>,
}

fn resize_image(input_path: &str, output_path: &str, max_width: u32, max_height: u32) {
    let input = image::open(input_path).unwrap();
    let output = input.resize(max_width, max_height, FilterType::Lanczos3);
    output.save(output_path).unwrap();
}

fn main() {
    let params = Parameter::parse();
    if let Some(output_dir) = &params.output_dir {
        fs::create_dir_all(output_dir).unwrap();
    }

    let glob = Glob::new(&params.input_path).unwrap();
    for item in glob.walk(".") {
        let entry = item.unwrap();
        let path = entry.path();
        let input_path = path.to_str().unwrap();
        let output_path = params
            .output_dir
            .as_ref()
            .map_or(input_path.to_string(), |value| {
                let input_file_name = path.file_name().unwrap().to_str().unwrap();
                let mut path_buf = PathBuf::new();
                path_buf.push(value);
                path_buf
                    .join(input_file_name)
                    .into_os_string()
                    .into_string()
                    .unwrap()
            });

        resize_image(
            input_path,
            &output_path,
            params.max_width,
            params.max_height,
        )
    }
}
