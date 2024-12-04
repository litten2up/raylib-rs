use raylib::prelude::*;

macro_rules! add_menu_color {
    ($ui:tt, $name:tt, $src:tt, $dst:tt) => {
        if $ui.menu_item($name) {
            *$dst.lock().unwrap() = Color::$src;
        }
    };
}

use std::sync::Mutex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init().width(640).height(480).build();

    let logo = rl.load_texture_from_image(&thread, &Image::load_image("static/billboard.png")?)?;

    let mut color = Mutex::new(Color::WHITE);

    while (!rl.window_should_close()) {
        let mut d = rl.begin_drawing(&thread);
        {
            let color = *color.lock().unwrap();
            d.clear_background(color);
            d.draw_texture(
                &logo,
                d.get_screen_width() / 2 - logo.width() / 2,
                d.get_screen_height() / 2 - logo.height() / 2,
                color,
            );
        }

        d.start_imgui(|ui| {
            ui.window("Color Changer")
                .position([0.0, 0.0], ::imgui::Condition::Always)
                .size(
                    [d.get_screen_width() as f32, 32.0],
                    ::imgui::Condition::Always,
                )
                .movable(false)
                .resizable(false)
                .title_bar(false)
                .build(|| {
                    ui.menu("Colors", || {
                        add_menu_color!(ui, "Light Gray", LIGHTGRAY, color);
                        add_menu_color!(ui, "Gray", GRAY, color);
                        add_menu_color!(ui, "Dark Gray", DARKGRAY, color);
                        add_menu_color!(ui, "Yellow", YELLOW, color);
                        add_menu_color!(ui, "Gold", GOLD, color);
                        add_menu_color!(ui, "Orange", ORANGE, color);
                        add_menu_color!(ui, "Pink", PINK, color);
                        add_menu_color!(ui, "Red", RED, color);
                        add_menu_color!(ui, "Maroon", MAROON, color);
                        add_menu_color!(ui, "Green", GREEN, color);
                        add_menu_color!(ui, "Lime", LIME, color);
                        add_menu_color!(ui, "Dark Green", DARKGREEN, color);
                        add_menu_color!(ui, "Sky Blue", SKYBLUE, color);
                        add_menu_color!(ui, "Blue", BLUE, color);
                        add_menu_color!(ui, "Dark Blue", DARKBLUE, color);
                        add_menu_color!(ui, "Purple", PURPLE, color);
                        add_menu_color!(ui, "Dark Purple", DARKPURPLE, color);
                        add_menu_color!(ui, "Beige", BEIGE, color);
                        add_menu_color!(ui, "Brown", BROWN, color);
                        add_menu_color!(ui, "Dark Brown", DARKBROWN, color);
                        add_menu_color!(ui, "White", WHITE, color);
                        add_menu_color!(ui, "Black", BLACK, color);
                        add_menu_color!(ui, "Blank", BLANK, color);
                        add_menu_color!(ui, "Magenta", MAGENTA, color);
                        add_menu_color!(ui, "Ray White", RAYWHITE, color);
                    })
                });
        });
    }

    Ok(())
}
