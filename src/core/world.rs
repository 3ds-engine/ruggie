use mlua::{Table, AnyUserData, Function};

pub struct World {
    current_scene: Table,
}
 
impl World {
    pub fn new(scene: Table) -> Self {
        Self { current_scene: scene }
    }
 
    pub fn render(&self, renderer: &AnyUserData) -> mlua::Result<()> {
        let render: Function = self.current_scene.get("render")?;
        render.call(renderer)
    }
}
