use std::hash::{DefaultHasher, Hash, Hasher};

const COLLISION_LIMIT: usize = 2;

#[derive(Debug, Clone)]
pub struct GlobalSet<T> {
    data: Vec<Option<T>>,
    inserted_elements: usize,
}

enum IndexResult {
    Existing(usize),
    Empty(usize),
    CollisionLimitReached,
}

fn next_capacity_step(current: usize) -> usize {
    (current * 2).max(4)
}

pub trait Settable: Hash + Clone + PartialEq {}
impl<T: Hash + Clone + PartialEq> Settable for T {}

impl<T: Settable> GlobalSet<T> {
    fn hash(input: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }

    fn index_for(&self, input: &T) -> IndexResult {
        let list_size = self.data.len() as u64;
        let hash = Self::hash(input);
        let index: usize = (hash % (list_size)) as usize;
        let mut collisions = 0;

        let wrap_index = |cols: usize| (((index + cols) as u64) % list_size) as usize;

        while let Some(val) = self
            .data
            .get(wrap_index(collisions))
            .and_then(|i| i.as_ref())
        {
            if val == input {
                return IndexResult::Existing(wrap_index(collisions));
            }

            collisions += 1;

            if collisions >= COLLISION_LIMIT {
                return IndexResult::CollisionLimitReached;
            }
        }

        IndexResult::Empty(wrap_index(collisions))
    }

    pub fn new(input: &[T]) -> Self {
        let mut new_set = Self {
            data: vec![None; next_capacity_step(input.len())],
            inserted_elements: 0,
        };

        for item in input {
            new_set.add(item.clone());
        }

        new_set
    }

    pub fn contains(&self, element: &T) -> bool {
        matches!(self.index_for(element), IndexResult::Existing(_))
    }

    fn resize(&mut self) {
        let new_capacity = next_capacity_step(self.data.len());
        let old_vec = std::mem::replace(&mut self.data, vec![None; new_capacity]);

        self.inserted_elements = 0;

        for item in old_vec.into_iter().flatten() {
            self.add(item);
        }
    }

    pub fn add(&mut self, element: T) {
        match self.index_for(&element) {
            IndexResult::Existing(_) => {}
            IndexResult::Empty(index) => {
                self.data[index] = Some(element);
                self.inserted_elements += 1;
            }
            IndexResult::CollisionLimitReached => {
                self.resize();
                self.add(element);
            }
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.inserted_elements > other.inserted_elements {
            return false;
        }

        self.data
            .iter()
            .flatten()
            .all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.inserted_elements == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data
            .iter()
            .flatten()
            .all(|element| !other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut new_list = Self::new(&[]);
        for element in self.data.iter().flatten() {
            if other.contains(element) {
                new_list.add(element.clone());
            }
        }
        new_list
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut new_list = Self::new(&[]);
        for element in self.data.iter().flatten() {
            if !other.contains(element) {
                new_list.add(element.clone());
            }
        }

        new_list
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut new_list = self.clone();
        for element in other.data.iter().flatten() {
            new_list.add(element.clone());
        }
        new_list
    }
}

impl<T: Settable> PartialEq for GlobalSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.inserted_elements != other.inserted_elements {
            return false;
        }

        self.is_subset(other)
    }
}
