use std::cell::RefCell;
use std::rc::Rc;

use crate::domain::entities::user::User;
use crate::domain::repositories::path_segment::PathSegmentRepository;
use crate::domain::types::enums::SegmentTypeEnum;
use crate::infrastructure::in_memory::repositories::path_segment::InMemoryPathSegmentRepository;

#[test]
fn test_add_root_segment() {
    let trainer = Rc::new(RefCell::new(User {
        pseudo: "foo".to_string(),
    }));

    let mut repo = InMemoryPathSegmentRepository::new();

    let root = repo
        .add_root("root", Rc::clone(&trainer), SegmentTypeEnum::Lesson)
        .unwrap();

    let root_borrowed = root.borrow();
    assert_eq!(root_borrowed.name, "root");
    assert!(root_borrowed.segment_parent.is_none());
    assert_eq!(root_borrowed.segment_type, SegmentTypeEnum::Lesson);
    assert!(root_borrowed.trainer.is_some());

    let roots = repo.get_roots();
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].borrow().name, "root");
}

#[test]
fn test_add_child_segment() {
    let trainer = Rc::new(RefCell::new(User {
        pseudo: "foo".to_string(),
    }));

    let mut repo = InMemoryPathSegmentRepository::new();

    let root = repo
        .add_root("root", Rc::clone(&trainer), SegmentTypeEnum::Directory)
        .unwrap();

    let child = repo.add(&root, "child", SegmentTypeEnum::Lesson).unwrap();

    {
        let root_borrowed = root.borrow();
        assert_eq!(root_borrowed.children.len(), 1);
        assert_eq!(root_borrowed.children[0].borrow().name, "child");
    }

    {
        let child_borrowed = child.borrow();
        assert_eq!(child_borrowed.name, "child");
        assert_eq!(child_borrowed.segment_type, SegmentTypeEnum::Lesson);
        assert!(child_borrowed.segment_parent.is_some());
    }
}

#[test]
fn test_delete_segment() {
    let trainer = Rc::new(RefCell::new(User {
        pseudo: "foo".to_string(),
    }));

    let mut repo = InMemoryPathSegmentRepository::new();

    let root = repo
        .add_root("root", Rc::clone(&trainer), SegmentTypeEnum::Directory)
        .unwrap();
    let child = repo.add(&root, "child", SegmentTypeEnum::Lesson).unwrap();

    repo.delete(&child).unwrap();

    let root_borrowed = root.borrow();
    assert!(root_borrowed.children.is_empty());
}

#[test]
fn test_delete_root_segment() {
    let trainer = Rc::new(RefCell::new(User {
        pseudo: "foo".to_string(),
    }));

    let mut repo = InMemoryPathSegmentRepository::new();

    let root = repo
        .add_root("root", Rc::clone(&trainer), SegmentTypeEnum::Directory)
        .unwrap();

    repo.delete(&root).unwrap();
    assert!(repo.get_roots().is_empty());
}

#[test]
fn test_recursive_deletion() {
    let trainer = Rc::new(RefCell::new(User {
        pseudo: "foo".to_string(),
    }));

    let mut repo = InMemoryPathSegmentRepository::new();

    let root = repo
        .add_root("root", Rc::clone(&trainer), SegmentTypeEnum::Directory)
        .unwrap();
    let child1 = repo
        .add(&root, "child1", SegmentTypeEnum::Directory)
        .unwrap();
    let _child2 = repo
        .add(&child1, "child2", SegmentTypeEnum::Lesson)
        .unwrap();

    repo.delete_recursively(&child1).unwrap();

    let root_borrowed = root.borrow();
    assert!(root_borrowed.children.is_empty());
}
