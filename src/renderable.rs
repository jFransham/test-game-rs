use ::colors::Color;
use std::ops::Deref;
use graphics::*;
use graphics::math::*;
use opengl_graphics::{ Texture, GlGraphics };

pub enum RenderableShape<'a, T: Color> {
    Rectangle(types::Rectangle, T),
    Ellipse(types::Rectangle, T),
    Polygon(types::Polygon<'a>, T),
    Image(&'a Texture),
}

pub struct Renderable<'a, T: Color> {
    shape:  RenderableShape<'a, T>,
    offset: Vec2d,
}

impl<'a, T: Color> Renderable<'a, T> {
    pub fn new(shape: RenderableShape<'a, T>, offset: Vec2d) -> Renderable<'a, T> {
        Renderable {
            shape: shape,
            offset: offset,
        }
    }

    pub fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d) -> () {
        let trans = transform.trans(self.offset[0], self.offset[1]);

        match self.shape {
            RenderableShape::Rectangle(rect, ref color) =>
                rectangle(color.f32_4(), rect, trans, opengl),
            RenderableShape::Ellipse(rect, ref color) =>
                ellipse(color.f32_4(), rect, trans, opengl),
            RenderableShape::Polygon(poly, ref color) =>
                polygon(color.f32_4(), &poly, trans, opengl),
            RenderableShape::Image(tex) =>
                image(tex, trans, opengl),
        }
    }
}
