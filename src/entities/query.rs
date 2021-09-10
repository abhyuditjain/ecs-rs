use crate::custom_errors::CustomError;
use crate::entities::Entities;
use eyre::Result;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;

type QueryResult = (Vec<usize>, Vec<Vec<Rc<RefCell<dyn Any>>>>);

#[derive(Debug)]
pub struct Query<'a> {
    map: u32,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: 0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        match self.entities.get_bitmask(&type_id) {
            None => return Err(CustomError::ComponentNotRegistered.into()),
            Some(bitmask) => {
                // if self.map | bitmask != self.map {
                self.map |= bitmask;
                self.type_ids.push(type_id);
                // }
            }
        }
        Ok(self)
    }

    pub fn run(&self) -> QueryResult {
        let indices = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, &entity_map)| {
                if entity_map & self.map == self.map {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let results = self
            .type_ids
            .iter()
            .map(|type_id| {
                let components = self.entities.components.get(type_id).unwrap();
                indices
                    .iter()
                    .map(|&index| components[index].as_ref().unwrap().clone())
                    .collect()
            })
            .collect();

        (indices, results)
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::query::Query;
    use crate::entities::Entities;
    use eyre::Result;
    use std::any::TypeId;

    #[test]
    fn query_mask_updating_with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        let mut query = Query::new(&entities);

        query.with_component::<u32>()?.with_component::<f32>()?;
        assert_eq!(query.map, 3);

        assert_eq!(query.type_ids[0], TypeId::of::<u32>());
        assert_eq!(query.type_ids[1], TypeId::of::<f32>());

        Ok(())
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn run() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        entities
            .create_entity()
            .with_component(10_u32)?
            .with_component(20.0_f32)?;
        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(50.0_f32)?;
        entities
            .create_entity()
            .with_component(15_u32)?
            .with_component(25.0_f32)?;

        let mut query = Query::new(&entities);

        let results = query
            .with_component::<u32>()?
            .with_component::<f32>()?
            .run();

        assert_eq!(results.1.len(), 2);

        let u32s = &results.1[0];
        let f32s = &results.1[1];
        let indices = &results.0;

        assert_eq!(u32s.len(), 2);
        assert_eq!(f32s.len(), 2);
        assert_eq!(indices.len(), 2);

        assert_eq!(indices[0], 0);
        assert_eq!(indices[1], 3);

        assert_eq!(u32s[0].borrow().downcast_ref::<u32>().unwrap(), &10);
        assert_eq!(u32s[1].borrow().downcast_ref::<u32>().unwrap(), &15);

        assert_eq!(f32s[0].borrow().downcast_ref::<f32>().unwrap(), &20.0_f32);
        assert_eq!(f32s[1].borrow().downcast_ref::<f32>().unwrap(), &25.0_f32);

        Ok(())
    }
}
