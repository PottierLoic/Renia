use macroquad::{miniquad::conf, prelude::*};

mod board;
use board::*;
mod constants;

fn window_conf() -> conf::Conf {
    conf::Conf {
        window_title: "Lenia cellular automata".to_owned(),
        window_width: constants::WINDOW_SIZE,
        window_height: constants::WINDOW_SIZE,
        ..Default::default()
    }
}

fn create_board_texture(board: &Board) -> Texture2D {
    let mut image_data = vec![0u8; constants::BOARD_SIZE * constants::BOARD_SIZE * 4];
    for x in 0..constants::BOARD_SIZE {
        for y in 0..constants::BOARD_SIZE {
            let index = (y * constants::BOARD_SIZE + x) * 4;
            let val = (board.tiles[y * constants::BOARD_SIZE + x] * 255.0) as u8;
            image_data[index] = val;
            image_data[index + 1] = val;
            image_data[index + 2] = val;
            image_data[index + 3] = 255;
        }
    }
    let image = Image {
        bytes: image_data,
        width: constants::BOARD_SIZE as u16,
        height: constants::BOARD_SIZE as u16,
    };
    Texture2D::from_image(&image)
}

fn draw_board(board_texture: Texture2D) {
    draw_texture_ex(
        &board_texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                constants::BOARD_SIZE as f32 * constants::TILE_SIZE as f32,
                constants::BOARD_SIZE as f32 * constants::TILE_SIZE as f32,
            )),
            ..Default::default()
        },
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new();
    loop {
        clear_background(BLACK);
        draw_board(create_board_texture(&board));
        // update_board();
        next_frame().await;
    }
}
