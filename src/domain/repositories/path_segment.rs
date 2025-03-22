use std::cell::RefCell;
use std::rc::Rc;

use super::super::entities::{path_segment::*, user::*};
use super::super::types::enums::SegmentTypeEnum;

pub trait PathSegmentRepository {
    fn add_root(
        &mut self,
        segment_name: &str,
        trainer: Rc<RefCell<User>>,
        segment_type: SegmentTypeEnum,
    ) -> Result<Rc<RefCell<PathSegment>>, String>;

    fn add(
        &mut self,
        path_segment_parent: &Rc<RefCell<PathSegment>>,
        segment_name: &str,
        segment_type: SegmentTypeEnum,
    ) -> Result<Rc<RefCell<PathSegment>>, String>;

    fn delete(&mut self, path_segment: &Rc<RefCell<PathSegment>>) -> Result<(), String>;
    fn delete_recursively(&mut self, path_segment: &Rc<RefCell<PathSegment>>)
        -> Result<(), String>;

    fn get_roots(&self) -> Vec<Rc<RefCell<PathSegment>>>;
}
