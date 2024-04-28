use ic_stable_structures::BTreeMap;

use crate::api::{User, UserId};

use super::{Memory, MEMORY_MANAGER, USERS_MEMORY_ID};

pub type UsersMemory = BTreeMap<UserId, User, Memory>;

pub fn init_users() -> UsersMemory {
    UsersMemory::init(get_users_memory())
}

fn get_users_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEMORY_ID))
}
