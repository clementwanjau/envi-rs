use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MetaRegister {
    pub name: String,
    pub index: i32,
    pub size: i32,
    pub shift_offset: i32,
}

#[derive(Clone, Debug)]
pub struct StatusMetaRegister {
    pub name: String,
    pub index: i32,
    pub size: i32,
    pub shift_offset: i32,
    pub description: String
}

#[derive(Clone, Debug)]
pub struct RegisterContextData {
    _rctx_vals: Vec<i32>,
    _rctx_dirty: bool,
    _rctx_pc_index: i32,
    _rctx_sp_index: i32,
    _rctx_sr_index: Option<i32>,
    _rctx_reg_def: Vec<(String, i32)>,
    _rctx_reg_metas: Vec<MetaRegister>,
    _rctx_stat_metas: Option<Vec<StatusMetaRegister>>,
}


pub trait RegisterContext{
    fn get_data(&self) -> &RegisterContextData;
    fn get_register_snap(&self) -> Vec<i32>;
    
    fn is_dirty(&self) -> bool;
    
    fn set_is_dirty(&self, dirty: bool);
    
    fn set_register_indexes(&self, pc_index: i32, sp_index: i32, sr_index: Option<i32>);
    
    fn load_reg_def(&self, reg_def: Vec<(String, i32)>, defval: Option<i32>);
    
    fn get_reg_def(&self) -> Vec<(String, i32)>;
    
    fn load_reg_metas(&self, reg_metas: Vec<MetaRegister>, stat_metas: Option<Vec<StatusMetaRegister>>);
    
    fn add_meta_register(&self, reg_meta: MetaRegister);
    
    fn is_meta_register(&self, index: i32) -> bool {
        (index & 0xffff) != index
    }
    
    fn get_register_info(&self, meta: Option<bool>) -> (i32, Vec<MetaRegister>, i32, i32, Vec<i32>);
    
    fn set_register_info(&self, reg_info: (i32, Vec<MetaRegister>, i32, i32, Vec<i32>));
    
    fn get_register_name(&self, index: i32) -> String;
    
    fn get_program_counter(&self) -> i32;
    
    fn set_program_counter(&self, pc: i32);
    
    fn get_stack_counter(&self) -> i32;
    
    fn set_stack_counter(&self, sp: i32);
    
    fn has_status_register(&self) -> bool;
    
    /// Return a list of status register names and descriptions.
    fn get_status_reg_name_desc(&self) -> Option<(String, String)>;
    
    fn get_register_names(&self) -> Vec<String>;
    
    fn get_registers(&self) -> HashMap<&str, i32>;
    
    fn get_register(&self, index: i32) -> i32{
        let rindx = index & 0xffff;
        let data = self.get_data();
        let mut value = data._rctx_vals[rindx as usize];
        if rindx != index {
            value = self._xlate_to_meta_reg(index, value);
        }
        value
    }
    
    fn _xlate_to_meta_reg(&self, index: i32, mut value: i32) -> i32 {
        let rindx = index & 0xffff;
        let offset = (index >> 24) & 0xff;
        let width = (index >>16) & 0xff;
        
        let mask = 2i32.pow(width as u32) - 1;
        if offset != 0 {
            value >>= offset;
        }
        value& mask
    }
}