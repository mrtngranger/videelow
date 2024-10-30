// videolow/src/lib.rs

use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
/// Error type for video conversion and downloading errors.
pub enum VideoConversionError {
    /// Error that occurs when a command fails to execute.
    #[error("Failed to execute command: {0}")]
    CommandError(String),
    /// Error indicating that a file was not found.
    #[error("File not found: {0}")]
    FileNotFound(String),
}
/// Downloads a YouTube video as an MP4 file using `yt-dlp`.
///
/// # Arguments
///
/// * `url` - The YouTube URL of the video to download.
/// * `output_path` - The path where the downloaded file will be saved.
///
/// # Errors
///
/// Returns a `VideoConversionError` if the download process fails.
pub fn download_youtube_video(url: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Downloading video from YouTube as MP4...");

    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
        .arg("-o")
        .arg(output_path)
        .arg(url)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| VideoConversionError::CommandError(e.to_string()))?;

    if status.success() {
        println!("Video downloaded successfully: {}", output_path);
        Ok(())
    } else {
        Err(VideoConversionError::CommandError("Command failed".to_string()))
    }
}

/// Converts an MP4 file to a QuickTime-compatible format using `ffmpeg`.
///
/// # Arguments
///
/// * `input_path` - Path to the original MP4 file.
/// * `output_path` - Path to save the QuickTime-compatible MP4 file.
///
/// # Errors
///
/// Returns a `VideoConversionError` if the conversion process fails.
pub fn convert_to_quicktime_compatible_mp4(input_path: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Re-encoding video to QuickTime-compatible MP4...");

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libx264")         // H.264 codec for video
        .arg("-c:a")
        .arg("aac")             // AAC codec for audio
        .arg("-movflags")
        .arg("+faststart")      // Enables streaming compatibility for QuickTime
        .arg(output_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| VideoConversionError::CommandError(e.to_string()))?;

    if status.success() {
        println!("Re-encoding successful: {}", output_path);
        Ok(())
    } else {
        Err(VideoConversionError::CommandError("Re-encoding to QuickTime-compatible format failed".to_string()))
    }
}
/// Downloads a YouTube video as an MP4 file using `yt-dlp`.
///
/// # Arguments
///
/// * `url` - The YouTube URL of the video to download.
/// * `output_path` - The path where the downloaded file will be saved.
///
/// # Errors
///
/// Returns a `VideoConversionError` if the download process fails.
pub fn download_youtube_audio(url: &str, output_path: &str) -> Result<(), VideoConversionError> {
    println!("Downloading audio from YouTube as MP3...");

    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestaudio")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--audio-quality")
        .arg("192K")
        .arg("-o")
        .arg(output_path)
        .arg(url)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| VideoConversionError::CommandError(e.to_string()))?;

    if status.success() {
        println!("Audio downloaded successfully as MP3: {}", output_path);
        Ok(())
    } else {
        Err(VideoConversionError::CommandError("Command failed".to_string()))
    }
}