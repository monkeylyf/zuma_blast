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
    renderer::{Camera, ImageFormat, SpriteSheetFormat, SpriteSheet, Texture},
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

pub struct Arena;

impl SimpleState for Arena {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let sprite_sheet_handle = init_sprites(world);

        world.register::<Maze>();
        world.register::<Handle<SpriteSheet>>();

        init_maze(world, sprite_sheet_handle);
        init_camera(world, &dimensions);
        init_title(world);
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn init_sprites(world: &mut World) -> Handle<SpriteSheet> {
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            //"texture/maze.png",
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub struct Maze {
    pub height: f32,
    pub width: f32,
}

impl Maze {
    fn new() -> Maze {
        Maze {
            height: 8.0,
            width: 16.0,
        }
    }
}

impl Component for Maze {
    type Storage = DenseVecStorage<Self>;
}


fn init_maze(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(50.0, 50.0, 0.0);

    // Create maze entity.
    world
        .create_entity()
        .with(Maze::new())
        .with(sprite_sheet_handle)
        .with(transform)
        .build();
}

fn init_title(world: &mut World) {
    // this creates the simple gray background UI element.
    world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
                "".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                30.0,
                -30.0,
                0.0,
                440.0,
                50.0,
                ))
        .build();

    let font: FontHandle = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );


    world
        .create_entity()
        .with(UiTransform::new(
                "id".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                40.0,
                -40.0,
                1.0,
                440.0,
                50.0,
            ))
        .with(UiText::new(
                font,
                "Microsoft Erge".to_string(),
                [1.0, 1.0, 1.0, 1.0],  // Color.
                30.0,  // Font size.
                LineMode::Single,
                Anchor::TopLeft,
                ))
        .build();
}
