use std::collections::HashMap;

pub trait RegisterContext{
    fn get_register_names(&self) -> Vec<String>;
    
    fn get_registers(&self) -> HashMap<&str, i32>;
}