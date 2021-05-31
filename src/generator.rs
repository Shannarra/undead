use crate::entities::{self, Entity};

pub fn generate_all(zombie_code: String) {
    let depth :u32 = 0;
    let next_summon = false;
    let mut entity: &dyn Entity;

    let zombie = entities::zombie::Zombie::new(&"test");
    entity = &zombie;

    println!("{:?}", entity.name())
}