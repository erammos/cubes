use aurion::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Rotation {
    rotation: f32,
}

impl Rotation {
    pub fn new() -> Option<MonoBehavior> {
        Some(Rc::new(RefCell::new(Rotation { rotation: 0.0 })))
    }
}
impl Behavior for Rotation {
    fn awake(&mut self, renderer: &Graphics) {
        self.rotation = 0.0;
    }

    fn update(&mut self, this: GameObject, delta_time: f32) {
        println!("{}", this.get_name());
        self.rotation += delta_time;
        let (scale, rotation, translation) =
            this.get_local_transform().to_scale_rotation_translation();
        let rotation = Quat::from_rotation_y(self.rotation);
        this.set_local_transform(Mat4::from_scale_rotation_translation(
            scale,
            rotation,
            translation,
        ));
    }
}
