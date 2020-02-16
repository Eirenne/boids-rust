use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod states;
mod components;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with(systems::separation::SeparationSystem, "separation_system", &[])
        .with(systems::cohesion::CohesionSystem, "cohesion_system", &[])
        .with(systems::alignment::AlignmentSystem, "alignment_system", &[])
        .with(systems::movement::MovementSystem, "movement_system", &["separation_system",
            "cohesion_system", "alignment_system"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(resources,
                                    states::game::GameState, game_data)?;
    game.run();

    Ok(())
}