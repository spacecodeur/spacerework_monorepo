use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::super::types::enums::SegmentTypeEnum;
use super::user::User;

#[derive(Debug)]
pub struct PathSegment {
    pub name: String,
    pub segment_parent: Option<Weak<RefCell<PathSegment>>>,
    pub children: Vec<Rc<RefCell<PathSegment>>>,
    pub trainer: Option<Rc<RefCell<User>>>,
    pub segment_type: SegmentTypeEnum,
}
