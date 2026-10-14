//! Save a short Dailymotion sample using the Rust extractor and FFmpeg's HLS reader.
//!
//! Requires yt-dlp and ffmpeg on PATH.
//! Usage: cargo run --example dailymotion -- <URL> [output.mp4] [seconds]

use std::path::PathBuf;

use tokio::process::Command;
use yt_dlp::Downloader;
use yt_dlp::client::deps::Libraries;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let url = args.next().ok_or("Usage: dailymotion <URL> [output.mp4] [seconds]")?;
    let output = PathBuf::from(args.next().unwrap_or_else(|| "dailymotion-sample.mp4".into()));
    let seconds: u32 = args.next().unwrap_or_else(|| "30".into()).parse()?;
    if seconds == 0 {
        return Err("seconds must be greater than zero".into());
    }

    let libraries = Libraries::new(PathBuf::from("yt-dlp"), PathBuf::from("ffmpeg"));
    let downloader = Downloader::builder(libraries, ".")
        .with_args(vec!["--no-playlist".into()])
        .build()
        .await?;
    let video = downloader.fetch_video_infos(&url).await?;
    println!("Title: {}", video.title);

    // Dailymotion exposes muxed HLS formats. Choose the smallest available rendition
    // for this trial and let FFmpeg assemble the segments into an MP4.
    let format = video
        .formats
        .iter()
        .filter(|format| {
            format.download_info.url.is_some()
                && format
                    .codec_info
                    .video_codec
                    .as_deref()
                    .is_some_and(|codec| codec != "none")
                && format
                    .codec_info
                    .audio_codec
                    .as_deref()
                    .is_some_and(|codec| codec != "none")
        })
        .min_by_key(|format| format.video_resolution.height.unwrap_or(u32::MAX))
        .ok_or("No combined audio/video stream found")?;
    let stream_url = format.download_info.url.as_deref().ok_or("Missing stream URL")?;
    println!("Format: {}; saving {seconds} seconds", format.format_id);

    let status = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error", "-nostdin", "-n"])
        .arg("-user_agent")
        .arg(&format.download_info.http_headers.user_agent)
        .arg("-i")
        .arg(stream_url)
        .args(["-t", &seconds.to_string(), "-c", "copy", "-movflags", "+faststart"])
        .arg(&output)
        .status()
        .await?;
    if !status.success() {
        return Err("FFmpeg could not save the sample".into());
    }
    println!("Saved: {}", output.display());
    Ok(())
}
