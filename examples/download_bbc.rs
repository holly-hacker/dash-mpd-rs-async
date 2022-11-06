// download_bbc.rs
//
// Run with `cargo run --example download_bbc`
//
// Check the extended attributes associated with the downloaded file (on Unix platforms)
// with "xattr -l <output-path>"

use std::process;
use env_logger::Env;
use dash_mpd::fetch::DashDownloader;

#[tokio::main]
async fn main () {
    env_logger::Builder::from_env(Env::default().default_filter_or("info,reqwest=warn")).init();
    let url = "http://rdmedia.bbc.co.uk/dash/ondemand/testcard/1/client_manifest-ctv-events.mpd";
    let ddl = DashDownloader::new(url)
        .worst_quality()
        .verbosity(2);
    #[cfg(target_os = "windows")]
    let ddl = ddl.with_vlc("C:/Program Files/VideoLAN/VLC/vlc.exe");
    match ddl.download().await {
        Ok(path) => println!("Downloaded to {:?}", path),
        Err(e) => {
            eprintln!("Download failed: {:?}", e);
            process::exit(-1);
        },
    }
}
