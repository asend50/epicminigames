/*
By: <Asen Doiron>
Date: 2026-06-05
Program Details: <Main menu screen for the game.>
*/

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::image_button::ImageButton;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32, fourcournersscore: i32) -> (String, TextureManager, i32, i32, i32, i32) {

    let mut lbl_title = Label::new("Epic Minigames", 250.0, 200.0, 100);

    let mut lbl_description = Label::new("Beat each minigame to collect each item.\nCollect every item in order to beat the game.", 220.0, 300.0, 35);

    let lbl_chickenclicker = Label::new("Chicken Clicker", 90.0, 815.0, 30);

    let lbl_gemcollector = Label::new("Gem Collector", 350.0, 815.0, 30);

    let lbl_skillcheck = Label::new("Skill Check", 620.0, 815.0, 30);

    let lbl_fourcorners = Label::new("Four Corners", 860.0, 815.0, 30);

    let mut img_background = StillImage::new(
        "",
        1450.0,  // width
        1300.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_background.set_preload(tm.get_preload("assets/desktopbackground.png").unwrap());

    let mut btn_filefolder1 = ImageButton::new(
        125.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;

    let mut btn_filefolder2 = ImageButton::new(
        375.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;

    let mut btn_filefolder3 = ImageButton::new(
        625.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;

    let mut btn_filefolder4 = ImageButton::new(
        875.0,  // width
        650.0,  // height
        125.0,  // x position
        125.0,   // y position
        "assets/filefolder.png",        // normal state image
        "assets/filefolderhover.png",  // hover state image
    ).await;
    
    let mut img_egg = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        200.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_gem = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        400.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_gear = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        600.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_colorcircle = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        100.0,  // height
        800.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_trophy = StillImage::new(
        "",     // Empty string creates a transparent image
        450.0,  // width
        450.0,  // height
        325.0,  // x position
        725.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let btn_xbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        75.0,  // width
        75.0,   // height
        "assets/pixelx.png",        // normal state image
        "assets/pixelxhover.png",  // hover state image
    ).await;

    loop {
        clear_background(WHITE);
        img_background.draw();
        img_trophy.draw();
        img_egg.draw();
        img_gem.draw();
        img_gear.draw();
        img_colorcircle.draw();
        lbl_title.draw();
        lbl_description.draw();
        lbl_chickenclicker.draw();
        lbl_gemcollector.draw();
        lbl_skillcheck.draw();
        lbl_fourcorners.draw();

        if btn_filefolder1.click() {
            return ("clickermenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore);
        }

        if btn_filefolder2.click() {
            return ("gemcollectormenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore);
        }

        if btn_filefolder3.click() {
            return ("skillcheckmenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore);
        }

        if btn_filefolder4.click() {
            return ("fourcornersmenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore);
        }

    if chickenclickerscore == 1 {
        img_egg.set_preload(tm.get_preload("assets/pixelegg.png").unwrap());
        btn_filefolder1.enabled = false;
    }

    if gemcollectorscore == 1 {
        img_gem.set_preload(tm.get_preload("assets/diamond.png").unwrap());
        btn_filefolder2.enabled = false;
    }

    if skillcheckpoints == 1 {
        img_gear.set_preload(tm.get_preload("assets/gear.png").unwrap());
        btn_filefolder3.enabled = false;
    }

    if fourcournersscore == 1 {
        img_colorcircle.set_preload(tm.get_preload("assets/colorcircle.png").unwrap());
        btn_filefolder4.enabled = false;
    }

    if chickenclickerscore == 1 && gemcollectorscore == 1 && skillcheckpoints == 1 && fourcournersscore == 1 {
        lbl_title.set_text("   You Win!");
        lbl_description.set_text("      Congratulations on beating\nevery minigame and collecting every item.");
        img_trophy.set_preload(tm.get_preload("assets/trophy.png").unwrap());
        if btn_xbutton.click() {
        return ("break".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscore);
    }

    }
        next_frame().await;
    }
}