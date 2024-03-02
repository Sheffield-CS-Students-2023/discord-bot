use crate::{Context, Error};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use poise::{command, CreateReply};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusttype::{point, Font, Scale};
use serenity::all::CreateAttachment;
use std::io::Cursor;

const DIMENSIONS: usize = 5;
const MAX_TEXT_SCALE: Scale = Scale { x: 25.0, y: 25.0 };

fn get_scale_num(text: &str) -> Scale {
    // Get the scale of the text based on number of lines
    let lines = text.lines().count();
    match lines {
        1 => Scale::uniform(25.0),
        2 => Scale::uniform(18.0),
        3 => Scale::uniform(15.0),
        _ => Scale::uniform(14.0),
    }
}

fn get_multiplier_from_line(lines: usize) -> usize {
    match lines {
        2 => 80 / 10,
        3 => 75 / 10,
        _ => 60 / 10,
    }
}

fn generate_bingo_card(cells: Vec<Vec<&str>>) -> Vec<u8> {
    // Constants for bingo card dimensions
    const CELL_SIZE: u32 = 150;
    const CARD_WIDTH: u32 = DIMENSIONS as u32 * CELL_SIZE;
    const CARD_HEIGHT: u32 = DIMENSIONS as u32 * CELL_SIZE;
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
            // let scale = Scale::uniform(20.0); // Adjust text size as needed
            // let scale = get_scale_num(cell);
			let scale = cell.get_scale(CELL_SIZE, &font);

            let cell_string = cell.split_into_lines(CELL_SIZE, scale, &font);

            // if there is more than one line
            if cell_string.len() > 1 {
                let longest_line = cell_string.iter().max_by_key(|line| line.len()).unwrap();
                for (i, line) in cell_string.iter().enumerate() {
                    let offset = (
                        // center text given on height (number of lines) and width (length of longest line)
                        CELL_SIZE as i32
                            - (longest_line.len() as i32
                                * get_multiplier_from_line(cell_string.len()) as i32),
                        (CELL_SIZE as i32 - (cell_string.len() as i32 * 20)) / 2,
                    );
                    let offset = (0, 0);
                    draw_text_mut(
                        &mut img,
                        Rgba([0u8, 0u8, 0u8, 255u8]),
                        x as i32 + offset.0 as i32,
                        y as i32 + offset.1 as i32 + (i as i32 * 20),
                        scale,
                        &font,
                        line,
                    );
                }
            } else {
                let offset = (
                    // center text given on height (number of lines) and width (length of longest line)
                    (CELL_SIZE as i32 - (cell.len() as i32 * 10)) / 2,
                    (CELL_SIZE as i32 - (cell.lines().count() as i32 * 20)) / 2,
                );
                draw_text_mut(
                    &mut img,
                    Rgba([0u8, 0u8, 0u8, 255u8]),
                    x as i32 + offset.0,
                    y as i32 + offset.1,
                    scale,
                    &font,
                    cell,
                );
            }
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
    if items.len() < (DIMENSIONS * DIMENSIONS) {
        panic!("Not enough items to generate a Bingo card");
    }

    // Shuffle the items randomly
    let mut rng = thread_rng();
    let mut shuffled_items = items.clone();
    shuffled_items.shuffle(&mut rng);

    // Extract 25 items for the Bingo card
    let mut bingo_card = Vec::new();
    for i in 0..DIMENSIONS {
        let row = shuffled_items[i * DIMENSIONS..(i + 1) * DIMENSIONS].to_vec();
        bingo_card.push(row);
    }

    // Replace the center item with "FREE"
    let middle = (DIMENSIONS as f32 / 2.0).floor() as usize;
    bingo_card[middle][middle] = "FREE";

    bingo_card
}

#[command(prefix_command)]
pub async fn bingo(ctx: Context<'_>) -> Result<(), Error> {
    let items: Vec<_> = include_str!("bingo_words.txt").lines().collect();

    let bingo_card = create_bingo_card(items);
    let img = generate_bingo_card(bingo_card);

    let reply = CreateReply::default()
        .attachment(CreateAttachment::bytes(img, "bingo.png".to_string()))
        .content("Your bingo card");

    ctx.send(reply).await?;

    Ok(())
}

trait StringSizer {
    fn measure_width(&self, scale: Scale, font: &Font) -> f32;
    fn split_into_lines(&self, max_width: u32, scale: Scale, font: &Font) -> Vec<&str>;
    fn get_scale(&self, max_width: u32, font: &Font) -> Scale;
}

impl StringSizer for str {
    fn measure_width(&self, scale: Scale, font: &Font) -> f32 {
        let width = font
            .layout(self, scale, point(0.0, 0.0))
            .last()
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .unwrap_or(0.0);

        width
    }

    fn split_into_lines(&self, max_width: u32, scale: Scale, font: &Font) -> Vec<&str> {
        let width = self.measure_width(scale, &font);

        if width <= max_width as f32 {
            vec![self]
        } else {
            let end_of_line = font
                .layout(self, scale, point(0.0, 0.0))
                .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
                .position(|g| g > max_width as f32)
                .expect("Given an empty line");

            let space_position = end_of_line
                - self[0..end_of_line as usize]
                    .bytes()
                    .rev()
                    .position(|b| b == b' ')
					.expect("Given a word thats too large to fit in the max_width");

            let next_lines = &self[space_position..].split_into_lines(max_width, scale, font);

            let mut ret = vec![&self[0..space_position - 1]];
            ret.extend(next_lines);
            ret
        }
    }

    fn get_scale(&self, max_width: u32, font: &Font) -> Scale {
        let largest_width = self
            .split(' ')
            .map(|s| s.measure_width(MAX_TEXT_SCALE, font))
			// We can't use `max` here because f32 does not impliment Ord
            .reduce(|f, max| f.max(max))
			.expect("Given empty string");

		if largest_width > max_width as f32 {
			Scale::uniform(MAX_TEXT_SCALE.x * (max_width as f32 / largest_width))
		} else {
			MAX_TEXT_SCALE
		}
    }
}
