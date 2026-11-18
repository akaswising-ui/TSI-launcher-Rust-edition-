mod world;

use macroquad::prelude::*;
use world::{BlockKind, World, WORLD_SIZE};

const EYE_HEIGHT: f32 = 1.62;

struct Player {
    pos: Vec3,
    velocity_y: f32,
    yaw: f32,
    pitch: f32,
    grounded: bool,
}

impl Player {
    fn spawn(world: &World) -> Self {
        let x = (WORLD_SIZE / 2) as i32;
        let z = (WORLD_SIZE / 2) as i32;
        Self {
            pos: vec3(x as f32 + 0.5, world.height_at(x, z).unwrap_or(5) as f32, z as f32 + 0.5),
            velocity_y: 0.0,
            yaw: PI * 0.5,
            pitch: -0.08,
            grounded: true,
        }
    }
    fn direction(&self) -> Vec3 {
        vec3(self.yaw.cos() * self.pitch.cos(), self.pitch.sin(), self.yaw.sin() * self.pitch.cos()).normalize()
    }
    fn eye(&self) -> Vec3 { self.pos + vec3(0.0, EYE_HEIGHT, 0.0) }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "TSI Rust Edition".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

fn color_of(kind: BlockKind, x: i32, z: i32) -> Color {
    let v = ((x.wrapping_mul(17) ^ z.wrapping_mul(31)) & 7) as f32 * 0.012;
    match kind {
        BlockKind::Grass => Color::new(0.20 + v, 0.58 + v, 0.22, 1.0),
        BlockKind::Dirt => Color::new(0.42 + v, 0.27, 0.14, 1.0),
        BlockKind::Stone => Color::new(0.43 + v, 0.46 + v, 0.49, 1.0),
        BlockKind::Sand => Color::new(0.78, 0.68 + v, 0.40, 1.0),
        BlockKind::Wood => Color::new(0.39, 0.24, 0.12, 1.0),
    }
}

fn raycast(world: &World, origin: Vec3, direction: Vec3) -> Option<(i32, i32)> {
    for step in 2..76 {
        let point = origin + direction * (step as f32 * 0.08);
        let x = point.x.floor() as i32;
        let z = point.z.floor() as i32;
        let Some(height) = world.height_at(x, z) else { continue; };
        if point.y >= 0.0 && point.y < height as f32 { return Some((x, z)); }
    }
    None
}

fn draw_block_world(world: &World, player: &Player) {
    let px = player.pos.x.floor() as i32;
    let pz = player.pos.z.floor() as i32;
    let radius = 16;
    for z in (pz - radius).max(0)..=(pz + radius).min(WORLD_SIZE as i32 - 1) {
        for x in (px - radius).max(0)..=(px + radius).min(WORLD_SIZE as i32 - 1) {
            let h = world.height_at(x, z).unwrap_or(1) as f32;
            let top = world.top_at(x, z).unwrap_or(BlockKind::Grass);
            let base_h = (h - 1.0).max(1.0);
            let base = if top == BlockKind::Sand { Color::new(0.56, 0.45, 0.27, 1.0) } else { color_of(BlockKind::Dirt, x, z) };
            draw_cube(vec3(x as f32 + 0.5, base_h * 0.5, z as f32 + 0.5), vec3(1.0, base_h, 1.0), None, base);
            draw_cube(vec3(x as f32 + 0.5, h - 0.5, z as f32 + 0.5), vec3(1.0, 1.0, 1.0), None, color_of(top, x, z));

            if world.has_tree(x, z) {
                for level in 0..3 {
                    draw_cube(vec3(x as f32 + 0.5, h + 0.5 + level as f32, z as f32 + 0.5), vec3(0.34, 1.0, 0.34), None, color_of(BlockKind::Wood, x, z));
                }
                for dz in -1..=1 {
                    for dx in -1..=1 {
                        draw_cube(vec3(x as f32 + 0.5 + dx as f32 * 0.55, h + 3.0, z as f32 + 0.5 + dz as f32 * 0.55), vec3(0.9, 0.9, 0.9), None, Color::new(0.12, 0.42, 0.18, 1.0));
                    }
                }
            }
        }
    }
}

fn button(rect: Rect, label: &str) -> bool {
    let (mx, my) = mouse_position();
    let inside = rect.contains(vec2(mx, my));
    let down = inside && is_mouse_button_down(MouseButton::Left);
    let fill = if down { Color::new(0.08, 0.48, 0.45, 0.95) } else { Color::new(0.035, 0.075, 0.10, 0.78) };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, Color::new(0.35, 0.86, 0.78, 0.9));
    let size = (rect.h * 0.32).clamp(13.0, 22.0);
    let text = measure_text(label, None, size as u16, 1.0);
    draw_text(label, rect.x + (rect.w - text.width) * 0.5, rect.y + (rect.h + text.height) * 0.35, size, WHITE);
    down
}

fn controls(w: f32, h: f32) -> [Rect; 7] {
    [
        Rect::new(18.0, h - 106.0, 48.0, 48.0),
        Rect::new(70.0, h - 158.0, 48.0, 48.0),
        Rect::new(70.0, h - 106.0, 48.0, 48.0),
        Rect::new(122.0, h - 106.0, 48.0, 48.0),
        Rect::new(w - 224.0, h - 110.0, 60.0, 60.0),
        Rect::new(w - 150.0, h - 158.0, 64.0, 64.0),
        Rect::new(w - 82.0, h - 90.0, 64.0, 64.0),
    ]
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new(0x5453_4931);
    let mut player = Player::spawn(&world);
    let mut selected = BlockKind::Dirt;
    let mut toast = "Explore · mine blocks · build upward".to_owned();
    let mut toast_time = 4.0_f32;

    loop {
        let dt = get_frame_time().clamp(0.0, 0.05);
        let w = screen_width();
        let h = screen_height();
        let ui = controls(w, h);
        let (mx, my) = mouse_position();
        let over_ui = ui.iter().any(|rect| rect.contains(vec2(mx, my)));

        if is_mouse_button_down(MouseButton::Left) && !over_ui && mx > 190.0 && my < h - 170.0 {
            let delta = mouse_delta_position();
            player.yaw += delta.x * 2.8;
            player.pitch = (player.pitch - delta.y * 2.4).clamp(-1.15, 1.15);
        }
        if is_key_down(KeyCode::Left) { player.yaw -= 1.8 * dt; }
        if is_key_down(KeyCode::Right) { player.yaw += 1.8 * dt; }
        if is_key_down(KeyCode::Up) { player.pitch = (player.pitch + 1.2 * dt).clamp(-1.15, 1.15); }
        if is_key_down(KeyCode::Down) { player.pitch = (player.pitch - 1.2 * dt).clamp(-1.15, 1.15); }

        let look = player.direction();
        let mut forward = vec3(look.x, 0.0, look.z);
        if forward.length() > 0.001 { forward = forward.normalize(); }
        let right = vec3(-forward.z, 0.0, forward.x);
        let mut movement = vec3(0.0, 0.0, 0.0);
        if is_key_down(KeyCode::W) || (is_mouse_button_down(MouseButton::Left) && ui[1].contains(vec2(mx, my))) { movement += forward; }
        if is_key_down(KeyCode::S) || (is_mouse_button_down(MouseButton::Left) && ui[2].contains(vec2(mx, my))) { movement -= forward; }
        if is_key_down(KeyCode::A) || (is_mouse_button_down(MouseButton::Left) && ui[0].contains(vec2(mx, my))) { movement -= right; }
        if is_key_down(KeyCode::D) || (is_mouse_button_down(MouseButton::Left) && ui[3].contains(vec2(mx, my))) { movement += right; }

        if movement.length() > 0.001 {
            movement = movement.normalize();
            let speed = if is_key_down(KeyCode::LeftShift) { 7.0 } else { 4.4 };
            let current_floor = world.height_at(player.pos.x.floor() as i32, player.pos.z.floor() as i32).unwrap_or(1) as f32;
            let next_x = player.pos.x + movement.x * speed * dt;
            let next_z = player.pos.z + movement.z * speed * dt;
            if world.height_at(next_x.floor() as i32, player.pos.z.floor() as i32).is_some_and(|v| v as f32 <= current_floor + 1.0) {
                player.pos.x = next_x.clamp(1.0, WORLD_SIZE as f32 - 1.0);
            }
            if world.height_at(player.pos.x.floor() as i32, next_z.floor() as i32).is_some_and(|v| v as f32 <= current_floor + 1.0) {
                player.pos.z = next_z.clamp(1.0, WORLD_SIZE as f32 - 1.0);
            }
        }

        let jump = is_key_pressed(KeyCode::Space) || (is_mouse_button_pressed(MouseButton::Left) && ui[4].contains(vec2(mx, my)));
        if jump && player.grounded { player.velocity_y = 7.6; player.grounded = false; }
        player.velocity_y -= 21.0 * dt;
        player.pos.y += player.velocity_y * dt;
        let floor = world.height_at(player.pos.x.floor() as i32, player.pos.z.floor() as i32).unwrap_or(1) as f32;
        if player.pos.y <= floor { player.pos.y = floor; player.velocity_y = 0.0; player.grounded = true; }

        if is_key_pressed(KeyCode::Key1) { selected = BlockKind::Grass; }
        if is_key_pressed(KeyCode::Key2) { selected = BlockKind::Dirt; }
        if is_key_pressed(KeyCode::Key3) { selected = BlockKind::Stone; }
        if is_key_pressed(KeyCode::Key4) { selected = BlockKind::Sand; }
        if is_key_pressed(KeyCode::Key5) { selected = BlockKind::Wood; }

        let target = raycast(&world, player.eye(), player.direction());
        let mine = (is_mouse_button_pressed(MouseButton::Left) && !over_ui)
            || (is_mouse_button_pressed(MouseButton::Left) && ui[5].contains(vec2(mx, my)));
        let place = is_mouse_button_pressed(MouseButton::Right)
            || (is_mouse_button_pressed(MouseButton::Left) && ui[6].contains(vec2(mx, my)));
        if mine {
            if let Some((x, z)) = target {
                if world.mine_top(x, z) { toast = "Block mined".to_owned(); toast_time = 1.5; }
            }
        }
        if place {
            if let Some((x, z)) = target {
                if world.place_top(x, z, selected) { toast = "Block placed".to_owned(); toast_time = 1.5; }
            }
        }

        let eye = player.eye();
        clear_background(Color::new(0.48, 0.72, 0.92, 1.0));
        set_camera(&Camera3D {
            position: eye,
            target: eye + player.direction(),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 68.0,
            ..Default::default()
        });
        draw_block_world(&world, &player);
        if let Some((x, z)) = target {
            let y = world.height_at(x, z).unwrap_or(1) as f32 - 0.5;
            draw_cube_wires(vec3(x as f32 + 0.5, y, z as f32 + 0.5), vec3(1.04, 1.04, 1.04), YELLOW);
        }

        set_default_camera();
        draw_rectangle(0.0, 0.0, w, 42.0, Color::new(0.02, 0.06, 0.09, 0.76));
        draw_text("TSI RUST EDITION", 18.0, 27.0, 22.0, Color::new(0.40, 0.94, 0.82, 1.0));
        draw_text(&format!("FPS {}  |  X {:.1}  Z {:.1}", get_fps(), player.pos.x, player.pos.z), 245.0, 25.0, 17.0, WHITE);
        draw_line(w * 0.5 - 9.0, h * 0.5, w * 0.5 + 9.0, h * 0.5, 2.0, WHITE);
        draw_line(w * 0.5, h * 0.5 - 9.0, w * 0.5, h * 0.5 + 9.0, 2.0, WHITE);

        let hot_x = (w - 270.0) * 0.5;
        for (i, (label, kind)) in [
            ("1", BlockKind::Grass), ("2", BlockKind::Dirt), ("3", BlockKind::Stone),
            ("4", BlockKind::Sand), ("5", BlockKind::Wood),
        ].iter().enumerate() {
            let x = hot_x + i as f32 * 54.0;
            draw_rectangle(x, h - 58.0, 50.0, 48.0, if *kind == selected { Color::new(0.10, 0.37, 0.34, 0.95) } else { Color::new(0.02, 0.05, 0.08, 0.78) });
            draw_rectangle(x + 8.0, h - 48.0, 22.0, 22.0, color_of(*kind, i as i32, 1));
            draw_text(label, x + 34.0, h - 25.0, 15.0, WHITE);
        }
        button(ui[0], "◀");
        button(ui[1], "▲");
        button(ui[2], "▼");
        button(ui[3], "▶");
        button(ui[4], "JUMP");
        button(ui[5], "MINE");
        button(ui[6], "PLACE");

        draw_rectangle(12.0, 50.0, 355.0, 29.0, Color::new(0.02, 0.05, 0.08, 0.66));
        draw_text(&toast, 22.0, 70.0, 16.0, WHITE);
        toast_time -= dt;
        if toast_time <= 0.0 {
            toast = "Drag right side to look · WASD / pad to move".to_owned();
            toast_time = 1000.0;
        }
        if is_key_pressed(KeyCode::Escape) {
            toast = "Close the app to exit".to_owned();
            toast_time = 3.0;
        }

        next_frame().await;
    }
}
