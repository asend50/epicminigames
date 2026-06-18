/*
By: <Asen Doiron>
Date: 2026-06-05
Program Details: <Program Description Here>
*/

mod modules;

mod menu;
mod clicker;
mod clickermenu;
mod gemcollector;
mod gemcollectormenu;
mod skillcheck;
mod skillcheckmenu;
mod fourcorners;
mod fourcornersmenu;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions; // If you want to customize the loading screen

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Epic Minigames".to_owned(),
        window_width: 1300,
        window_height: 1300,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "menu".to_string();

    let mut tm = TextureManager::new();

    let loading_options = LoadingScreenOptions {
       title: Some("Epic Minigames".to_string()),
       background_color: DARKBLUE,
       bar_fill_color: PURPLE,
       // Use default values for other options
       ..Default::default()
   };
   tm.preload_with_loading_screen(&["assets/desktopbackground.png", "assets/filefolder.png", "assets/filefolderhover.png", "assets/pixelchicken.png", "assets/pixelchickenhover.png", "assets/pixelfarm.png", "assets/pixelegg.png", "assets/pixelarrow.png", "assets/pixelarrowhover.png", "assets/stevespriteright.png", "assets/stevespriteleft.png", "assets/cave.png", "assets/diamond.png", "assets/emerald.png", "assets/gold.png", "assets/amethyst.png", "assets/minecart.png", "assets/chickenclickertitle.png", "assets/gemcollectortitle.png", "assets/skillchecktitle.png", "assets/bar.png", "assets/greenbar.png", "assets/blackbar.png", "assets/redbackground.png", "assets/gear.png", "assets/fourcornerstitle.png", "assets/redcorner.png", "assets/orangecorner.png", "assets/bluecorner.png", "assets/greencorner.png", "assets/circle.png", "assets/you.png", "assets/colorcircle.png", "assets/youwin.png", "assets/pixelx.png", "assets/pixelxhover.png", "assets/trophy.png", "assets/skillcheckbackground.png", "assets/skillcheckbackgroundred.png"], Some(loading_options)).await;

    let mut last_switch = get_time() - 0.02;

    let mut chickenclickerscore = 0;

    let mut gemcollectorscore = 0;

    let mut skillcheckpoints = 0;

    let mut fourcournersscore = 0;
    
    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore) = match current_screen.as_str() {
                "menu" => menu::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "clicker" => clicker::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "clickermenu" => clickermenu::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "gemcollector" => gemcollector::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "gemcollectormenu" => gemcollectormenu::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "skillcheck" => skillcheck::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "skillcheckmenu" => skillcheckmenu::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "fourcorners" => fourcorners::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                "fourcornersmenu" => fourcornersmenu::run(tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
