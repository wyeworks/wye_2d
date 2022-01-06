use super::physics_system::positioning::{
    collision::Interaction,
    positioning::{Physics, Sizable},
};
use crate::ecs::{components::npc::Npc, game_state::EntityIndex};
use ggez::{
    self,
    graphics::{Color, DrawMode, DrawParam, TextFragment},
    Context, GameResult, *,
};

pub fn render(
    physics_components: &Vec<Option<Physics>>,
    player_physics: &Physics,
    npcs_components: &Vec<Option<Npc>>,
    current_interaction: &Option<Interaction>,
    interacting_with: &Option<EntityIndex>,
    ctx: &mut Context,
) -> GameResult {
    draw_object(ctx, &player_physics)?;

    for object in physics_components {
        match object {
            Some(physics) => draw_object(ctx, &physics)?,
            None => (),
        }
    }

    match current_interaction {
        Some(interaction) => {
            draw_interaction(
                npcs_components[interacting_with.unwrap()]
                    .as_ref()
                    .unwrap(),
                physics_components[interacting_with.unwrap()]
                    .as_ref()
                    .unwrap(),
                &interaction,
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
    interaction: &Interaction,
    ctx: &mut Context,
) -> GameResult {
    let dialog_box_w = npc_physics.size.width * 3.0;
    let dialog_box_h = npc_physics.size.height * 4.0;

    let dialog_box_x = npc_physics.position.x + npc_physics.size.width * 2.0;
    let dialog_box_y = npc_physics.position.y - npc_physics.size.height * 1.5;

    draw_dialog_box(dialog_box_x, dialog_box_y, dialog_box_w, dialog_box_h, ctx)?;

    draw_dialog_box_content(dialog_box_x, dialog_box_y, npc_component, interaction, ctx)?;

    Ok(())
}

fn draw_dialog_box(
    dialog_box_x: f32,
    dialog_box_y: f32,
    dialog_box_w: f32,
    dialog_box_h: f32,
    ctx: &mut Context,
) -> GameResult {
    let rect = graphics::Rect::new(dialog_box_x, dialog_box_y, dialog_box_w, dialog_box_h);

    let rect_mesh = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        rect,
        graphics::Color::from_rgb(247, 241, 227),
    )?;

    graphics::draw(ctx, &rect_mesh, DrawParam::default())?;
    Ok(())
}

fn draw_dialog_box_content(
    dialog_box_x: f32,
    dialog_box_y: f32,
    npc_component: &Npc,
    interaction: &Interaction,
    ctx: &mut Context,
) -> GameResult {
    let name_text = create_default_text(npc_component.name.clone());

    let mut coords = [dialog_box_x, dialog_box_y];
    let mut params = draw_params_from_coords(coords);

    graphics::draw(ctx, &name_text, params)?;

    let interaction_text = create_default_text(interaction.dialog.to_string());

    coords = [dialog_box_x + 10.0, dialog_box_y + 20.0];
    params = draw_params_from_coords(coords);

    graphics::draw(ctx, &interaction_text, params)?;

    match &interaction.options {
        Some(_) => draw_dialog_options(dialog_box_x, dialog_box_y, interaction, ctx)?,
        None => (),
    }

    Ok(())
}

fn draw_greeting(dialog_box_x: f32, dialog_box_y: f32, ctx: &mut Context) -> GameResult {
    let coords = [dialog_box_x + 10.0, dialog_box_y + 20.0];
    let params = draw_params_from_coords(coords);
    let greeting_text = create_default_text("Hi Julián! What can I help you with? ".to_string());

    graphics::draw(ctx, &greeting_text, params)?;
    Ok(())
}

fn draw_dialog_options(
    dialog_box_x: f32,
    dialog_box_y: f32,
    interaction: &Interaction,
    ctx: &mut Context,
) -> GameResult {
    let mut coords = [dialog_box_x + 10.0, dialog_box_y + 30.0];
    let mut params;

    for (index, option) in interaction.options.as_ref().unwrap().iter().enumerate() {
        let option_text;
        if interaction.hovered_option == index {
            option_text = create_default_text(format!("> {}", option));
        } else {
            option_text = create_default_text(option.into());
        }
        coords = [coords[0] + 10f32, coords[1] + 20f32];
        params = draw_params_from_coords(coords);
        graphics::draw(ctx, &option_text, params)?;
    }
    Ok(())
}

fn draw_params_from_coords(coords: [f32; 2]) -> graphics::DrawParam {
    graphics::DrawParam::default().dest(coords)
}

fn create_default_text(text: String) -> graphics::Text {
    graphics::Text::new(TextFragment {
        text,
        color: Some(Color::BLACK),
        font: Some(graphics::Font::default()),
        ..Default::default()
    })
}
