use super::super::super::utils::constants::NPC_COUNT;
use super::super::{
    input_system::interaction::*, physics_system::physics::*, render_system::camera::Camera,
};
use crate::ecs::{
    components::npc::Npc,
    game_state::EntityIndex,
    sprites::{
        draw::{Draw, DrawComponent},
        npc_sprite::NpcSprite,
        tile_sprite::TileSprite,
    },
};
use ggez::{
    self,
    graphics::{
        spritebatch::SpriteBatch, Color, DrawMode, DrawParam, Rect, StrokeOptions, TextFragment,
    },
    Context, GameResult, *,
};

pub fn draw_tiles(
    ctx: &mut Context,
    camera: &Camera,
    tiles: &mut Vec<Box<TileSprite>>,
    world_sprite_batch: &mut SpriteBatch,
    draw_param: graphics::DrawParam,
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

pub fn draw_npcs(
    ctx: &mut Context,
    camera: &Camera,
    physics_components: &Vec<Option<Physics>>,
    npcs_components: &Vec<Option<Npc>>,
    npcs_sprite_batch: &mut SpriteBatch,
    npcs_sprite: &mut NpcSprite,
    draw_param: graphics::DrawParam,
) -> GameResult {
    for npc_count in 0..NPC_COUNT {
        npcs_sprite.draw(
            npcs_sprite_batch,
            camera,
            &physics_components[npc_count as usize].unwrap(),
            &npcs_components[npc_count as usize].as_ref().unwrap(),
        );
    }

    graphics::draw(ctx, npcs_sprite_batch, draw_param)?;
    npcs_sprite_batch.clear();

    Ok(())
}

pub fn draw_sprite<T: Draw>(
    ctx: &mut Context,
    camera: &Camera,
    physics: &Physics,
    sprite_batch: &mut SpriteBatch,
    sprite: &mut T,
    frames: usize,
    draw_param: graphics::DrawParam,
) -> GameResult {
    sprite.draw(sprite_batch, camera, physics, frames);

    graphics::draw(ctx, sprite_batch, draw_param)?;
    sprite_batch.clear();

    Ok(())
}

pub fn draw_sprite_component<T: DrawComponent<Component = E>, E>(
    ctx: &mut Context,
    camera: &Camera,
    physics: &Physics,
    sprite_batch: &mut SpriteBatch,
    sprite: &mut T,
    frames: usize,
    draw_param: graphics::DrawParam,
    component: &E,
) -> GameResult {
    sprite.draw_component(sprite_batch, camera, physics, frames, component);

    graphics::draw(ctx, sprite_batch, draw_param)?;
    sprite_batch.clear();

    Ok(())
}

pub fn draw_interactions(
    ctx: &mut Context,
    camera_size: &Size,
    npcs_components: &Vec<Option<Npc>>,
    current_interaction: &Option<Interaction>,
    interacting_with: &Option<EntityIndex>,
) -> GameResult {
    match current_interaction {
        Some(interaction) => {
            draw_interaction(
                ctx,
                camera_size,
                npcs_components[interacting_with.unwrap()].as_ref().unwrap(),
                &interaction,
            )?;
        }
        None => (),
    }

    Ok(())
}

fn draw_interaction(
    ctx: &mut Context,
    camera_size: &Size,
    npc_component: &Npc,
    interaction: &Interaction,
) -> GameResult {
    let dialog_box = graphics::Image::new(ctx, "/dialog_box.png")?;

    let dialog_box_x = camera_size.w_half() - (dialog_box.dimensions().w / 2.0);

    let dialog_box_y =
        camera_size.height - dialog_box.dimensions().h - 100.0 + (dialog_box.dimensions().h / 2.0);

    let mut draw_params = graphics::DrawParam::default()
        .dest([dialog_box_x, dialog_box_y])
        .scale([1.0, 1.0]);

    graphics::draw(ctx, &dialog_box, draw_params)?;

    let avatar_box = graphics::Image::new(ctx, "/avatar_box.png")?;

    let avatar_box_x =
        camera_size.w_half() - (dialog_box.dimensions().w / 2.0) + avatar_box.dimensions().x / 2.0;

    draw_params = graphics::DrawParam::default()
        .dest([avatar_box_x, dialog_box_y])
        .scale([1.0, 1.0]);

    graphics::draw(ctx, &avatar_box, draw_params)?;

    let name_text = create_default_text(npc_component.name.clone());

    let coords = [
        avatar_box_x + avatar_box.dimensions().w / 2.0 - name_text.dimensions(ctx).w / 2.0,
        dialog_box_y + dialog_box.dimensions().h - dialog_box.dimensions().h / 4.0,
    ];
    draw_params = draw_params_from_coords(coords);

    graphics::draw(ctx, &name_text, draw_params)?;
    let avatar = graphics::Image::new(ctx, format!("/{:}.png", npc_component.name))?;

    let avatar_coords = [
        avatar_box_x + avatar_box.dimensions().w / 2.0 - (avatar.dimensions().w * 0.5) / 2.0,
        dialog_box_y + dialog_box.dimensions().h / 2.0 - 100.0,
    ];

    draw_params = graphics::DrawParam::default()
        .dest(avatar_coords)
        .scale([0.5, 0.5]);

    graphics::draw(ctx, &avatar, draw_params)?;

    let speech_box_dimensions = Rect {
        x: dialog_box_x + avatar_box.dimensions().w,
        y: dialog_box_y,
        w: dialog_box.dimensions().w - avatar_box.dimensions().w,
        h: dialog_box.dimensions().h,
    };

    draw_dialog_box_content(ctx, speech_box_dimensions, interaction)?;

    Ok(())
}

fn draw_dialog_box_content(
    ctx: &mut Context,
    speech_box_dimensions: graphics::Rect,
    interaction: &Interaction,
) -> GameResult {
    let interaction_text = create_default_npc_dialog_text(interaction.dialog.to_string());

    let coords = [
        speech_box_dimensions.x + 10.0,
        speech_box_dimensions.y + 20.0,
    ];
    let params = draw_params_from_coords(coords);

    graphics::draw(ctx, &interaction_text, params)?;

    let (a, b) = (
        mint::Point2 {
            x: speech_box_dimensions.x + 20.0,
            y: speech_box_dimensions.y + 40.0,
        },
        mint::Point2 {
            x: speech_box_dimensions.x + 170.0,
            y: speech_box_dimensions.y + 40.0,
        },
    );
    let line =
        graphics::Mesh::new_line(ctx, &[a, b], 1.0, graphics::Color::from_rgb(210, 218, 226))?;

    graphics::draw(ctx, &line, graphics::DrawParam::default())?;

    match &interaction.options {
        Some(_) => draw_dialog_options(
            speech_box_dimensions.x,
            speech_box_dimensions.y,
            interaction,
            ctx,
        )?,
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
        color: Some(Color::WHITE),
        font: Some(graphics::Font::default()),
        ..Default::default()
    })
}

fn create_default_npc_dialog_text(text: String) -> graphics::Text {
    graphics::Text::new(TextFragment {
        text,
        color: Some(Color::WHITE),
        font: Some(graphics::Font::default()),
        ..Default::default()
    })
}
