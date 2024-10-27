use std::fs::{create_dir_all};
use std::process::{Command, Stdio};
use clap::Parser;

/// Struct to parse command line arguments using clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the video to download
    #[arg(short, long)]
    url: String,

    /// Custom name for the output video and audio files (without extension)
    #[arg(short, long, default_value = "video")]
    name: String,

    /// Output directory where the files will be saved
    #[arg(short, long, default_value = "Processed")]
    output_dir: String,

    /// Video format to download (e.g., mp4, webm)
    #[arg(short, long, default_value = "mp4")]
    format: String,
}

fn download_youtube_video(url: &str, output_path: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading video from YouTube using yt-dlp...");

    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg(format)  // Specify the format, e.g., mp4
        .arg("-o")
        .arg(output_path)  // Custom output path for the video
        .arg(url)  // YouTube URL
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        println!("Video downloaded successfully: {}", output_path);
    } else {
        eprintln!("Error downloading the video.");
    }

    Ok(())
}

fn convert_to_mp3(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting video to MP3...");

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg(output_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        println!("Conversion successful: {}", output_path);
    } else {
        eprintln!("Error during conversion.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Define paths
    let processed_dir = &args.output_dir;
    let video_path = format!("{}/{}.{}", processed_dir, args.name, args.format);
    let mp3_path = format!("{}/{}.mp3", processed_dir, args.name);

    // Create the output directory if it doesn't exist
    create_dir_all(processed_dir)?;

    // Step 1: Download the video from YouTube using yt-dlp
    if let Err(e) = download_youtube_video(&args.url, &video_path, &args.format) {
        eprintln!("Failed to download video: {}", e);
        return Err(e); // Exit if download fails
    }

    // Step 2: Convert to MP3 using FFmpeg
    if std::path::Path::new(&video_path).exists() {
        convert_to_mp3(&video_path, &mp3_path)?;
    } else {
        eprintln!("Error: Video file not found, cannot convert.");
    }

    Ok(())
}