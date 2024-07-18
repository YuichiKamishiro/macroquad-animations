use macroquad::prelude::*;
struct SpriteSheet {
    texture: Texture2D,
}

impl SpriteSheet {
    pub fn new() -> Self {
        SpriteSheet {
            texture: Texture2D::empty(),
        }
    }

    pub async fn load(&mut self, path: &str) {
        match load_texture(path).await {
            Ok(texture) => self.texture = texture,
            Err(err) => println!("Error while loading texture: {err}"),
        }
    }
}

struct Animator {
    sprite_sheet: SpriteSheet,
    current_time: f32,
    current_frame: usize,
    rects: Vec<(Rect, f32)>,
}

impl Animator {
    pub fn new() -> Self {
        Animator {
            sprite_sheet: SpriteSheet::new(),
            current_time: 0.,
            current_frame: 0,
            rects: Vec::new(),
        }
    }
    pub async fn load(&mut self, path: &str) {
        self.sprite_sheet.load(path).await
    }
    pub fn add_frames(&mut self, rects: Vec<(Rect, f32)>) {
        self.rects = rects;
    }
    pub fn next_step(&mut self) {
        self.current_time = self.current_time + get_frame_time();

        if self.current_time >= self.rects[self.current_frame].1 {
            self.current_time = self.current_time - self.rects[self.current_frame].1;
            self.current_frame = self.current_frame + 1;
        }

        if self.current_frame == self.rects.len() {
            self.current_frame = 0;
        }
        
    }
    pub fn draw(&self) {
        if self.rects.is_empty() {
            println!("Error frames weren't added, make sure you called add_frames() func");
            return;
        }
        draw_texture_ex(&self.sprite_sheet.texture, 100., 100., WHITE, DrawTextureParams {
            source: Some(self.rects[self.current_frame].0),
            ..Default::default()
        });
    }
}

#[macroquad::main("Hello")]
async fn main() {
    let mut animator = Animator::new();

    animator.load("attack.png").await;

    let frames = vec![
        (Rect::new(0., 0., 120., 80.), 0.5),
        (Rect::new(120., 0., 120., 80.), 0.1),
        (Rect::new(240., 0., 120., 80.), 0.1),
        (Rect::new(360., 0., 120., 80.), 0.3),
    ];

    animator.add_frames(frames);
    
    loop {
        clear_background(BLACK);

        animator.next_step();
        animator.draw();


        next_frame().await
    }
}
