extern crate ecs;

use ecs::*;

struct Foo(usize);
impl Component for Foo {}
impl HotComponent for Foo {}

fn main() {
    let mut world = ecs::World::new();
    let new_entity = world.register_entity();
    
    let component = Foo(100);
    world.register_component(new_entity, component);

    let new_entity = world.register_entity();
    println!("new_entity = {}", new_entity);
}