use super::physics_system::positioning::{
    collision::Interaction,
    positioning::{Physics, Sizable},
};
use crate::ecs::components::npc::Npc;
use ggez::{
    self,
    graphics::{Color, DrawMode, DrawParam, TextFragment},
    Context, GameResult, *,
};

pub fn render(
    physics_components: &Vec<Option<Physics>>,
    npc_components: &Vec<Option<Npc>>,
    current_interaction: &Option<Interaction>,
    ctx: &mut Context,
) -> GameResult {
    for object in physics_components {
        match object {
            Some(physics) => draw_object(ctx, &physics)?,
            None => (),
        }
    }

    match current_interaction {
        Some(interaction) => {
            let interacting_with = interaction.interacting_with;
            draw_interaction(
                npc_components[interacting_with].as_ref().unwrap(),
                physics_components[interacting_with].as_ref().unwrap(),
                ctx,
            )?;
        }
        None => (),
    }

    Ok(())
}

pub fn draw_object(ctx: &mut Context, physics: &Physics) -> GameResult {
    let rect = graphics::Rect::new(
        physics.position.x - physics.size.w_half(),
        physics.position.y - physics.size.h_half(),
        physics.get_size().width,
        physics.get_size().height,
    );

    let rect_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, physics.color)?;

    graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

    Ok(())
}

pub fn draw_interaction(
    npc_component: &Npc,
    npc_physics: &Physics,
    ctx: &mut Context,
) -> GameResult {
    let dialog_box_w = npc_physics.size.width * 3.0;
    let dialog_box_h = npc_physics.size.height * 5.0;

    let dialog_box_x = npc_physics.position.x + npc_physics.size.width * 2.0;
    let dialog_box_y = npc_physics.position.y - npc_physics.size.height * 1.5;

    let rect = graphics::Rect::new(dialog_box_x, dialog_box_y, dialog_box_w, dialog_box_h);

    let rect_mesh = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        rect,
        graphics::Color::from_rgb(247, 241, 227),
    )?;

    graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

    let dialog_text = graphics::Text::new(TextFragment {
        text: npc_component.name.clone(),
        color: Some(Color::BLACK),
        font: Some(graphics::Font::default()),
        ..Default::default()
    });

    let coords = [dialog_box_x, dialog_box_y];
    let params = graphics::DrawParam::default().dest(coords);

    graphics::draw(ctx, &dialog_text, params)?;

    Ok(())
}
