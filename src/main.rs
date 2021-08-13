use quicksilver::{
    input::{ButtonState, Key},
    prelude::{Rectangle},
    geom::{Vector, Shape},
    lifecycle::{run, Settings, State, Window, Asset},
    graphics::{Image, Color, Font, FontStyle, Background::{Blended, Img}},
    Result,
    Future
};

use rand::Rng;
use std::collections::HashMap;


struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    map_size: Vector,
    map: Vec<Tile>,
    tileset: Asset<HashMap<char, Image>>,
    tile_size_px: Vector,
    cursor: Cursor,
    selected_tile: SelectedTile,
}


#[derive(Clone, Debug, PartialEq)]
struct Cursor {
    pos: Vector,
    glyph: char,
}

#[derive(Clone, Debug, PartialEq)]
struct SelectedTile {
    pos: Vector,
    glyph: char,
    is_selected: bool,
}


#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}

fn gen_random_tile_color() -> Color {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => Color::CYAN,
        1 => Color::ORANGE,
        2 => Color::MAGENTA,
        3 => Color::PURPLE,
        _ => Color::WHITE,
    }
}


fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let glyph = if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                '#'
            } else {
                'o'
            };
            let color = if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                Color::BLACK
            } else {
                gen_random_tile_color()
            };
            let tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: glyph,
                color: color,
            };

            map.push(tile);
        }
    }
    map
}

fn is_adjacent_tiles(x: f32, y: f32, xx: f32, yy: f32) -> bool {
    // Diagnals not included.
    return (x - xx).abs() + (y - yy).abs() <= 1.0;
}


#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    glyph: char,
    color: Color,
    hp: i32,
    max_hp: i32,
}


impl State for Game {
    // Load the assets and initialise the game
    fn new() -> Result<Self> {
        let font_mononoki = "./font/mononoki-Regular.ttf";
        let font_sqaure = "./font/square.ttf";
        let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("The Adventure of Microsoft Erge", &FontStyle::new(50.0, Color::BLACK))
        }));

        let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Microsoft Weather & Finance",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        let map_size = Vector::new(20, 15);
        let map = generate_map(map_size);

        let tile_size_px = Vector::new(24, 24);
        let game_glyphs = "#o_S";

        let tileset = Asset::new(Font::load(font_sqaure).and_then(move |text| {
            let tiles = text
                .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
                .expect("Could not render the font tileset.");
            let mut tileset = HashMap::new();
            for (index, glyph) in game_glyphs.chars().enumerate() {
                let pos = (index as i32 * tile_size_px.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                tileset.insert(glyph, tile);
            }
            Ok(tileset)
        }));

        let cursor = Cursor {
            pos: Vector::new(1, 1),
            glyph: '_',
        };

        let selected_tile = SelectedTile {
            pos: Vector::new(0, 0),
            glyph: 'S',
            is_selected: false,
        };

        Ok(Self {
            title,
            mononoki_font_info,
            map_size,
            map,
            tileset,
            tile_size_px,
            cursor,
            selected_tile,
        })
    }

    // Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;

        let (cursor, selected_tile, map_size) = (&mut self.cursor, &mut self.selected_tile, &self.map_size);
        let x = cursor.pos.x as usize;
        let y = cursor.pos.y as usize;

        // Move left inbound.
        if (window.keyboard()[Key::Left] == Pressed || window.keyboard()[Key::H] == Pressed)
            && x > 1 {
                if !selected_tile.is_selected {
                    cursor.pos.x -= 1.0;
                } else if is_adjacent_tiles(cursor.pos.x - 1.0, cursor.pos.y, selected_tile.pos.x, selected_tile.pos.y) {
                    cursor.pos.x -= 1.0;
                } else {
                    // Do nothing.
                }
            }
        // Move right inbound.
        if (window.keyboard()[Key::Right] == Pressed || window.keyboard()[Key::L] == Pressed)
            && map_size.x - 2.0 > cursor.pos.x {
                if !selected_tile.is_selected {
                    cursor.pos.x += 1.0;
                } else if is_adjacent_tiles(cursor.pos.x + 1.0, cursor.pos.y, selected_tile.pos.x, selected_tile.pos.y) {
                    cursor.pos.x += 1.0;
                } else {
                    // Do nothing.
                }
            }
        // Move up inbound.
        if (window.keyboard()[Key::Up] == Pressed || window.keyboard()[Key::K] == Pressed )
            && y > 1 {
                if !selected_tile.is_selected {
                    cursor.pos.y -= 1.0;
                } else if is_adjacent_tiles(cursor.pos.x, cursor.pos.y - 1.0, selected_tile.pos.x, selected_tile.pos.y) {
                    cursor.pos.y -= 1.0;
                } else {
                    // Do nothing.
                }
            }
        // Move down inbound.
        if (window.keyboard()[Key::Down] == Pressed || window.keyboard()[Key::J] == Pressed)
            && map_size.y - 2.0 > cursor.pos.y {
                if !selected_tile.is_selected {
                    cursor.pos.y += 1.0;
                } else if is_adjacent_tiles(cursor.pos.x, cursor.pos.y + 1.0, selected_tile.pos.x, selected_tile.pos.y) {
                    cursor.pos.y += 1.0;
                } else {
                    // Do nothing.
                }
            }
        // Unselecte tile.
        if window.keyboard()[Key::Space] == Pressed {
            selected_tile.is_selected = false;
        }
        // Selete tile or unselete a selected one.
        if window.keyboard()[Key::Return] == Pressed {
            // println!("<{}, {}>", cursor.pos.x, cursor.pos.y);
            if selected_tile.is_selected && selected_tile.pos.x == cursor.pos.x && selected_tile.pos.y == cursor.pos.y {
                // Unselect a selected tile.
                selected_tile.is_selected = false;
            } else {
                // Select a tile.
                selected_tile.pos.x = cursor.pos.x;
                selected_tile.pos.y = cursor.pos.y;
                selected_tile.is_selected = true;
            }
        }

        Ok(())
    }

    // Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.title.execute(|image| {
            window.draw(
                &image
                .area()
                .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
                );
            Ok(())
        })?;

        self.mononoki_font_info.execute(|image| {
            window.draw(
                &image
                .area()
                .translate((2, window.screen_size().y as i32 - 60)),
                Img(&image),
                );
            Ok(())
        })?;

        let tile_size_px = self.tile_size_px;
        let offset_px = Vector::new(50, 120);

        // Draw map.
        let (tileset, map, selected) = (&mut self.tileset, &self.map, &self.selected_tile);
        tileset.execute(|tileset| {
            for tile in map.iter() {
                let pos_px = tile.pos.times(tile_size_px);
                if selected.is_selected && selected.pos.x == tile.pos.x && selected.pos.y == tile.pos.y {
                    // Draw selected tile, if any.
                    if let Some(image) = tileset.get(&'0') {
                        window.draw(
                            &Rectangle::new(offset_px + pos_px, image.area().size()),
                            Blended(&image, tile.color),
                            );
                    }
                } else if let Some(image) = tileset.get(&tile.glyph) {
                    // Draw tile.
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, tile.color),
                        );
                }
            }
            Ok(())
        })?;

        // Draw cursor.
        let (tileset, cursor) = (&mut self.tileset, &self.cursor);
        tileset.execute(|tileset| {
            if let Some(image) = tileset.get(&cursor.glyph) {
                let pos_px = offset_px + cursor.pos.times(tile_size_px);
                window.draw(
                    &Rectangle::new(pos_px, image.area().size()),
                    Blended(&image, Color::BLACK),
                    );
            }
            Ok(())
        })?;

        Ok(())
    }
}


fn main() {
    // let settings = Settings {
    //     ..Default::default()
    // };
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<Game>("Zuma Blast", Vector::new(800, 600), settings);
}
