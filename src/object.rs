use std::cmp::Ordering;

use alot::{LotId, Lots};

use crate::any::AnyComponent;
use crate::Name;

/// A map of Name -> `AnyComponent`.
///
/// The keys in this structure are sorted, using the interned string's pointers
/// for comparisons. This avoids comparing the actual strings, and reduces each
/// comparison to two pointer comparisons. The values are stored separately in a
/// slot map, which reduces the cost of removing and reinserting components, as
/// only one Vec needs to have elements moved.
pub struct Object {
    values: Lots<AnyComponent>,
    ordered_keys: Vec<Key>,
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

impl Object {
    pub const fn new() -> Self {
        Self {
            values: Lots::new(),
            ordered_keys: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Lots::with_capacity(capacity),
            ordered_keys: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: Name, value: AnyComponent) -> Option<AnyComponent> {
        match self.find_key(&key) {
            Ok(existing) => {
                let value_id = existing.value_id;
                Some(std::mem::replace(&mut self.values[value_id], value))
            }
            Err(insert_at) => {
                let value_id = self.values.push(value);
                self.ordered_keys.insert(insert_at, Key { key, value_id });
                None
            }
        }
    }

    pub fn get(&self, key: &Name) -> Option<&AnyComponent> {
        self.find_key(key)
            .ok()
            .map(|key| &self.values[key.value_id])
    }

    pub fn len(&self) -> usize {
        self.ordered_keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ordered_keys.is_empty()
    }

    fn find_key(&self, needle: &Name) -> Result<&Key, usize> {
        // When the collection contains 16 or fewer elements, there should be no
        // jumps before we reach a sequential scan for the key. When the
        // collection is larger, we use a binary search to narrow the search
        // window until the window is 16 elements or less.
        let mut min = 0;
        let mut max = self.ordered_keys.len();
        loop {
            let delta = max - min;
            if delta <= 16 {
                for (index, key) in self.ordered_keys[min..max].iter().enumerate() {
                    match key.partial_cmp(needle).expect("invalid comparison") {
                        Ordering::Less => continue,
                        Ordering::Equal => return Ok(key),
                        Ordering::Greater => return Err(index),
                    }
                }

                return Err(max);
            }

            let midpoint = min + delta / 2;
            match self.ordered_keys[midpoint]
                .partial_cmp(needle)
                .expect("invalid comparison")
            {
                Ordering::Less => min = midpoint + 1,
                Ordering::Equal => return Ok(&self.ordered_keys[midpoint]),
                Ordering::Greater => max = midpoint,
            }
        }
    }

    pub fn values(&self) -> alot::unordered::Iter<'_, AnyComponent> {
        self.values.iter()
    }

    pub fn into_values(self) -> alot::unordered::IntoIter<AnyComponent> {
        self.values.into_iter()
    }

    pub fn merge_with_filter(
        &mut self,
        other: &Self,
        mut filter: impl FnMut(&AnyComponent) -> bool,
    ) {
        let mut self_index = 0;
        let mut other_index = 0;

        while self_index < self.len() && other_index < other.len() {
            let self_key = &self.ordered_keys[self_index];
            let other_key = &other.ordered_keys[other_index];
            let other_component = &other.values[other_key.value_id];
            match self_key.cmp(other_key) {
                Ordering::Less => {
                    // Self has a key that other didn't.
                    self_index += 1;
                }
                Ordering::Equal => {
                    // Both have the value, we might need to merge.
                    self_index += 1;
                    other_index += 1;
                    if filter(other_component) {
                        self.values[self_key.value_id].merge_with(other_component);
                    }
                }
                Ordering::Greater => {
                    // Other has a value that self doesn't.
                    other_index += 1;
                    if !filter(other_component) {
                        continue;
                    }
                    let value_id = self.values.push(other_component.clone());
                    self.ordered_keys.insert(
                        self_index,
                        Key {
                            key: other_key.key.clone(),
                            value_id,
                        },
                    );
                    self_index += 1;
                }
            }
        }

        if other_index < other.ordered_keys.len() {
            // Other has more entries that we don't have
            for key in &other.ordered_keys[other_index..] {
                let other_component = &other.values[key.value_id];
                if !filter(other_component) {
                    continue;
                }
                let value_id = self.values.push(other_component.clone());
                self.ordered_keys.push(Key {
                    key: key.key.clone(),
                    value_id,
                });
            }
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let mut new_obj = Self::with_capacity(self.len());

        for key in &self.ordered_keys {
            new_obj.insert(key.key.clone(), self.values[key.value_id].clone());
        }

        new_obj
    }
}

pub struct Key {
    pub key: Name,
    pub value_id: LotId,
}

impl Eq for Key {}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl PartialEq<Name> for Key {
    fn eq(&self, other: &Name) -> bool {
        &self.key == other
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other.key)
            .expect("always returns a result")
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<Name> for Key {
    fn partial_cmp(&self, other: &Name) -> Option<Ordering> {
        match self.key.authority.as_ptr().cmp(&other.authority.as_ptr()) {
            Ordering::Equal => Some(self.key.name.as_ptr().cmp(&other.name.as_ptr())),
            not_equal => Some(not_equal),
        }
    }
}
