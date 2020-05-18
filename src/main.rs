// ==================================== Imports ==================================================
use sdl2;

// ==================================== Types ====================================================

// ==================================== Functions ================================================

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;



    Ok(())
}
