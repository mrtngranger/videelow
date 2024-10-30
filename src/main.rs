// videeolow/src/main.rs

use clap::{Parser, ValueEnum};
use std::fs::{create_dir_all, remove_file};
use std::path::Path;
use videelow::{download_youtube_audio, download_youtube_video, convert_to_quicktime_compatible_mp4, VideoConversionError};

#[derive(Parser, Debug)]
#[command(author, version, about = "Video downloader and converter")]
/// Struct for parsing command-line arguments.
struct Args {
    #[arg(short, long)]
    /// The YouTube URL of the video to download.
    url: String,

    /// Custom name for the output video and audio files (without extension).
    #[arg(short, long, default_value = "video")]
    name: String,

    /// Output directory where the files will be saved.
    #[arg(short, long, default_value = "Processed")]
    output_dir: String,

    /// Format of the output file: `mp4` for video or `mp3` for audio.
    #[arg(short, long, value_enum, default_value = "mp4")]
    format: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
/// Enum representing output formats for the downloaded file.
enum OutputFormat {
    /// Output as MP3 audio only.
    Mp3,
    /// Output as MP4 video only.
    Mp4,
}

fn main() -> Result<(), VideoConversionError> {
    let args = Args::parse();

    let processed_dir = &args.output_dir;
    let video_path = format!("{}/{}.mp4", processed_dir, args.name);
    let compatible_mp4_path = format!("{}/{}_complete.mp4", processed_dir, args.name);
    let mp3_path = format!("{}/{}.mp3", processed_dir, args.name);

    create_dir_all(processed_dir).map_err(|e| VideoConversionError::CommandError(e.to_string()))?;

    match args.format {
        OutputFormat::Mp4 => {
            // Step 1: Download the video
            download_youtube_video(&args.url, &video_path)?;

            // Step 2: Convert to QuickTime-compatible MP4
            if Path::new(&video_path).exists() {
                convert_to_quicktime_compatible_mp4(&video_path, &compatible_mp4_path)?;

                // Cleanup: Delete the original video file after successful re-encoding
                remove_file(&video_path).map_err(|e| VideoConversionError::CommandError(format!("Failed to delete file: {}", e)))?;
                println!("Original file {} deleted after re-encoding.", video_path);
            } else {
                return Err(VideoConversionError::FileNotFound(video_path));
            }
        }
        OutputFormat::Mp3 => {
            download_youtube_audio(&args.url, &mp3_path)?;
        }
    }

    Ok(())
}