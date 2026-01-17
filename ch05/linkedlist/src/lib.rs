//! # linkedlist crate
//!
//! 이 크레이트는 간단한 연결 리스트를 구현합니다.
//!
//! 이 크레이트는 책 [_Rust Advanced Techniques_](https://www.manning.com/books/idiomatic-rust)의 예제입니다.
//!
//! ## 사용법
//!
//! ```rust
//! use linkedlist::LinkedList;
//!
//! let mut animals = LinkedList::new();
//! animals.append("Triceratops");
//! animals.append("Velociraptor");
//! animals.append("Stegosaurus");
//! animals.append("Spinosaurus");
//! assert_eq!(animals.into_iter().collect::<Vec<_>>(), vec!["Triceratops", "Velociraptor", "Stegosaurus", "Spinosaurus"]);
//! ```
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

pub struct ListItem<T> {
    pub data: ItemData<T>,
    /// 연결 리스트의 다음 항목
    next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
    /// 리스트에 새 항목을 생성한다.
    fn new(t: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(t)),
            next: None,
        }
    }
}

/// 반복자 접근이 가능한 단일 연결 리스트를 제공한다.
pub struct LinkedList<T> {
    /// 리스트의 첫번째 항목에 대한 포인터
    head: Option<ListItemPtr<T>>,
}

impl<T> LinkedList<T> {
    /// 새로운 빈 [`LinkedList<T>`] 객체를 생성한다.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// 요소를 리스트 끝에 추가한다.
    /// 만일 리스트가 비어있다면 해당 요소는 리스트의 첫번째 요소가 된다.
    pub fn append(&mut self, t: T) {
        match &self.head {
            Some(head) => {
                let mut next = head.clone();
                while next.as_ref().borrow().next.is_some() {
                    let n = next.as_ref().borrow().next.as_ref().unwrap().clone();
                    next = n;
                }
                next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(t))));
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(ListItem::new(t))));
            }
        }
    }

    /// 리스트를 반복할 반복자를 반환한다.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.clone(),
            data: None,
            phantom: PhantomData,
        }
    }

    /// 리스트의 가변 반복자를 반환한다.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.clone(),
            data: None,
            phantom: PhantomData,
        }
    }

    /// 리스트를 소모해 요소의 값을 반환하는 반복자를 만든다.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: self.head.clone(),
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::new();
        cloned.clone_from(self);
        cloned
    }

    fn clone_from(&mut self, source: &Self) {
        self.head = None;
        source.iter().for_each(|item| {
            self.append(item.clone());
        });
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

pub struct Iter<'a, T> {
    next: Option<ListItemPtr<T>>,
    data: Option<ItemData<T>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                self.data = Some(ptr.as_ref().borrow().data.clone());
                unsafe { Some(&*self.data.as_ref().unwrap().as_ptr()) }
            }
            None => None,
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<ListItemPtr<T>>,
    data: Option<ItemData<T>>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                self.data = Some(ptr.as_ref().borrow().data.clone());
                unsafe { Some(&mut *self.data.as_ref().unwrap().as_ptr()) }
            }
            None => None,
        }
    }
}

pub struct IntoIter<T> {
    next: Option<ListItemPtr<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                let listitem = Rc::try_unwrap(ptr).map(|refcell| refcell.into_inner());
                match listitem {
                    Ok(listitem) => Rc::try_unwrap(listitem.data)
                        .map(|refcell| refcell.into_inner())
                        .ok(),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}
