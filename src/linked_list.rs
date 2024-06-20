type Item<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Item<T>,
}

struct Node<T> {
    value: T,
    next: Item<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut next_node = &self.head;

        while let Some(next) = next_node {
            count += 1;
            next_node = &next.next;
        }

        count
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            value: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.next;
        Some(old_head.value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node_ref| &node_ref.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list = Self::new();

        let mut next = self.head;

        while let Some(next_node) = next.take() {
            new_list.push(next_node.value);
            next = next_node.next;
        }

        new_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = Self::new();

        for item in iter.into_iter() {
            new_list.push(item);
        }

        new_list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        let mut rev_list = linked_list.rev();

        while let Some(item) = rev_list.pop() {
            vec.push(item);
        }

        vec
    }
}
