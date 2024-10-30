use std::fs::{create_dir_all, remove_file};
use std::path::Path;
use std::process::{Command, Stdio};
use clap::Parser;
use thiserror::Error;

/// Struct to parse command line arguments using clap
#[derive(Parser, Debug)]
#[command(author, version, about = "Video downloader and converter")]
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
}

/// Custom error type for improved error handling
#[derive(Error, Debug)]
enum VideoConversionError {
    #[error("Failed to execute command: {0}")]
    CommandError(String),

    #[error("File not found: {0}")]
    FileNotFound(String),
}

/// Helper function to run external commands
fn run_command(command: &mut Command) -> Result<(), VideoConversionError> {
    let status = command.status().map_err(|e| VideoConversionError::CommandError(e.to_string()))?;
    if status.success() {
        Ok(())
    } else {
        Err(VideoConversionError::CommandError("Command failed".to_string()))
    }
}

/// Function to download YouTube videos with yt-dlp
fn download_youtube_video(url: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Downloading video from YouTube...");

    run_command(
        Command::new("yt-dlp")
            .arg("-f")
            .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best") // Use MP4 format for compatibility
            .arg("-o")
            .arg(output_path)
            .arg(url)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit()),
    )?;

    println!("Video downloaded successfully: {}", output_path);
    Ok(())
}

/// Function to convert MP4 to a QuickTime-compatible format
fn convert_to_quicktime_compatible_mp4(input_path: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Re-encoding video to QuickTime-compatible MP4...");

    run_command(
        Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-c:v")
            .arg("libx264") // H.264 codec for video
            .arg("-c:a")
            .arg("aac")     // AAC codec for audio
            .arg("-movflags")
            .arg("+faststart") // For streaming compatibility
            .arg(output_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit()),
    )?;

    println!("Re-encoding successful: {}", output_path);
    Ok(())
}

/// Function to convert MP4 to MP3 audio-only
fn convert_to_mp3(input_path: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Converting video to MP3...");

    run_command(
        Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-vn")   // Exclude video
            .arg("-q:a")
            .arg("2")     // Set audio quality
            .arg(output_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit()),
    )?;

    println!("Conversion to MP3 successful: {}", output_path);
    Ok(())
}

fn main() -> Result<(), VideoConversionError> {
    let args = Args::parse();

    // Define paths
    let processed_dir = &args.output_dir;
    let video_path = format!("{}/{}.mp4", processed_dir, args.name);
    let compatible_mp4_path = format!("{}/{}_complete.mp4", processed_dir, args.name);
    let mp3_path = format!("{}/{}.mp3", processed_dir, args.name);

    // Ensure the output directory exists
    create_dir_all(processed_dir).map_err(|e| VideoConversionError::CommandError(e.to_string()))?;

    // Step 1: Download video
    download_youtube_video(&args.url, &video_path)?;

    // Step 2: Convert to QuickTime-compatible MP4 if necessary
    if Path::new(&video_path).exists() {
        convert_to_quicktime_compatible_mp4(&video_path, &compatible_mp4_path)?;

        // Cleanup: Delete original video file after successful re-encoding
        remove_file(&video_path).map_err(|e| VideoConversionError::CommandError(format!("Failed to delete file: {}", e)))?;
        println!("Original file {} deleted after re-encoding.", video_path);
    } else {
        return Err(VideoConversionError::FileNotFound(video_path));
    }

    // Step 3: Convert compatible MP4 to MP3
    if Path::new(&compatible_mp4_path).exists() {
        convert_to_mp3(&compatible_mp4_path, &mp3_path)?;
    } else {
        return Err(VideoConversionError::FileNotFound(compatible_mp4_path));
    }

    Ok(())
}