use std::{cmp::Ordering, slice::{Iter, IterMut}, fs::File, path::Path};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{BackendError, ToBackendResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OrderMode {
    #[default]
    Alphabetical,
    Initiative
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CreatureContainer {
    ordering: OrderMode,
    creatures: Vec<Creature>
}

impl From<Vec<Creature>> for CreatureContainer {
    fn from(mut value: Vec<Creature>) -> Self {
        value.sort_by(alphabetical);

        Self {
            ordering: OrderMode::Alphabetical,
            creatures: value
        }
    }
}

impl CreatureContainer {
    pub fn load_from(path: &Path) -> Result<Self, BackendError> {
        let mut file = File::open(path).to_backend_result()?;
        let content: Vec<CreatureData> = serde_json::from_reader(&mut file).to_backend_result()?;
        let content = content.into_iter()
            .map(|c| Creature::from(c))
            .collect::<Vec<_>>();

        Ok(Self {
            ordering: OrderMode::Alphabetical,
            creatures: content
        })
    }

    pub fn save_to(&self, path: &Path) -> Result<(), BackendError> {
        let mut file = File::create(path).to_backend_result()?;
        let content = self.creatures.iter()
            .map(|c| CreatureData::from(c))
            .collect::<Vec<_>>();

        serde_json::to_writer_pretty(&mut file, &content).to_backend_result()
    }

    pub fn sort(&mut self) {
        match self.ordering {
            OrderMode::Alphabetical => self.creatures.sort_by(alphabetical),
            OrderMode::Initiative => self.creatures.sort()
        }
    }

    pub fn set_order_mode(&mut self, mode: OrderMode) {
        if mode != self.ordering {
            self.ordering = mode;
            self.sort();
        }
    }

    pub fn get_index_from_id(&self, id: Uuid) -> Option<usize> {
        self.creatures.iter()
            .enumerate()
            .find_map(|(idx, creature)| {
                if creature.id == id {
                    Some(idx)
                } else {
                    None
                }
            })
    }

    pub fn insert(&mut self, creature: Creature) {
        self.creatures.push(creature);
        self.sort();
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Creature> {
        self.creatures.get(index)
    }

    pub fn get_mut_by_index(&mut self, index: usize) -> Option<&mut Creature> {
        self.creatures.get_mut(index)
    }

    pub fn remove_by_index(&mut self, index: usize) -> Creature {
        self.creatures.remove(index)
    }

    pub fn get(&self, id: Uuid) -> Option<&Creature> {
        let index = self.get_index_from_id(id)?;
        self.get_by_index(index)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Creature> {
        let index = self.get_index_from_id(id)?;
        self.get_mut_by_index(index)
    }

    pub fn remove(&mut self, id: Uuid) -> Option<Creature> {
        let index = self.get_index_from_id(id)?;
        Some(self.remove_by_index(index))
    }

    pub fn iter(&self) -> Iter<Creature> {
        self.creatures.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Creature> {
        self.creatures.iter_mut()
    }

    pub fn cloned(&self) -> Vec<Creature> {
        self.creatures.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatureData {
    id: Uuid,
    name: String,
    initiative: isize,
}

impl From<&Creature> for CreatureData {
    fn from(value: &Creature) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
            initiative: value.initiative()
        }
    }
}

impl From<Creature> for CreatureData {
    fn from(value: Creature) -> Self {
        CreatureData::from(&value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Creature {
    id: Uuid,
    name: String,
    selected: bool,
    initiative: isize,
    sub_order: isize
}

impl From<&CreatureData> for Creature {
    fn from(value: &CreatureData) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            selected: false,
            initiative: value.initiative,
            sub_order: 0
        }
    }
}

impl From<CreatureData> for Creature {
    fn from(value: CreatureData) -> Self {
        Creature::from(&value)
    }
}

impl std::fmt::Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: '{}', name: '{}'}}", self.id, self.name)
    }
}

impl<T: Into<String>> From<T> for Creature {
    fn from(value: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.into(),
            selected: false,
            initiative: 0,
            sub_order: 0
        }
    }
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.initiative.partial_cmp(&other.initiative) {
            Some(Ordering::Equal) => {},
            ord => return ord
        };

        match self.sub_order.partial_cmp(&other.sub_order) {
            Some(Ordering::Equal) => {},
            ord => return ord
        }

        self.name.to_lowercase().partial_cmp(&other.name.to_lowercase())
    }
}

impl Ord for Creature {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.initiative.cmp(&other.initiative) {
            Ordering::Equal => {},
            ord => return ord
        };

        match self.sub_order.cmp(&other.sub_order) {
            Ordering::Equal => {},
            ord => return ord
        }

        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl Creature {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    pub fn initiative(&self) -> isize {
        self.initiative
    }

    pub fn set_initiative(&mut self, value: isize) {
        self.initiative = value;
    }

    pub fn sub_order(&self) -> isize {
        self.sub_order
    }

    pub fn set_sub_order(&mut self, value: isize) {
        self.sub_order = value;
    }
}

fn alphabetical(a: &Creature, b: &Creature) -> Ordering {
    a.name.to_lowercase().cmp(&b.name.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let creature = Creature::from("Test creature");
        let serialized = serde_json::to_string_pretty(&creature).unwrap();

        println!("{}", serialized);
    }

    #[test]
    fn test_deserialization() {
        let creature = Creature::from("Test creature");
        let serialized = serde_json::to_string_pretty(&creature).unwrap();
        let deserialized: Creature = serde_json::from_str(&serialized).unwrap();

        println!("{:#?}", deserialized);
        assert_eq!(creature, deserialized);
    }
}