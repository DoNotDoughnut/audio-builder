fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    frc_audio_builder::compile("music", "audio.bin")?;
    println!("Finished in {}ms.", start.elapsed().as_millis());
    Ok(())
}