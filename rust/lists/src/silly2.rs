pub struct List<'a, T> {
    data: T,
    prev: Option<&'a List<'a, T>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
    pub fn push<U>(
        prev: Option<&'a List<'a, T>>,
        data: T,
        callback: impl FnOnce(&List<'a, T>) -> U,
    ) -> U {
        let list = List { data, prev: prev };
        callback(&list)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter { next: Some(self) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev;
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn elegance() {
        // as long as push alive, can use the list in the callback
        List::push(None, 3, |list| {
            assert_eq!(list.iter().copied().sum::<i32>(), 3);
            List::push(Some(list), 5, |list| {
                List::push(Some(list), 7, |list| {
                    assert_eq!(list.iter().copied().sum::<i32>(), 3 + 5 + 7);
                });
                assert_eq!(list.iter().copied().sum::<i32>(), 3 + 5);
            });
            assert_eq!(list.iter().copied().sum::<i32>(), 3);
        })
    }
}
