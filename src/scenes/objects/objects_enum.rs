use enum_dispatch::enum_dispatch;

use super::bullet::Bullet;
use super::test::TestObj;
use crate::scenes::object::IDObject;

#[enum_dispatch(IDObject)]
pub enum Objects {
    Bullet(Bullet),
    Test(TestObj),
}
