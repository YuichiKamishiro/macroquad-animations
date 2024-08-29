use macroquad::prelude::*;
pub struct Animator {
    texture: Texture2D,
    current_time: f32,
    current_frame: usize,
    rects: Vec<(Rect, f32)>,
}

#[allow(dead_code)]
impl Animator {
    pub fn new() -> Self {
        Animator {
            texture: Texture2D::empty(),
            current_time: 0.,
            current_frame: 0,
            rects: Vec::new(),
        }
    }
    pub async fn load(&mut self, path: &str) {
        match load_texture(path).await {
            Ok(texture) => self.texture = texture,
            Err(err) => println!("Error while loading texture: {err}"),
        }
    }
    pub fn add_frames(&mut self, rects: Vec<(Rect, f32)>) {
        self.rects = rects;
    }
    pub fn update(&mut self) {
        self.current_time = self.current_time + get_frame_time();

        if self.current_time >= self.rects[self.current_frame].1 {
            self.current_time = 0.;
            self.current_frame = self.current_frame + 1;
        }

        if self.current_frame == self.rects.len() {
            self.current_frame = 0;
        }
        
    }
    pub fn draw(&self, x: f32, y: f32) {
        if self.rects.is_empty() {
            println!("Error frames weren't added, make sure you called add_frames() func");
            return;
        }
        draw_texture_ex(&self.texture, x, y, WHITE, DrawTextureParams {
            source: Some(self.rects[self.current_frame].0),
            ..Default::default()
        });
    }

    pub fn draw_scaled(&self, x: f32, y: f32, new_size: Vec2) {
        if self.rects.is_empty() {
            println!("Error frames weren't added, make sure you called add_frames() func");
            return;
        }
        draw_texture_ex(&self.texture, x, y, WHITE, DrawTextureParams {
            source: Some(self.rects[self.current_frame].0),
            dest_size: Some(new_size),
            ..Default::default()
        });
    }
}
