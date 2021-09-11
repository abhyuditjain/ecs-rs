#[cfg(test)]
mod tests {
    use ecs_lib_rs::World;
    use eyre::Result;
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq)]
    struct Location(f32, f32);
    #[derive(Debug, PartialEq)]
    struct Size(f32);

    #[test]
    fn create_entity() -> Result<()> {
        let mut world = World::new();

        world.register_component::<Location>();
        world.register_component::<Size>();

        world
            .create_entity()
            .with_component(Location(42.0, 69.0))?
            .with_component(Size(10.0))?;

        Ok(())
    }

    #[test]
    fn query_entities() -> Result<()> {
        let mut world = World::new();

        world.register_component::<Location>();
        world.register_component::<Size>();

        world
            .create_entity()
            .with_component(Location(42.0, 24.0))?
            .with_component(Size(10.0))?;

        world.create_entity().with_component(Size(11.0))?;

        world.create_entity().with_component(Location(43.0, 25.0))?;

        world
            .create_entity()
            .with_component(Location(44.0, 26.0))?
            .with_component(Size(12.0))?;

        let results = world
            .query()
            .with_component::<Location>()?
            .with_component::<Size>()?
            .run();

        let locations: &Vec<Rc<RefCell<dyn Any>>> = &results.1[0];
        let sizes: &Vec<Rc<RefCell<dyn Any>>> = &results.1[1];

        assert_eq!(sizes.len(), 2);
        assert_eq!(locations.len(), 2);

        let borrowed_first_location = locations[0].borrow();
        let location = borrowed_first_location.downcast_ref::<Location>().unwrap();
        assert_eq!(location, &Location(42.0, 24.0));

        let borrowed_first_size = sizes[0].borrow();
        let size = borrowed_first_size.downcast_ref::<Size>().unwrap();
        assert_eq!(size, &Size(10.0));

        let borrowed_second_location = locations[1].borrow();
        let location = borrowed_second_location.downcast_ref::<Location>().unwrap();
        assert_eq!(location, &Location(44.0, 26.0));

        let borrowed_second_size = sizes[1].borrow();
        let size = borrowed_second_size.downcast_ref::<Size>().unwrap();
        assert_eq!(size, &Size(12.0));

        Ok(())
    }

    #[test]
    fn delete_component_from_entity() -> Result<()> {
        let mut world = World::new();

        world.register_component::<Location>();
        world.register_component::<Size>();

        world
            .create_entity()
            .with_component(Location(10.0, 11.0))?
            .with_component(Size(10.0))?;
        world
            .create_entity()
            .with_component(Location(20.0, 21.0))?
            .with_component(Size(20.0))?;

        world.delete_component_by_entity_id::<Location>(0)?;

        let results = world
            .query()
            .with_component::<Location>()?
            .with_component::<Size>()?
            .run();

        assert_eq!(results.0.len(), 1);
        assert_eq!(results.0[0], 1);

        Ok(())
    }

    #[test]
    fn add_component_to_entity_by_id() -> Result<()> {
        let mut world = World::new();

        world.register_component::<Location>();
        world.register_component::<Size>();

        world.create_entity().with_component(Location(10.0, 11.0))?;

        world.add_component_to_entity_by_id(0, Size(10.0))?;

        let results = world
            .query()
            .with_component::<Location>()?
            .with_component::<Size>()?
            .run();

        assert_eq!(results.0.len(), 1);

        Ok(())
    }

    #[test]
    fn delete_entity_by_id() -> Result<()> {
        let mut world = World::new();

        world.register_component::<Location>();
        world.register_component::<Size>();

        assert!(world.delete_entity_by_id(0).is_err());

        world
            .create_entity()
            .with_component(Location(10.0, 11.0))?
            .with_component(Size(10.0))?;

        world.delete_entity_by_id(0)?;

        let results = world
            .query()
            .with_component::<Location>()?
            .with_component::<Size>()?
            .run();

        assert_eq!(results.0.len(), 0);
        assert_eq!(results.1[0].len(), 0);
        assert_eq!(results.1[1].len(), 0);

        Ok(())
    }
}
