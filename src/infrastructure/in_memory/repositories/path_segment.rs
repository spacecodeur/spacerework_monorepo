use std::cell::RefCell;
use std::rc::Rc;

use crate::domain::entities::{path_segment::*, user::*};
use crate::domain::repositories::path_segment::PathSegmentRepository;
use crate::domain::types::enums::SegmentTypeEnum;

pub struct InMemoryPathSegmentRepository {
    roots: Vec<Rc<RefCell<PathSegment>>>,
}

impl InMemoryPathSegmentRepository {
    pub fn new() -> Self {
        Self { roots: Vec::new() }
    }
}

impl PathSegmentRepository for InMemoryPathSegmentRepository {
    fn add_root(
        &mut self,
        segment_name: &str,
        trainer: Rc<RefCell<User>>,
        segment_type: SegmentTypeEnum,
    ) -> Result<Rc<RefCell<PathSegment>>, String> {
        let segment = Rc::new(RefCell::new(PathSegment {
            name: segment_name.to_string(),
            segment_parent: None,
            children: vec![],
            trainer: Some(Rc::clone(&trainer)),
            segment_type,
        }));
        self.roots.push(segment.clone());
        Ok(segment)
    }

    fn add(
        &mut self,
        path_segment_parent: &Rc<RefCell<PathSegment>>,
        segment_name: &str,
        segment_type: SegmentTypeEnum,
    ) -> Result<Rc<RefCell<PathSegment>>, String> {
        let segment = Rc::new(RefCell::new(PathSegment {
            name: segment_name.to_string(),
            segment_parent: Some(Rc::downgrade(path_segment_parent)),
            children: vec![],
            trainer: None,
            segment_type,
        }));

        path_segment_parent
            .borrow_mut()
            .children
            .push(segment.clone());

        Ok(segment)
    }

    fn delete(&mut self, path_segment: &Rc<RefCell<PathSegment>>) -> Result<(), String> {
        if let Some(weak_parent) = &path_segment.borrow().segment_parent {
            if let Some(parent_rc) = weak_parent.upgrade() {
                parent_rc
                    .borrow_mut()
                    .children
                    .retain(|child| !Rc::ptr_eq(child, path_segment));
            }
        } else {
            self.roots.retain(|root| !Rc::ptr_eq(root, path_segment));
        }
        Ok(())
    }

    fn delete_recursively(
        &mut self,
        path_segment: &Rc<RefCell<PathSegment>>,
    ) -> Result<(), String> {
        let children = path_segment.borrow().children.clone();
        for child in children {
            self.delete_recursively(&child)?;
        }
        self.delete(path_segment)?;
        Ok(())
    }

    fn get_roots(&self) -> Vec<Rc<RefCell<PathSegment>>> {
        self.roots.clone()
    }
}
