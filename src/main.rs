/*
By: <Your Name Here>
Date: 2026-09-21
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
 use crate::ui::label::Label;
 use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "hello_11u".to_string(),
        window_width: 1250,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let btn_name = TextButton::new(50.0, 600.0, 150.0, 50.0, "Name", BLUE, GREEN, 30);
 let btn_age = TextButton::new(250.0, 600.0, 150.0, 50.0, "Age", BLUE, GREEN, 30);
 let btn_school = TextButton::new(450.0, 600.0, 150.0, 50.0, "School", BLUE, GREEN, 30);
 let btn_food = TextButton::new(650.0, 600.0, 150.0, 50.0, "Food", BLUE, GREEN, 30);
 let btn_sport = TextButton::new(850.0, 600.0, 150.0, 50.0, "Sport", BLUE, GREEN, 30);
 let btn_exit = TextButton::new(1050.0, 600.0, 150.0, 50.0, "Exit", BLUE, GREEN, 30);
 let mut lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
 lbl_out.with_fixed_size(450.0, 450.0).with_border(BLACK, 1.0);

let tm = TextureManager::new();
tm.preload_with_loading_screen(&["assets/happy.png","assets/maze.png"], None, None).await;
let mut img_out = StillImage::from_preload(
        tm.get_preload("assets/happy.png").unwrap(),
        400.0,
        400.0,
        550.0,
        50.0,
        true,
        1.0,
    );
    loop {
        clear_background(WHITE);
        //draw_grid(50.0, BROWN);
        if btn_name.click() {
            lbl_out.set_text("Mathew Dusome is HERE");
            img_out.set_preload(tm.get_preload("assets/maze.png").unwrap());
        }
        if btn_age.click() {}
        if btn_school.click() {}
        if btn_food.click() {}
        if btn_sport.click() {}
        if btn_exit.click() {
            break;
        }
        lbl_out.draw();
        img_out.draw();
        next_frame().await;
    }
}
