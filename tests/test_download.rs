// videelow/tests/test_download.rs

use videelow::{download_youtube_audio, download_youtube_video, VideoConversionError};
use std::fs::remove_file;
use std::path::Path;

#[test]
fn test_download_youtube_audio() -> Result<(), VideoConversionError> {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let output_path = "Processed/test_audio.mp3";

    download_youtube_audio(url, output_path)?;
    assert!(Path::new(output_path).exists());

    remove_file(output_path).ok(); // Cleanup
    Ok(())
}

#[test]
fn test_download_youtube_video() -> Result<(), VideoConversionError> {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let output_path = "Processed/test_video.mp4";

    download_youtube_video(url, output_path)?;
    assert!(Path::new(output_path).exists());

    remove_file(output_path).ok(); // Cleanup
    Ok(())
}