pub mod query;

use crate::custom_errors::CustomError;
use eyre::Result;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type ComponentList = Vec<Option<Rc<RefCell<dyn Any>>>>;

#[derive(Default, Debug)]
pub struct Entities {
    components: HashMap<TypeId, ComponentList>,
    bit_masks: HashMap<TypeId, u32>,
    map: Vec<u32>,
    first_empty_index: usize,
}

impl Entities {
    pub fn register_component<T: Any>(&mut self) {
        self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(Vec::new);
        self.bit_masks
            .entry(TypeId::of::<T>())
            .or_insert(1 << (self.components.len() - 1));
    }

    pub fn create_entity(&mut self) -> &mut Self {
        if let Some((index, _)) = self.map.iter().enumerate().find(|(_, mask)| **mask == 0) {
            self.first_empty_index = index;
        } else {
            self.components.iter_mut().for_each(|(_, v)| v.push(None));
            self.map.push(0);
            self.first_empty_index = self.map.len() - 1;
        }
        self
    }

    pub fn with_component(&mut self, component: impl Any) -> Result<&mut Self> {
        let type_id = &component.type_id();
        let index = self.first_empty_index;
        match self.components.get_mut(type_id) {
            None => Err(CustomError::ComponentNotRegistered.into()),
            Some(component_list) => {
                let component_at_index = component_list
                    .get_mut(index)
                    .ok_or(CustomError::CreateComponentNeverCalled)
                    .unwrap();
                *component_at_index = Some(Rc::new(RefCell::new(component)));
                let bitmask = self.bit_masks.get(type_id).unwrap();
                *(self.map.get_mut(index).unwrap()) |= bitmask;
                Ok(self)
            }
        }
    }

    pub fn get_bitmask(&self, type_id: &TypeId) -> Option<u32> {
        self.bit_masks.get(type_id).copied()
    }

    pub fn delete_component_by_entity_id<T: Any>(&mut self, id: usize) -> Result<()> {
        let type_id = TypeId::of::<T>();
        match self.bit_masks.get(&type_id) {
            None => Err(CustomError::ComponentNotRegistered.into()),
            Some(&mask) => {
                self.map[id] ^= mask;
                Ok(())
            }
        }
    }

    pub fn add_component_by_entity_id(&mut self, id: usize, component: impl Any) -> Result<()> {
        let type_id = component.type_id();
        match self.bit_masks.get(&type_id) {
            None => Err(CustomError::ComponentNotRegistered.into()),
            Some(&mask) => {
                let components = self.components.get_mut(&type_id).unwrap();
                components[id] = Some(Rc::new(RefCell::new(component)));
                self.map[id] |= mask;
                Ok(())
            }
        }
    }

    pub fn delete_by_id(&mut self, id: usize) -> Result<()> {
        match self.map.get_mut(id) {
            None => Err(CustomError::EntityDoesNotExist.into()),
            Some(entity) => {
                *entity = 0;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::Entities;
    use eyre::Result;
    use std::any::TypeId;

    #[derive(Debug, PartialEq)]
    struct Health(u32);

    #[derive(Debug, PartialEq)]
    struct Speed(u32);

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();
        assert!(entities.components.get(&TypeId::of::<Health>()).is_none());
        entities.register_component::<Health>();
        let health_components = entities.components.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(health_components.len(), 0);
    }

    #[test]
    fn bitmask_updated_when_registering_entity() {
        let mut entities = Entities::default();
        assert!(entities.components.get(&TypeId::of::<Health>()).is_none());
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.register_component::<u32>();

        let bitmask = entities.bit_masks.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(*bitmask, 1);

        let bitmask = entities.bit_masks.get(&TypeId::of::<Speed>()).unwrap();
        assert_eq!(*bitmask, 2);

        let bitmask = entities.bit_masks.get(&TypeId::of::<u32>()).unwrap();
        assert_eq!(*bitmask, 4);

        // Does not exist
        let bitmask = entities.bit_masks.get(&TypeId::of::<String>());
        assert_eq!(bitmask, None);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        entities.create_entity();
        let health_components = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let speed_components = entities.components.get(&TypeId::of::<Speed>()).unwrap();
        assert_eq!(health_components.len(), 1);
        assert_eq!(speed_components.len(), 1);
        assert!(health_components[0].is_none());
        assert!(speed_components[0].is_none());
    }

    #[test]
    fn with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(10))?;

        let health_component = entities
            .components
            .get(&TypeId::of::<Health>())
            .unwrap()
            .first()
            .unwrap()
            .as_ref()
            .unwrap()
            .borrow();
        let health = health_component.downcast_ref::<Health>().unwrap();
        assert_eq!(health, &Health(100));

        let speed_component = entities
            .components
            .get(&TypeId::of::<Speed>())
            .unwrap()
            .first()
            .unwrap()
            .as_ref()
            .unwrap()
            .borrow();
        let speed = speed_component.downcast_ref::<Speed>().unwrap();
        assert_eq!(speed, &Speed(10));
        Ok(())
    }

    #[test]
    fn map_updated_when_creating_entities() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(10))?;
        let entity_map = entities.map[0];
        assert_eq!(entity_map, 3);

        entities.create_entity().with_component(Speed(10))?;
        let entity_map = entities.map[1];
        assert_eq!(entity_map, 2);
        Ok(())
    }

    #[test]
    fn delete_component_by_entity_id() -> Result<()> {
        let mut entities = Entities::default();

        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(50))?;

        entities.delete_component_by_entity_id::<Health>(0)?;

        assert_eq!(entities.map[0], 2);

        Ok(())
    }

    #[test]
    fn add_component_by_entity_id() -> Result<()> {
        let mut entities = Entities::default();

        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        entities.create_entity().with_component(Health(100))?;

        entities.add_component_by_entity_id(0, Speed(10))?;

        assert_eq!(entities.map[0], 3);

        let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap()[0]
            .as_ref()
            .unwrap()
            .borrow();
        let speed = speed.downcast_ref::<Speed>().unwrap();

        assert_eq!(speed, &Speed(10));

        Ok(())
    }

    #[test]
    fn delete_by_id() -> Result<()> {
        let mut entities = Entities::default();

        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        assert!(entities.delete_by_id(0).is_err());

        entities.create_entity().with_component(Health(100))?;

        entities.delete_by_id(0)?;

        assert_eq!(entities.map[0], 0);

        Ok(())
    }

    #[test]
    fn created_entities_use_deleted_entities_space() -> Result<()> {
        let mut entities = Entities::default();

        entities.register_component::<Health>();

        entities.create_entity().with_component(Health(100))?;
        entities.create_entity().with_component(Health(50))?;

        entities.delete_by_id(0)?;

        entities.create_entity().with_component(Health(25))?;

        assert_eq!(entities.map[0], 1);

        let health_components = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let health = health_components[0].as_ref().unwrap().borrow();
        let health = health.downcast_ref::<Health>().unwrap();
        assert_eq!(health, &Health(25));

        Ok(())
    }
}
