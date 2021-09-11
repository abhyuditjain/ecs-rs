mod custom_errors;
mod entities;
mod resources;

use crate::entities::query::Query;
use crate::entities::Entities;
use crate::resources::Resources;
use eyre::Result;
use std::any::Any;

#[derive(Default)]
pub struct World {
    resources: Resources,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a resource.
    /// ```
    /// use ecs_lib_rs::World;
    /// let mut world = World::new();
    /// world.add_resource(1_u32);
    /// assert_eq!(world.get_resource::<u32>(), Some(&1));
    /// ```
    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource)
    }

    /// Query for a resource and get a reference to it. The type of the resource must be added in so that it can find it.
    /// ```
    /// use ecs_lib_rs::World;
    /// let mut world = World::new();
    /// assert_eq!(world.get_resource::<u32>(), None);
    /// world.add_resource(1_u32);
    /// assert_eq!(world.get_resource::<u32>(), Some(&1));
    /// ```
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a mutable reference to it. The type of the resource must be added in so that it can find it.
    /// ```
    /// use ecs_lib_rs::World;
    /// let mut world = World::new();
    /// assert_eq!(world.get_resource_mut::<u32>(), None);
    /// world.add_resource(1_u32);
    /// {
    ///     let x = world.get_resource_mut::<u32>();
    ///     assert_eq!(x, Some(&mut 1_u32));
    ///     let x = x.unwrap();
    ///     *x += 1;
    /// }
    /// assert_eq!(world.get_resource::<u32>(), Some(&2));
    /// ```
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// Removes the resource from the world. Returns `None` if the resource wasn't present and hence was not deleted.
    /// Otherwise, it returns `Some(data)`
    /// ```   
    /// use ecs_lib_rs::World;
    /// let mut world = World::new();
    /// world.add_resource(1_u32);
    /// world.remove_resource::<u32>();
    /// assert_eq!(world.get_resource::<u32>(), None);
    /// ```
    pub fn remove_resource<T: Any>(&mut self) -> Option<Box<dyn Any>> {
        self.resources.remove::<T>()
    }

    /// Register a component. The type of the resource must be added in so that it can find it.
    /// ```
    /// use ecs_lib_rs::World;
    /// let mut world = World::new();
    /// world.register_component::<u32>();
    ///
    /// ```
    pub fn register_component<T: Any>(&mut self) {
        self.entities.register_component::<T>()
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }

    pub fn delete_component_by_entity_id<T: Any>(&mut self, id: usize) -> Result<()> {
        self.entities.delete_component_by_entity_id::<T>(id)
    }

    pub fn add_component_to_entity_by_id(&mut self, id: usize, component: impl Any) -> Result<()> {
        self.entities.add_component_by_entity_id(id, component)
    }

    pub fn delete_entity_by_id(&mut self, id: usize) -> Result<()> {
        self.entities.delete_by_id(id)
    }
}
