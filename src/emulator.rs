use crate::constants::Endianess;
use crate::memory::MemoryObject;
use crate::registers::RegisterContext;

/// The emulator should be able to
/// be extended for the architecutures which are included
/// in the envi framework.  You *must* mix in
/// an instance of your architecture abstraction module.
///
/// (NOTE: Most users will just use an arch mod and call getEmulator())
///
/// The intention is for "light weight" emulation to be
/// implemented mostly for user-space emulation of
/// protected mode execution.
pub trait Emulator: RegisterContext + MemoryObject {
    /// Initialize an emulator option used by the emulator type.
    /// Arch specific options should begin with <arch>: and platform
    /// options should begin with <platform>:
    fn init_emu_opts(&self, opt: &str, def_val: &str, doc: &str);
    
    fn set_emu_opt(&self, key: &str, val: &str);
    
    fn get_emu_opt(&self, key: &str) -> Option<&str>;
    
    fn set_endian(&mut self, endian: Endianess);
    
    fn get_endian(&self) -> Endianess;
    
}