use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::{
        Application,
        GameDataBuilder,
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    start_logger,
};

mod state;


fn main() -> amethyst::Result<()>{
    start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");
    let key_bindngs_config = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
            .with_bindings_from_file(&key_bindngs_config)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config)?
                         .with_clear([0.34, 0.36, 0.52, 1.0]))
            .with_plugin(RenderUi::default())
            .with_plugin(RenderFlat2D::default()))?;

    let mut game = Application::new(resources, state::Map, game_data)?;
    game.run();

    Ok(())
}
