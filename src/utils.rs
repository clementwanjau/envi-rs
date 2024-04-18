use std::rc::Rc;
use crate::ArchitectureModule;
use crate::constants::{ARCH_NAMES, ARCH_NAMES_REV};

/// Get the architecture constant by the human name.
pub fn get_arch_by_name(arch_name: &str) -> Option<i32> {
    ARCH_NAMES_REV.get(arch_name).cloned()
}

/// Get the architecture name by the constant.
pub fn get_arch_by_id(arch_id: i32) -> Option<&'static str> {
    ARCH_NAMES.get(&arch_id).cloned()
}


pub fn get_current_arch() -> Option<&'static str> {
    None
}

pub fn get_arch_modules(arch: Option<i32>) ->  Vec<Rc<dyn ArchitectureModule>>{
    vec![]
}