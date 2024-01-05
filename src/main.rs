use base64::Engine;
use base64::engine::general_purpose;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AromaGuard Unpacker");
    println!(" > This was the dumbest garbage that I have ever seen. Next time, make it worth my time.");
    println!(" > Developed with love by AdolfCracks <3.");
    println!();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} file.jar", args[0]);
        println!("You can drag the file you wish to unpack directly into the executable!");
        return Ok(());
    }

    let path = &args[1];
    let content = std::fs::read(path)?;

    println!(" > Executing insane decoding logic...");
    let decoded = general_purpose::STANDARD
        .decode(content)?;

    let new_path = format!("{}-unpacked.jar", path);

    std::fs::write(&new_path, decoded)?;

    println!(" > Wrote output to {}", new_path);

    Ok(())
}