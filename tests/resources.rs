use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod tests {
    use crate::FpsResource;
    use ecs_rs::World;
    use std::any::Any;

    #[test]
    fn create_and_get_resources_immutably() {
        let mut world = World::new();
        let fps = world.get_resource::<FpsResource>();
        assert_eq!(fps, None);
        world.add_resource(FpsResource(60));
        let fps = world.get_resource::<FpsResource>();
        assert_eq!(fps, Some(&FpsResource(60)));
    }

    #[test]
    fn get_resources_mutably() {
        let mut world = World::new();
        let fps = world.get_resource_mut::<FpsResource>();
        assert_eq!(fps, None);
        world.add_resource(FpsResource(60));
        {
            let fps = world.get_resource_mut::<FpsResource>();
            assert_eq!(fps, Some(&mut FpsResource(60)));
            let fps = fps.unwrap();
            (*fps).0 += 1;
        }
        let fps = world.get_resource::<FpsResource>();
        assert_eq!(fps, Some(&FpsResource(61)));
    }

    #[test]
    fn delete_resource() {
        let mut world = World::new();
        assert_eq!(
            world.remove_resource::<FpsResource>().type_id(),
            (None as Option<Box<dyn Any>>).type_id()
        );
        world.add_resource(FpsResource(60));
        assert_eq!(
            world.remove_resource::<FpsResource>().map(|o| o.type_id()),
            Some((Box::new(FpsResource(60)) as Box<dyn Any>).type_id())
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FpsResource(u32);

impl Deref for FpsResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FpsResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
