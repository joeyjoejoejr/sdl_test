use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    surface::Surface,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;
const BUFF_SIZE: usize = (WIDTH * HEIGHT * 4) as usize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Show logs from wgpu
    env_logger::init();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Raw Window Handle Example", WIDTH * 4, HEIGHT * 4)
        .position_centered()
        .borderless()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().ok_or("Couldn't find gl driver")?)
        .build()?;
    let texture_creator = canvas.texture_creator();
    let surface = Surface::new(WIDTH, HEIGHT, sdl2::pixels::PixelFormatEnum::RGBA8888)?;
    let mut texture = surface.as_texture(&texture_creator)?;
    let mut buffer: [u8; BUFF_SIZE] = [0; BUFF_SIZE];

    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new();

    'game_loop: loop {
        let events = event_pump.poll_iter();
        for event in events {
            match event {
                // Close events
                Event::Quit { .. } => break 'game_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,

                // // Resize the window
                // Event::Window {
                //     window_id,
                //     win_event: WindowEvent::SizeChanged(width, height),
                //     ..
                // } if window_id == window.id() => pixels.resize_surface(width as u32, height as u32),
                _ => (),
            }
        }
        // Update internal state
        world.update();
        // Draw the current frame
        world.draw(&mut buffer);
        texture.update(None, &buffer, (4 * WIDTH) as usize)?;
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
