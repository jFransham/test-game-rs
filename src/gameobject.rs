use ::colors::Color;
use ::renderable::*;
use ::physics::*;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;

pub struct GameObject<'a, T: Actor, U: Color> {
    pub actor: T,
    pub body: Body,
    pub visible: Renderable<'a, U>,
}

impl<'a, A: Actor, B: Color> Object for GameObject<'a, A, B> {
    fn update(&mut self, args: &UpdateArgs) {
        self.actor.update(&self.body, args);
    }

    fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d) {
        use graphics::math;
        self.visible.render(opengl, &math::multiply(*transform, self.actor.transform()));
    }
}

pub trait Object {
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d);
}

pub trait Actor {
    fn update(&mut self, body: &Body, args: &UpdateArgs);
    fn bump<T: Actor>(&mut self, _: &mut T, _: BumpArgs) { }
    fn transform(&self) -> Matrix2d;
}

pub struct BumpArgs;

pub struct Container<'a> {
    objects: Vec<Box<Object + 'a>>,
}

impl<'a> Container<'a> {
    pub fn new<'b>() -> Container<'b> { Container { objects: vec![] } }
    pub fn add<T: Object + 'a>(&mut self, obj: T) { self.objects.push(Box::new(obj)); }
}

impl<'a> Object for Container<'a> {
    fn update(&mut self, args: &UpdateArgs) {
        for obj in self.objects.iter_mut() { obj.update(args); }
    }

    fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d) {
        for obj in self.objects.iter() { obj.render(opengl, transform); }
    }
}

pub trait PhysicsContainer {
    
}
