/*
By: <Asen Doiron>
Date: 2026-06-05
Program Details: <The purpose of this minigame is to make the user choose one of the four corners. If the corner they choose is also chosen by the program, they lose. If they choose a corner that isnt choosen by the program five times in a row they win.>
*/

use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::image_button::ImageButton;
use miniquad::date;

pub async fn run(tm: TextureManager, chickenclickerscore: i32, gemcollectorscore: i32, skillcheckpoints: i32, _fourcournersscore: i32) -> (String, TextureManager, i32, i32, i32, i32) {

    let mut fourcournersscore = 0;
    let mut fourcournersscorevalue = fourcournersscore;

    let btn_arrowbutton = ImageButton::new(
        50.0,  // x position
        50.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/pixelarrow.png",        // normal state image
        "assets/pixelarrowhover.png",  // hover state image
    ).await;

    let mut img_redcorner = StillImage::new(
        "",
        575.0,  // width
        575.0,  // height
        0.0,  // x position
        575.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_redcorner.set_preload(tm.get_preload("assets/redcorner.png").unwrap());
    let redcorner = img_redcorner;

    let mut img_orangecorner = StillImage::new(
        "",
        575.0,  // width
        575.0,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_orangecorner.set_preload(tm.get_preload("assets/orangecorner.png").unwrap());
    let orangecorner = img_orangecorner;

    let mut img_bluecorner = StillImage::new(
        "",
        575.0,  // width
        575.0,  // height
        575.0,  // x position
        575.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_bluecorner.set_preload(tm.get_preload("assets/bluecorner.png").unwrap());
    let bluecorner = img_bluecorner;

    let mut img_greencorner = StillImage::new(
        "",
        575.0,  // width
        575.0,  // height
        575.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_greencorner.set_preload(tm.get_preload("assets/greencorner.png").unwrap());
    let greencorner = img_greencorner;

    let mut img_circle = StillImage::new(
        "",
        150.0,  // width
        150.0,  // height
        500.0,  // x position
        500.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_circle.set_preload(tm.get_preload("assets/circle.png").unwrap());
    let circle = img_circle;

    let mut img_you = StillImage::new(
        "",
        150.0,  // width
        150.0,  // height
        1000.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    img_you.set_preload(tm.get_preload("assets/you.png").unwrap());
    let mut you = img_you;

    let lbl_box1 = Label::new("1", 500.0, 700.0, 80);
    let lbl_box2 = Label::new("2", 500.0, 500.0, 80);
    let lbl_box3 = Label::new("3", 615.0, 500.0, 80);
    let lbl_box4 = Label::new("4", 615.0, 700.0, 80);
    let mut lbl_circle = Label::new("", 555.0, 590.0, 80);
    let mut lbl_result = Label::new("", 450.0, 160.0, 80);
    let mut lbl_reset = Label::new("", 350.0, 250.0, 50);
    let mut lbl_score = Label::new("0", 555.0, 80.0, 80);

    rand::srand(date::now() as u64);

    let mut x = 2000.0;
    let mut y = 0.0;

    let mut play = 1;

    loop {
        clear_background(WHITE);
        redcorner.draw();
        orangecorner.draw();
        bluecorner.draw();
        greencorner.draw();
        circle.draw();
        you.draw();
        lbl_box1.draw();
        lbl_box2.draw();
        lbl_box3.draw();
        lbl_box4.draw();
        lbl_circle.draw();
        lbl_result.draw();
        lbl_reset.draw();
        lbl_score.draw();

        if btn_arrowbutton.click() {
            return ("fourcornersmenu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscorevalue);
        }

        if is_key_pressed(KeyCode::Key1) && play == 1 {
            x = 250.0;
            y = 750.0;
            play = 0;

            let randombox = rand::gen_range(1, 5);
            lbl_circle.set_text(&randombox.to_string());

            if randombox == 1 {
                lbl_result.set_text("You Lose!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore = 0;
                lbl_score.set_text(&fourcournersscore.to_string());
            } 

            else {
                lbl_result.set_text("You Win!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore += 1;
                lbl_score.set_text(&fourcournersscore.to_string());
            }

        }

        if is_key_pressed(KeyCode::Key2) && play == 1 {
            x = 250.0;
            y = 250.0;
            play = 0;

            let randombox = rand::gen_range(1, 5);
            lbl_circle.set_text(&randombox.to_string());

            
            if randombox == 2 {
                lbl_result.set_text("You Lose!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore = 0;
                lbl_score.set_text(&fourcournersscore.to_string());
            } 

            else {
                lbl_result.set_text("You Win!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore += 1;
                lbl_score.set_text(&fourcournersscore.to_string());
            }
        }
        

        if is_key_pressed(KeyCode::Key3) && play == 1 {
            x = 750.0;
            y = 250.0;
            play = 0;

            let randombox = rand::gen_range(1, 5);
            lbl_circle.set_text(&randombox.to_string());

            if randombox == 3 {
                lbl_result.set_text("You Lose!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore = 0;
                lbl_score.set_text(&fourcournersscore.to_string());
            } 

            else {
                lbl_result.set_text("You Win!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore += 1;
                lbl_score.set_text(&fourcournersscore.to_string());
            }
        }

        if is_key_pressed(KeyCode::Key4) && play == 1 {
            x = 750.0;
            y = 750.0;
            play = 0;

            let randombox = rand::gen_range(1, 5);
            lbl_circle.set_text(&randombox.to_string());

            if randombox == 4 {
                lbl_result.set_text("You Lose!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore = 0;
                lbl_score.set_text(&fourcournersscore.to_string());
            }

            else {
                lbl_result.set_text("You Win!");
                lbl_reset.set_text("Press Space to Reset");
                fourcournersscore += 1;
                lbl_score.set_text(&fourcournersscore.to_string());
            }
        }
    
        you.set_x(x);
        you.set_y(y);

        if is_key_pressed(KeyCode::Space) && play == 0 {
            play = 1;
            lbl_result.set_text("");
            lbl_reset.set_text("");
            lbl_circle.set_text("");
            x = 2000.0;
            y = 0.0;
        }

        if fourcournersscore == 5 {
            fourcournersscorevalue = 1;
            return ("menu".to_string(), tm, chickenclickerscore, gemcollectorscore, skillcheckpoints, fourcournersscorevalue);
        }
    

        next_frame().await;
    }
}