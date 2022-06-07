use super::super::{
    input_system::interaction::*, physics_system::physics::*, render_system::camera::Camera,
};
use crate::ecs::{
    components::npc::Npc,
    game_state::EntityIndex,
    sprites::{player_sprite::PlayerSprite, tile_sprite::TileSprite},
};
use ggez::{
    self,
    graphics::{spritebatch::SpriteBatch, Color, DrawMode, DrawParam, StrokeOptions, TextFragment},
    Context, GameResult, *,
};

pub fn draw_tiles(
    ctx: &mut Context,
    camera: &Camera,
    tiles: &mut Vec<Box<TileSprite>>,
    world_sprite_batch: &mut SpriteBatch,
    draw_param: graphics::DrawParam
) -> GameResult {
    for i in 0..tiles.len() {
        tiles[i].draw(world_sprite_batch, camera);
    }

    graphics::draw(ctx, world_sprite_batch, draw_param)?;
    world_sprite_batch.clear();

    Ok(())
}

pub fn draw_world_bounds(ctx: &mut Context, camera: &Camera, world_size: &Size) -> GameResult {
    let rectangle_position = Position {
        x: world_size.w_half(),
        y: world_size.h_half(),
    };

    let position_in_camera: Position = camera.world_to_screen(&rectangle_position);

    let rect = graphics::Rect::new(
        position_in_camera.x - world_size.w_half(),
        position_in_camera.y - world_size.h_half(),
        world_size.width,
        world_size.height,
    );

    let stroke_options = StrokeOptions::default();
    let drawing_mode = DrawMode::Stroke(stroke_options);
    let rect_mesh = graphics::Mesh::new_rectangle(ctx, drawing_mode, rect, Color::BLACK)?;

    graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

    Ok(())
}

pub fn draw_player(
    ctx: &mut Context,
    camera: &Camera,
    player_physics: &Physics,
    player_sprite_batch: &mut SpriteBatch,
    player_sprite: &mut PlayerSprite,
    frames: usize,
    draw_param: graphics::DrawParam
) -> GameResult {
    player_sprite.draw(player_sprite_batch, camera, player_physics, frames);
    
    graphics::draw(ctx, player_sprite_batch, draw_param)?;
    player_sprite_batch.clear();

    Ok(())
}

pub fn draw_objects(ctx: &mut Context, camera: &Camera, physics_components: &Vec<Option<Physics>>) -> GameResult {
    for object in physics_components {
        match object {
            Some(physics) => draw_object(ctx, &physics, camera)?,
            None => (),
        }
    }

    Ok(())
}

fn draw_object(ctx: &mut Context, physics: &Physics, camera: &Camera) -> GameResult {
    let position_in_camera: Position = camera.world_to_screen(&physics.position);
    let rect = graphics::Rect::new(
        position_in_camera.x - physics.size.w_half(),
        position_in_camera.y - physics.size.h_half(),
        physics.size.width,
        physics.size.height,
    );

    let rect_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, physics.color)?;

    graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

    Ok(())
}

// -------------------------------------------------------------------
// Will be removed/updated

pub fn draw_interactions(
    ctx: &mut Context,
    physics_components: &Vec<Option<Physics>>,
    npcs_components: &Vec<Option<Npc>>,
    current_interaction: &Option<Interaction>,
    interacting_with: &Option<EntityIndex>
) -> GameResult {
    match current_interaction {
        Some(interaction) => {
            draw_interaction(
                npcs_components[interacting_with.unwrap()].as_ref().unwrap(),
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

fn draw_interaction(
    npc_component: &Npc,
    npc_physics: &Physics,
    interaction: &Interaction,
    ctx: &mut Context,
) -> GameResult {
    let dialog_box_w = npc_physics.size.width * 15.0;
    let dialog_box_h = npc_physics.size.height * 5.0;

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

    let interaction_text = create_default_npc_dialog_text(interaction.dialog.to_string());

    coords = [dialog_box_x + 10.0, dialog_box_y + 20.0];
    params = draw_params_from_coords(coords);

    graphics::draw(ctx, &interaction_text, params)?;

    let (a, b) = (
        mint::Point2 {
            x: dialog_box_x + 20.0,
            y: dialog_box_y + 40.0,
        },
        mint::Point2 {
            x: dialog_box_x + 170.0,
            y: dialog_box_y + 40.0,
        },
    );
    let line =
        graphics::Mesh::new_line(ctx, &[a, b], 1.0, graphics::Color::from_rgb(210, 218, 226))?;

    graphics::draw(ctx, &line, graphics::DrawParam::default())?;

    match &interaction.options {
        Some(_) => draw_dialog_options(dialog_box_x, dialog_box_y, interaction, ctx)?,
        None => (),
    }

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
        color: Some(Color::from_rgb(30, 39, 46)),
        font: Some(graphics::Font::default()),
        ..Default::default()
    })
}

fn create_default_npc_dialog_text(text: String) -> graphics::Text {
    graphics::Text::new(TextFragment {
        text,
        color: Some(Color::from_rgb(87, 75, 144)),
        font: Some(graphics::Font::default()),
        ..Default::default()
    })
}
