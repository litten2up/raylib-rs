use raylib::prelude::*;
use rres_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialization
    //--------------------------------------------------------------------------------------
    const SCREEN_WIDTH: i32 = 384;
    const SCREEN_HEIGHT: i32 = 512;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rres example - rres load image")
        .build();

    // Load central directory from .rres file (if available)
    let dir = CentralDir::new("resources.rres").expect("File not found");

    // Get resource id from original fileName (stored in centra directory)
    let id = dir.get_resource_id("resources/images/fudesumi.png");

    // Setup password to load encrypted data (if required)
    set_cipher_password("password12345");

    // Load resource chunk from file providing the id
    let mut chunk = ResourceChunk::new("resources.rres", id).expect("Resource chunk not found");

    // Decompress/decipher resource data (if required)
    rl.unpack_resource_chunk(&mut chunk).expect("Unpack error");

    let image = rl
        .load_image_from_resource(chunk)
        .expect("Error loading image");

    let texture = rl.load_texture_from_image(&thread, &image)?;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }

    Ok(())
}
