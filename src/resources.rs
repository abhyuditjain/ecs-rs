use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id) {
            return data.downcast_ref();
        }
        None
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id) {
            return data.downcast_mut();
        }
        None
    }

    pub fn remove<T: Any>(&mut self) -> Option<Box<dyn Any>> {
        self.data.remove(&TypeId::of::<T>())
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use crate::resources::Resources;
    use std::any::{Any, TypeId};

    #[derive(Debug, PartialEq)]
    struct WorldWidth(f32);

    #[test]
    fn add_resource() {
        let mut resources = Resources::default();
        let world_width = WorldWidth(100.0);
        resources.add(world_width);

        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();

        assert_eq!(extracted_world_width.0, 100.0_f32);
    }

    #[test]
    fn get_resource() {
        let mut resources = Resources::default();
        let world_width = WorldWidth(100.0);
        assert_eq!(resources.get_ref::<WorldWidth>(), None);
        resources.add(world_width);
        assert_eq!(resources.get_ref::<WorldWidth>(), Some(&WorldWidth(100.0)));
    }

    #[test]
    fn get_mut() {
        let mut resources = Resources::default();
        let world_width = WorldWidth(100.0);
        assert_eq!(resources.get_mut::<WorldWidth>(), None);
        resources.add(world_width);
        {
            let world_width = resources.get_mut::<WorldWidth>();
            assert_eq!(world_width, Some(&mut WorldWidth(100.0)));
            let world_width = world_width.unwrap();
            (*world_width).0 += 100.0;
        }
        assert_eq!(resources.get_ref::<WorldWidth>(), Some(&WorldWidth(200.0)));
    }

    #[test]
    fn remove() {
        let mut resources = Resources::default();
        let world_width = WorldWidth(100.0);
        resources.add(world_width);
        assert_eq!(
            resources.remove::<WorldWidth>().map(|o| o.type_id()),
            Some((Box::new(WorldWidth(100.0)) as Box<dyn Any>).type_id())
        );
    }
}
