#![feature(const_fn)]
#![feature(const_type_id)]

mod bitset;
mod component;
mod entity;
mod errors;
mod system;
mod world;

pub use entity::Entity;
pub use component::*; 
pub use system::System;
pub use world::World;

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(Debug)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Acceleration {
        x: f32,
        y: f32,
    }

    impl Component for Position {}
    impl Component for Velocity {}
    impl Component for Acceleration {}

    struct MovementSystem {}

    impl System for MovementSystem {
        fn needed_components() -> &'static [ComponentId] {
            static COMPS: [ComponentId; 3] = [
                component_id::<Position>(), 
                component_id::<Velocity>(),
                component_id::<Acceleration>()
            ];
            &COMPS
        }

        fn run(mut comps: ComponentSet) {
            let mut positions = comps.get_components::<Position>();
            let mut velocities = comps.get_components::<Velocity>();
            let mut accelerations = comps.get_components::<Acceleration>();

            let pva_iter = positions.iter_mut()
                .zip(velocities.iter_mut())
                .zip(accelerations.iter_mut());
            
            for ((pos, vel), acl) in pva_iter {
                let pos = pos.as_any_mut().downcast_mut::<Position>().unwrap();
                let vel = vel.as_any_mut().downcast_mut::<Velocity>().unwrap();
                let acl = acl.as_any_mut().downcast_mut::<Acceleration>().unwrap();
                pos.x += vel.x;
                pos.y += vel.y;
                vel.x += acl.x;
                vel.y += acl.y;
            }
        }
    }

    #[test]
    fn it_works() {
        let mut world = World::new();
        let entity = world.register_entity();
        
        world.add_component(&entity, Position {
            x: 1.0,
            y: 1.5,
        });

        world.add_component(&entity, Velocity {
            x: 2.7,
            y: 2.0,
        });

        world.add_component(&entity, Acceleration {
            x: -1.0,
            y: -1.0,
        });

        for i in 0..10 {
            println!(
                "t={}, pos={:?}",
                i, 
                world.get_component::<Position>(&entity)
            );
            world.run_with::<MovementSystem>();
        }
    }
}
