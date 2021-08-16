use amethyst::{
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()>{
    let app_root = application_root_dir()?;

    Ok(())
}
