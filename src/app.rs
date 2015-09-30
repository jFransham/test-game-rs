use ::colors::{ CORNFLOWER_BLUE, RED, Color };
use ::renderable::*;
use piston::input::{ RenderArgs, UpdateArgs };
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;

pub trait Application {
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, opengl: &mut GlGraphics, args: &RenderArgs);
}

struct Camera {
    x:        f64,
    y:        f64,
    rotation: f64, // radians
}

impl Camera {
    fn new() -> Camera { Camera { x: 0.0, y: 0.0, rotation: 0.0 } }

    fn transform(&self, base: Matrix2d) -> Matrix2d {
        use graphics::*;

        return base.trans(-self.x, -self.y)
                   .rot_rad(-self.rotation);
    }
}

pub struct Game<'a> {
    world:  World<'a>,
    camera: Camera,
}

impl<'a> Game<'a> {
    pub fn new<'b>() -> Game<'b> {
        use graphics::*;

        let mut w = World::new();
        w.add(GameObject {
            actor: SpinActor::new(),
            visible: Renderable::new(
                    RenderableShape::Rectangle(rectangle::square(0.0, 0.0, 50.0), RED),
                    [-25.0, -25.0]
                ),
            });
        Game {
            world:  w,
            camera: Camera::new()
        }
    }
}

impl<'a> Application for Game<'a> {
    fn update(&mut self, args: &UpdateArgs) {
        self.world.update(args);
    }

    fn render(&self, opengl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (offset_x, offset_y) =
            ( args.width  as f64 / 2.0,
              args.height as f64 / 2.0
            );

        let ref cam = self.camera;
        opengl.draw(args.viewport(), |context, gl| {
            clear(CORNFLOWER_BLUE.f32_4(), gl);

            let trans = cam.transform(context.transform)
                           .trans(offset_x, offset_y);

            self.world.render(gl, &trans);

         //   rectangle(RED.f32_4(), rectangle::square(0.0, 0.0, 50.0), trans, gl);
        });
    }
}

struct World<'a> {
    objects: Vec<Box<Object + 'a>>,
}

impl<'a> World<'a> {
    fn new<'b>() -> World<'b> { World { objects: vec![] } }
    fn add<T: Object + 'a>(&mut self, obj: T) { self.objects.push(Box::new(obj)); }
}

impl<'a> Object for World<'a> {
    fn update(&mut self, args: &UpdateArgs) {
        for obj in self.objects.iter_mut() { obj.update(args); }
    }

    fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d) {
        for obj in self.objects.iter() { obj.render(opengl, transform); }
    }
}

struct DoNothingActor;
impl Actor for DoNothingActor {
    fn update(&mut self, _: &UpdateArgs) { }
    fn transform(&self) -> Matrix2d { use graphics::math; math::identity() }
}

struct SpinActor { rotation: f64 }
impl SpinActor { fn new() -> SpinActor { SpinActor { rotation: 0.0 } } }
impl Actor for SpinActor {
    fn update(&mut self, args: &UpdateArgs) { self.rotation += 1.0 * args.dt; }
    fn transform(&self) -> Matrix2d { use graphics::math; math::rotate_radians(self.rotation) }
}

struct GameObject<'a, T: Actor, U: Color> {
    actor: T,
    visible: Renderable<'a, U>,
}

impl<'a, A: Actor, B: Color> Object for GameObject<'a, A, B> {
    fn update(&mut self, args: &UpdateArgs) {
        self.actor.update(args);
    }

    fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d) {
        use graphics::math;
        self.visible.render(opengl, &math::multiply(*transform, self.actor.transform()));
    }
}

trait Object {
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, opengl: &mut GlGraphics, transform: &Matrix2d);
}

trait Actor {
    fn update(&mut self, args: &UpdateArgs);
    fn transform(&self) -> Matrix2d;
}
