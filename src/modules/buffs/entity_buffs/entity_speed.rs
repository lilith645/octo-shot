use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct EntitySpeedBuff {
  data: BuffData,
}

impl EntitySpeedBuff {
  pub fn new(value: f32) -> EntitySpeedBuff {
    EntitySpeedBuff {
      data: BuffData::new(9, 5, LootRarity::Rare).set_modified_value(value),
    }
  }
  
  pub fn flat_value(mut self) -> EntitySpeedBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn percentage_value(mut self) -> EntitySpeedBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for EntitySpeedBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn apply_stat_modifiers(&self, stat: &mut StatModifier) {
    let v = self.data().modified_value;
    if let Some(additive) = self.data().additive {
      if additive {
        stat.flat_speed += v;
      } else {
        stat.percentage_speed += v;
      }
    }
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    entity.add_stat_buff(Box::new(self.clone()))
  }
  
  fn apply_to_bullet(&self, _bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
