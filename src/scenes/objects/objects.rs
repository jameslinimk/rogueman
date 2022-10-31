use enum_dispatch::enum_dispatch;

use super::bullet::Bullet;
use crate::scenes::object::IDObject;

#[enum_dispatch(IDObject)]
pub(crate) enum Objects {
    Bullet(Bullet),
}
