use slab::Slab;


#[derive(Debug)]
pub struct NodeSlab<T>(pub(crate) Slab<Node<T>>);

#[derive(Debug)]
pub struct LinkedList {
    start: Option<usize>,
    end: Option<usize>
}

#[derive(Debug)]
pub struct Node<T> {
    pub(crate) value: T,
    prev: Option<usize>,
    next: Option<usize>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            start: None,
            end: None
        }
    }

    pub fn push<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>, value: T) -> usize {
        if self.start.is_none() {
            let node = Node {
                value,
                prev: None,
                next: None
            };

            let index = slab.insert(node);
            self.start = Some(index);
            index
        } else {
            let prev = self.end.or(self.start);

            let node = Node {
                value, prev,
                next: None
            };

            let index = slab.insert(node);
            self.end = Some(index);

            if let Some(prev) = prev {
                let node = &mut slab[prev];
                assert!(node.next.is_none());
                node.next = Some(index);
            }

            index
        }
    }

    pub fn pop_front<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>) -> Option<T> {
        let index = self.start?;
        let node = slab.remove(index);

        assert!(node.prev.is_none());

        self.start = if node.next == self.end {
            self.end.take()
        } else {
            node.next
        };

        if let Some(index) = self.start {
            slab[index].prev.take();
        }

        Some(node.value)
    }

    pub fn pop_last<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>) -> Option<T> {
        if let Some(index) = self.end.take() {
            let node = slab.remove(index);

            assert!(node.next.is_none());

            if let Some(index) = node.prev {
                slab[index].next.take();
            }

            if self.start != node.prev {
                self.end = node.prev;
            }

            Some(node.value)
        } else {
            let index = self.start.take()?;
            let node = slab.remove(index);

            assert!(node.prev.is_none());
            assert!(node.next.is_none());

            Some(node.value)
        }
    }

    pub fn touch<'a, T>(&mut self, NodeSlab(slab): &'a mut NodeSlab<T>, index: usize) -> Option<&'a mut T> {
        let (node_prev, node_next) = {
            let node = slab.get(index)?;
            (node.prev, node.next)
        };

        if let Some(next) = node_next {
            slab[next].prev = node_prev;
        } else {
            return Some(&mut slab[index].value);
        }

        if let Some(prev) = node_prev {
            slab[prev].next = node_next;
        } else {
            self.start = node_next;
        }

        let end = self.end.replace(index)?;
        slab[end].next = Some(index);

        let node = &mut slab[index];
        node.prev = Some(end);
        node.next.take();

        Some(&mut node.value)
    }

    pub fn remove<T>(&mut self, NodeSlab(slab): &mut NodeSlab<T>, index: usize) -> Option<T> {
        let node = if slab.contains(index) {
            // why not return Option :(
            slab.remove(index)
        } else {
            return None
        };

        if let Some(prev) = node.prev {
            slab[prev].next = node.next;
        } else {
            self.start = node.next;
        }

        if let Some(next) = node.next {
            slab[next].prev = node.prev;
        } else {
            self.end = node.prev;
        }

        Some(node.value)
    }
}

impl<T> NodeSlab<T> {
    pub fn new() -> NodeSlab<T> {
        NodeSlab(Slab::new())
    }

    pub fn with_capacity(cap: usize) -> NodeSlab<T> {
        NodeSlab(Slab::with_capacity(cap))
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        Some(&self.0.get(index)?.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        Some(&mut self.0.get_mut(index)?.value)
    }
}


#[test]
fn test_linkedlist() {
    let mut slab = NodeSlab::new();
    let mut list = LinkedList::new();

    list.push(&mut slab, 0);
    assert_eq!(Some(0), list.pop_front(&mut slab));
    assert_eq!(None, list.pop_front(&mut slab));

    list.push(&mut slab, 1);
    assert_eq!(Some(1), list.pop_last(&mut slab));
    assert_eq!(None, list.pop_last(&mut slab));

    list.push(&mut slab, 2);
    list.push(&mut slab, 3);
    assert_eq!(Some(2), list.pop_front(&mut slab));
    assert_eq!(Some(3), list.pop_last(&mut slab));
    assert_eq!(None, list.pop_front(&mut slab));
    assert_eq!(None, list.pop_last(&mut slab));

    list.push(&mut slab, 4);
    list.push(&mut slab, 5);
    assert_eq!(Some(5), list.pop_last(&mut slab));
    assert_eq!(Some(4), list.pop_front(&mut slab));
    assert_eq!(None, list.pop_last(&mut slab));
    assert_eq!(None, list.pop_front(&mut slab));

    let index6 = list.push(&mut slab, 6);
    let index7 = list.push(&mut slab, 7);
    let index8 = list.push(&mut slab, 8);
    assert_eq!(Some(7), list.remove(&mut slab, index7));
    assert_eq!(None, list.remove(&mut slab, index7));
    assert_eq!(Some(&6), slab.get(index6));
    assert_eq!(Some(&8), slab.get(index8));
    assert_eq!(Some(6), list.pop_front(&mut slab));
    assert_eq!(Some(8), list.pop_front(&mut slab));

    let index9 = list.push(&mut slab, 9);
    list.push(&mut slab, 10);
    assert_eq!(Some(&mut 9), list.touch(&mut slab, index9));
    assert_eq!(Some(10), list.pop_front(&mut slab));
    assert_eq!(Some(9), list.pop_front(&mut slab));

    let index11 = list.push(&mut slab, 11);
    let index12 = list.push(&mut slab, 12);
    list.push(&mut slab, 13);
    assert_eq!(Some(&mut 12), list.touch(&mut slab, index12));
    assert_eq!(Some(&mut 11), list.touch(&mut slab, index11));
    assert_eq!(Some(13), list.pop_front(&mut slab));
    assert_eq!(Some(12), list.pop_front(&mut slab));
    assert_eq!(Some(11), list.pop_front(&mut slab));

    for i in 0..32 {
        list.push(&mut slab, i);
    }
    for i in 0..32 {
        assert_eq!(Some(i), list.pop_front(&mut slab));
    }
    assert_eq!(None, list.pop_front(&mut slab));

    for i in 0..32 {
        list.push(&mut slab, i);
    }
    for i in (0..32).rev() {
        assert_eq!(Some(i), list.pop_last(&mut slab));
    }
    assert_eq!(None, list.pop_last(&mut slab));
}
