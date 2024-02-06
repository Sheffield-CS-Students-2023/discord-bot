use crate::{Context, Error};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use poise::{command, CreateReply};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusttype::{Font, Scale};
use serenity::all::CreateAttachment;
use std::io::Cursor;

fn generate_bingo_card(cells: Vec<Vec<&str>>) -> Vec<u8> {
    // Constants for bingo card dimensions
    const CELL_SIZE: u32 = 50;
    const CARD_WIDTH: u32 = 5 * CELL_SIZE;
    const CARD_HEIGHT: u32 = 5 * CELL_SIZE;
    const BORDER_WIDTH: u32 = 2;

    // Create a new RGBA image
    let mut img = ImageBuffer::new(CARD_WIDTH + BORDER_WIDTH, CARD_HEIGHT + BORDER_WIDTH);

    // Define cell colors
    let cell_color = Rgba([255u8, 255u8, 255u8, 255u8]);
    let border_color = Rgba([0u8, 0u8, 0u8, 255u8]);

    // Draw cells
    for (i, row) in cells.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let x = j as u32 * CELL_SIZE + BORDER_WIDTH;
            let y = i as u32 * CELL_SIZE + BORDER_WIDTH;
            for dx in 0..CELL_SIZE {
                for dy in 0..CELL_SIZE {
                    let px = x + dx;
                    let py = y + dy;
                    img.put_pixel(px, py, cell_color);
                }
            }
            // Draw cell borders
            for dx in 0..BORDER_WIDTH {
                for dy in 0..CELL_SIZE {
                    let px = x + dx;
                    let py = y + dy;
                    img.put_pixel(px, py, border_color);
                }
            }
            for dx in 0..CELL_SIZE {
                for dy in 0..BORDER_WIDTH {
                    let px = x + dx;
                    let py = y + dy;
                    img.put_pixel(px, py, border_color);
                }
            }
            // Draw text scaled to fit into the cell
            let font = Vec::from(include_bytes!("Arial.ttf") as &[u8]);
            let font = Font::try_from_vec(font).expect("Failed to load font file");
            let scale = Scale::uniform(20.0); // Adjust text size as needed
            let offset = (
                (CELL_SIZE - (cell.len() as u32 * 10)) / 2,
                (CELL_SIZE - 20) / 2,
            ); // Adjust offset based on text size
            draw_text_mut(
                &mut img,
                Rgba([0u8, 0u8, 0u8, 255u8]),
                x as i32 + offset.0 as i32,
                y as i32 + offset.1 as i32,
                scale,
                &font,
                cell,
            );
        }
    }

    // Draw card borders
    for x in 0..CARD_WIDTH + BORDER_WIDTH {
        for y in 0..BORDER_WIDTH {
            img.put_pixel(x, y, border_color);
            img.put_pixel(x, CARD_HEIGHT + BORDER_WIDTH - 1 - y, border_color);
        }
    }
    for y in 0..CARD_HEIGHT + BORDER_WIDTH {
        for x in 0..BORDER_WIDTH {
            img.put_pixel(x, y, border_color);
            img.put_pixel(CARD_WIDTH + BORDER_WIDTH - 1 - x, y, border_color);
        }
    }

    // Create a buffer to hold the PNG data
    let mut buf = Cursor::new(Vec::new());

    // Write the image into the buffer
    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .expect("Failed to write PNG data to buffer");

    // Extract the inner Vec<u8> from the Cursor and return it
    buf.into_inner()
}

fn create_bingo_card(items: Vec<&str>) -> Vec<Vec<&str>> {
    // Ensure there are enough unique items for a Bingo card
    if items.len() < 25 {
        panic!("Not enough items to generate a Bingo card");
    }

    // Shuffle the items randomly
    let mut rng = thread_rng();
    let mut shuffled_items = items.clone();
    shuffled_items.shuffle(&mut rng);

    // Extract 25 items for the Bingo card
    let mut bingo_card = Vec::new();
    for i in 0..5 {
        let row = shuffled_items[i * 5..(i + 1) * 5].to_vec();
        bingo_card.push(row);
    }

    // Replace the center item with "FREE"
    bingo_card[2][2] = "FREE";

    bingo_card
}

#[command(prefix_command)]
pub async fn bingo(ctx: Context<'_>) -> Result<(), Error> {
    println!("Bingo command");
    let items = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    let bingo_card = create_bingo_card(items);
    let img = generate_bingo_card(bingo_card);

    let reply = CreateReply::default()
        .attachment(CreateAttachment::bytes(img, "bingo.png".to_string()))
        .content("Your bingo card");

    ctx.send(reply).await?;

    Ok(())
}
