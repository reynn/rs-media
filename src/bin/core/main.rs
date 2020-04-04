use clap::{crate_version, Clap};
use mmmedia_core::metadata::ffprobe;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = "Nic Patterson")]
struct AppOpts {
    #[clap(short = "c", long = "config", default_value = "default.conf")]
    config: String,
    #[clap(short = "f", long = "file", multiple = true)]
    files: Vec<String>,
    #[clap(short = "d", long = "dir", multiple = true)]
    directories: Vec<String>,
    #[clap(long = "ffprobe-binary", multiple = true, default = "ffprobe")]
    ffprobe_binary: String,
}

fn main() -> std::io::Result<()> {
    let opts: AppOpts = AppOpts::parse();

    println!("opts: {:#?}", opts);

    let file_metadata = ffprobe::MetaData::new(
        &opts
            .files
            .get(0)
            .expect("Must provide at least one file to gather data on")[..],
        "ffprobe",
    );

    println!("file_metadata: {:#?}", file_metadata);

    Ok(())
}
