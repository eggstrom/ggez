//! Basic hello world example, drawing
//! to a canvas.

use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyInput;
use ggez::{Context, GameResult};

struct MainState {
    canvas_image: graphics::ScreenImage,
    frames: usize,
    draw_with_canvas: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let resources_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/resources");
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(format!("{resources_dir}/LiberationMono-Regular.ttf"))?,
        );
        let canvas_image = graphics::ScreenImage::new(ctx, None, 1., 1., 1);

        let s = MainState {
            canvas_image,
            draw_with_canvas: false,
            frames: 0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dest_point = Vec2::new(10.0, 10.0);

        let text = graphics::Text::new("Hello, world!")
            .set_font("LiberationMono")
            .set_scale(48.)
            .clone();

        if self.draw_with_canvas {
            println!("Drawing with canvas");
            let canvas_image = self.canvas_image.image(ctx);
            let mut canvas = graphics::Canvas::from_image(
                ctx,
                canvas_image.clone(),
                graphics::Color::from((255, 255, 255, 128)),
            );

            canvas.draw(
                &text,
                graphics::DrawParam::from(dest_point + vec2(15., 15.))
                    .color(Color::from((0, 0, 0, 255))),
            );
            canvas.finish(ctx)?;

            let mut canvas = graphics::Canvas::from_frame(ctx, Color::from((64, 0, 0, 0)));
            canvas.draw(
                &canvas_image,
                graphics::DrawParam::new().color(Color::from((255, 255, 255, 128))),
            );
            canvas.finish(ctx)?;
        } else {
            println!("Drawing without canvas");
            let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.25, 0.0, 0.0, 1.0]));

            canvas.draw(
                &text,
                graphics::DrawParam::from(dest_point).color(Color::from((192, 128, 64, 255))),
            );

            canvas.finish(ctx)?;
        }

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, _input: KeyInput, repeat: bool) -> GameResult {
        if !repeat {
            self.draw_with_canvas = !self.draw_with_canvas;
            println!("Canvas on: {}", self.draw_with_canvas);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new();
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
