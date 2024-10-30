use std::fs::{create_dir_all, remove_file};
use std::path::Path;
use std::process::{Command, Stdio};
use clap::{Parser, ValueEnum};
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

    /// Output format (mp3 or mp4)
    #[arg(short, long, value_enum, default_value = "mp4")]
    format: OutputFormat,
}

/// Enum to define allowed output formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Mp3,
    Mp4,
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

/// Function to download YouTube video as MP4 with yt-dlp
fn download_youtube_video(url: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Downloading video from YouTube as MP4...");

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

/// Function to download YouTube audio directly as MP3 with yt-dlp
fn download_youtube_audio(url: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Downloading audio from YouTube as MP3...");

    run_command(
        Command::new("yt-dlp")
            .arg("-f")
            .arg("bestaudio")             // Choose the best audio quality available
            .arg("--extract-audio")        // Extract audio only
            .arg("--audio-format")
            .arg("mp3")                    // Convert audio to MP3
            .arg("--audio-quality")
            .arg("192K")                   // Set a standard bitrate for quality
            .arg("-o")
            .arg(output_path)
            .arg(url)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit()),
    )?;

    println!("Audio downloaded successfully as MP3: {}", output_path);
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

fn main() -> Result<(), VideoConversionError> {
    let args = Args::parse();

    // Define paths
    let processed_dir = &args.output_dir;
    let video_path = format!("{}/{}.mp4", processed_dir, args.name);
    let compatible_mp4_path = format!("{}/{}_complete.mp4", processed_dir, args.name);
    let mp3_path = format!("{}/{}.mp3", processed_dir, args.name);

    // Ensure the output directory exists
    create_dir_all(processed_dir).map_err(|e| VideoConversionError::CommandError(e.to_string()))?;

    match args.format {
        OutputFormat::Mp4 => {
            // Download and process MP4
            download_youtube_video(&args.url, &video_path)?;

            if Path::new(&video_path).exists() {
                convert_to_quicktime_compatible_mp4(&video_path, &compatible_mp4_path)?;

                // Cleanup: Delete original video file after successful re-encoding
                remove_file(&video_path).map_err(|e| VideoConversionError::CommandError(format!("Failed to delete file: {}", e)))?;
                println!("Original file {} deleted after re-encoding.", video_path);
            } else {
                return Err(VideoConversionError::FileNotFound(video_path));
            }
        }
        OutputFormat::Mp3 => {
            // Download and process MP3 directly
            download_youtube_audio(&args.url, &mp3_path)?;
        }
    }

    Ok(())
}