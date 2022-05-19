use shaku::{Component, Interface};
use socket::Rooms as SocketRooms;

pub trait Rooms: Interface {
    fn get(&self) -> &SocketRooms;
}

#[derive(Component)]
#[shaku(interface = Rooms)]
pub struct RoomsImpl {
    #[shaku(default)]
    rooms: SocketRooms,
}

impl Rooms for RoomsImpl {
    fn get(&self) -> &SocketRooms {
        &self.rooms
    }
}
