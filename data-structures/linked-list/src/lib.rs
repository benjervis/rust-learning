type Item<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug)]
pub struct SimpleLinkedList<T> {
    head: Item<T>,
    len: usize,
}

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Item<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            value: element,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.next;
        self.len -= 1;
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

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T: PartialEq> SimpleLinkedList<T> {
    pub fn contains(&self, element: &T) -> bool {
        self.iter().any(|e| e == element)
    }
}

pub struct Iter<'a, T> {
    curr: &'a Item<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(list: &'a SimpleLinkedList<T>) -> Self {
        Self { curr: &list.head }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.as_ref().map(|v| {
            self.curr = &v.next;
            &v.value
        })
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
