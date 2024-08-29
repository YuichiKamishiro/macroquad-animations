mod animations;

use crate::animations as anim;
use macroquad::prelude::*;

#[macroquad::main("Hello")]
async fn main() {
    let mut animator = anim::Animator::new();

    animator.load("attack.png").await;

    let frames = vec![
        (Rect::new(0., 0., 120., 80.), 0.2),
        (Rect::new(120., 0., 120., 80.), 0.1),
        (Rect::new(240., 0., 120., 80.), 0.1),
        (Rect::new(360., 0., 120., 80.), 0.3),
    ];

    animator.add_frames(frames);
    
    loop {
        clear_background(BLACK);

        animator.update();
        animator.draw_scaled(0., 0., Vec2::new(240.0, 160.0));

        next_frame().await
    }
}
