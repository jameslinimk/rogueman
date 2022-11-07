use enum_dispatch::enum_dispatch;

pub trait Object {
    fn update(&mut self);
    fn draw(&mut self);
}

#[enum_dispatch]
pub trait IDObject {
    fn update(&mut self);
    fn draw(&mut self);
    fn get_id(&self) -> u32;
}

static mut _OBJ_ID: u32 = 0;
pub fn obj_id() -> u32 {
    unsafe {
        _OBJ_ID += 1;
        _OBJ_ID
    }
}
