mod rotation;

use aurion::prelude::event::Event;
use crate::rotation::Rotation;
use aurion::prelude::*;


fn main() {
    let vs = aurion::prelude::read_file("assets/shader.vs");
    let fs = aurion::prelude::read_file("assets/shader.fs");

    let mut app = App::new("my first app", 1920, 1080);
    let mut graphics = Graphics::new(&app);

    let world = GameObjectNode::new("world");
    let mut cube = GameObjectNode::new("cube");
    let cube2 = GameObjectNode::new("cube2");

    GameObjectNode::add_child(&world, &cube);
    GameObjectNode::add_child(&cube, &cube2);
    cube.add_behavior(Rotation::new().unwrap());
    cube.set_local_transform(Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)));
    cube2.set_local_transform(Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)));

    GameObjectNode::init(&world, &mut graphics);
    let mesh = graphics.create_cube();
    let shader = Shader::new(&graphics, &vs, &fs);
    shader.use_program();
    let mut last_time = Instant::now();

    'render: loop {
        for event in app.event_loop().poll_iter() {
            if let Event::Quit { .. } = event {
                break 'render;
            }
        }
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);
        last_time = current_time;
        GameObjectNode::update(&world, delta_time.as_secs_f32());
        graphics.begin_frame();
        graphics.draw_mesh(&cube2.get_world_transform(), &shader, &mesh);
        graphics.draw_mesh(&cube.get_world_transform(), &shader, &mesh);
        graphics.end_frame(&app.window());
    }
}
