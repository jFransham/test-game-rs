use ::colors::{CORNFLOWER_BLUE, RED, Color};
use ::renderable::*;
use ::physics::*;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;
use gameobject::*;

pub trait Application {
    fn update(&mut self, args: &UpdateArgs);
    fn render(&self, opengl: &mut GlGraphics, args: &RenderArgs);
}

pub struct Camera<T: Actor> {
    actor: T,
}

impl<T: Actor> Camera<T> {
    fn new(actor: T) -> Camera<T> {
        Camera { actor: actor }
    }

    fn transform(&self, base: Matrix2d) -> Matrix2d {
        use graphics::math::*;
        multiply(base, self.actor.transform())
    }

    fn update(&mut self, args: &UpdateArgs) {
        let body = Body {
            definition: BodyDef::NoCollide,
            velocity: [0.0; 2],
            position: [0.0; 2],
            rotation: 0.0,
        };

        self.actor.update(&body, args);
    }
}

pub struct Game<'a> {
    pub container: Container<'a>,
    pub camera: Camera<DoNothingActor>,
    pub world: World,
}

impl<'a> Game<'a> {
    pub fn new<'b>() -> Game<'b> {
        use graphics::*;

        let mut output: Game = Game {
                container: Container::new(),
                world: World { 
                    gravity: Gravity::None,
                },
                camera: Camera::new(DoNothingActor),
            };

        output.container.add(GameObject {
            actor: DoNothingActor,
            body: Body::new(BodyDef::NoCollide),
            visible: Renderable::new(
                    RenderableShape::Rectangle(rectangle::square(0.0, 0.0, 50.0), RED),
                    [-25.0, -25.0]
                ),
            });

        output
    }
}

impl<'a> Application for Game<'a> {
    fn update(&mut self, args: &UpdateArgs) {
        self.container.update(args);
        self.camera.update(args);
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

            self.container.render(gl, &trans);

         //   rectangle(RED.f32_4(), rectangle::square(0.0, 0.0, 50.0), trans, gl);
        });
    }
}

pub struct DoNothingActor;
impl Actor for DoNothingActor {
    fn update(&mut self, _: &Body, _: &UpdateArgs) { }
    fn transform(&self) -> Matrix2d { use graphics::math; math::identity() }
}

pub struct SpinActor {
    rotation: f64,
    speed: f64,
}

impl SpinActor {
    fn new(speed: f64) -> SpinActor {
        SpinActor {
            rotation: 0.0,
            speed: speed,
        }
    }
}

impl Actor for SpinActor {
    fn update(&mut self, _: &Body, args: &UpdateArgs) { self.rotation += 1.0 * args.dt; }
    fn transform(&self) -> Matrix2d { use graphics::math; math::rotate_radians(self.rotation) }
}
