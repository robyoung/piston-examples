
#![feature(globs)]

extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::rc::Rc;

use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
    Input,
};
use piston::sprite::*;
use piston::event::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

use graphics::*;

use sdl2_game_window::WindowSDL2;
use opengl_graphics::{
    Gl,
    Texture,
};

fn main() {
    let (width, height) = (300, 300);
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Sprite".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let asset_store = AssetStore::from_folder("../");
    let mut scene = Scene::new();

    let tex = Rc::new(Texture::from_path(&asset_store.path("rust-logo.png").unwrap()).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    let id = scene.add_child(sprite);

    // Run a sequence or animations.
    let seq = Sequence(vec![
        Action(Ease(EaseCubicOut, box ScaleTo(2.0, 0.5, 0.5))),
        Action(Ease(EaseBounceOut, box MoveBy(1.0, 0.0, 100.0))),
        Action(Ease(EaseElasticOut, box MoveBy(2.0, 0.0, -100.0))),
        Action(Ease(EaseBackInOut, box MoveBy(1.0, 0.0, -100.0))),
        Wait(0.5),
        Action(Ease(EaseExponentialInOut, box MoveBy(1.0, 0.0, 100.0))),
        Action(Blink(1.0, 5)),
        While(box WaitForever, vec![
            Action(Ease(EaseQuadraticIn, box FadeOut(1.0))),
            Action(Ease(EaseQuadraticOut, box FadeIn(1.0))),
        ]),
    ]);
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseExponentialInOut, box RotateTo(2.0, 360.0)));
    scene.run(id, &rotate);

    println!("Press any key to pause/resume the animation!");

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let ref mut gl = Gl::new(opengl);
    for e in EventIterator::new(&mut window, &event_settings) {
        scene.event(&e);

        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.width as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);

                scene.draw(&c, gl);
            },
            Input(piston::input::Press(_)) => {
                scene.toggle(id, &seq);
                scene.toggle(id, &rotate);
            },
            _ => {},
        }
    }
}
