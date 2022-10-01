pub(crate) trait Object {
    fn update(&mut self);
    fn draw(&mut self);
}

pub(crate) trait IDObject {
    fn update(&mut self);
    fn draw(&mut self);
    fn get_id(&self) -> u32;
}

static mut _OBJ_ID: u32 = 0;
pub(crate) fn obj_id() -> u32 {
    unsafe {
        _OBJ_ID += 1;
        return _OBJ_ID;
    }
}
