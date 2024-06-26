use std::hash::{DefaultHasher, Hash, Hasher};

use linked_list::SimpleLinkedList;

const COLLISION_LIMIT: usize = 2;

#[derive(Debug, Clone)]
pub struct BucketSet<T> {
    data: Vec<SimpleLinkedList<T>>,
    inserted_elements: usize,
}

fn next_capacity_step(current: usize) -> usize {
    (current * 2).max(4)
}

pub trait Settable: Hash + Clone + PartialEq {}
impl<T: Hash + Clone + PartialEq> Settable for T {}

impl<T: Settable> BucketSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut new_set = Self {
            data: vec![SimpleLinkedList::new(); next_capacity_step(input.len())],
            inserted_elements: 0,
        };

        for item in input {
            new_set.add(item.clone());
        }

        new_set
    }

    fn hash(input: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }

    fn get_bucket(&self, input: &T) -> usize {
        let list_size = self.data.len() as u64;
        let hash = Self::hash(input);
        (hash % (list_size)) as usize
    }

    pub fn contains(&self, element: &T) -> bool {
        let bucket_index = self.get_bucket(element);
        self.data[bucket_index].contains(element)
    }

    fn resize(&mut self) {
        let new_capacity = next_capacity_step(self.data.len());
        let old_vec =
            std::mem::replace(&mut self.data, vec![SimpleLinkedList::new(); new_capacity]);

        self.inserted_elements = 0;

        for item in old_vec.into_iter() {
            for list_item in item.iter() {
                self.add(list_item.clone());
            }
        }
    }

    pub fn add(&mut self, element: T) {
        let bucket = self.get_bucket(&element);
        let list = &mut self.data[bucket];

        if list.contains(&element) {
            return;
        }

        if list.len() >= COLLISION_LIMIT {
            self.resize();
            return self.add(element);
        }

        list.push(element);
        self.inserted_elements += 1;
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data
            .iter()
            .filter(|l| !l.is_empty())
            .flat_map(|l| l.iter())
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.inserted_elements > other.inserted_elements {
            return false;
        }

        self.values().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.inserted_elements == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.values().all(|element| !other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        self.values()
            .filter(|e| other.contains(e))
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        self.values()
            .filter(|e| !other.contains(e))
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        self.values().chain(other.values()).cloned().collect()
    }
}

impl<T: Settable> PartialEq for BucketSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.inserted_elements != other.inserted_elements {
            return false;
        }

        self.is_subset(other)
    }
}

impl<T: Settable> FromIterator<T> for BucketSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_set = Self::new(&[]);

        for item in iter.into_iter() {
            new_set.add(item);
        }

        new_set
    }
}
