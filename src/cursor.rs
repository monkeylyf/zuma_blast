use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{
        Component,
        DenseVecStorage
    },
    prelude::{
        Builder,
        GameData,
        SimpleState,
        StateData,
        World,
        WorldExt,
    },
    renderer::{Camera, ImageFormat, SpriteSheetFormat, SpriteSheet, SpriteRender, Texture},
    ui::{
        Anchor,
        FontHandle,
        LineMode,
        TtfFormat,
        UiTransform,
        UiImage,
        UiText,
    },
    window::ScreenDimensions,
};


pub struct Cursor {
    pub x: f32,
    pub y: f32,
    pub selected: bool,
}


impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_cursor(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut transform = Transform::default();
    transform.set_translation_xyz(50.0 * 10.0, 50.0 * 10.0, 0.0);

    // Assign the sprite for the ball. The ball is the second sprite in the sheet.
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 3);

    world
        .create_entity()
        .with(sprite_render.clone())
        // .with(Cursor {
        //     x: 0.0,
        //     y: 0.0,
        //     selected: false,
        // })
        .with(transform)
        .build();
}
