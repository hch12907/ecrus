#![feature(const_fn)]
#![feature(const_type_id)]

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

        fn run(comps: &mut [(ComponentId, Vec<&mut dyn Component>)]) {
            // Sigh, borrowck, we are taking one of the unique
            // elements only!
            let comps2 = unsafe { 
                let len = comps.len();
                let ptr = comps.as_mut_ptr();
                std::slice::from_raw_parts_mut(ptr, len)
            };

            let comps3 = unsafe { 
                let len = comps.len();
                let ptr = comps.as_mut_ptr();
                std::slice::from_raw_parts_mut(ptr, len)
            };

            let positions = comps.iter_mut()
                .filter(|(cid, _v)| *cid == component_id::<Position>())
                .nth(0)
                .unwrap();

            let velocities = comps2.iter_mut()
                .filter(|(cid, _v)| *cid == component_id::<Velocity>())
                .nth(0)
                .unwrap();

            let accelerations = comps3.iter_mut()
                .filter(|(cid, _v)| *cid == component_id::<Acceleration>())
                .nth(0)
                .unwrap();

            let pva_iter = positions.1.iter_mut()
                .zip(velocities.1.iter_mut())
                .zip(accelerations.1.iter_mut());
            
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
