use std::rc::Rc;
use crate::ArchitectureModule;
use crate::constants::{ARCH_DEFAULT, Endianess, MM_EXEC, MM_READ, MM_SHARED, MM_WRITE};
use crate::operands::OpCode;
use crate::Result;

pub trait MemoryData {
    fn get_imem_archs(&self) -> Vec<&Rc<dyn ArchitectureModule>>;

    fn get_imem_psize(&self) -> i32;
}


/// This is the interface spec (and a few helper utils)
/// for the unified memory object interface.
///
/// NOTE: If your actual underlying memory format is such
/// that over-riding anything (like isValidPointer!) can
/// be faster than the default implementation, DO IT!
pub trait Memory{
    fn get_data(&self) -> &Rc<dyn MemoryData>;

    fn get_endian(&self) -> Endianess;

    fn set_endian(&mut self, endian: Endianess);

    fn set_mem_architecture(&mut self, arch: i32);

    /// Get a reference to the default arch module for the memory object.
    fn get_mem_arch_module(&self, arch: Option<i32>) -> &Rc<dyn ArchitectureModule> {
        let arch = arch.unwrap_or(ARCH_DEFAULT);
        self.get_data().get_imem_archs()[arch as usize]
    }

    fn get_pointer_size(&self) -> i32;

    /// Read memory from the specified virtual address for size bytes
    /// and return it as a rust String.
    ///
    /// Example: mem.read_memory(0x41414141, 20) -> "A..."
    fn read_memory(&self, addr: i32, size: i32) -> Option<String>;

    /// Write the given bytes to the specified virtual address.
    ///
    /// Example: mem.write_memory(0x41414141, "VISI".as_bytes())
    fn write_memory(&self, addr: i32, data: &[u8]);

    /// Change the protections for the given memory map. On most platforms
    /// the va/size *must* exactly match an existing memory map.
    fn protect_memory(&self, addr: i32, size: i32, perms: i32);

    /// Check to be sure that the given virtual address and size
    /// is contained within one memory map, and check that the
    /// perms are contained within the permission bits
    /// for the memory map. `(MM_READ | MM_WRITE | MM_EXEC | ...)`
    ///
    /// Example:
    /// ```ignore
    /// mem.probe_memory(0x41414141, 20, MM_WRITE) //check if the memory for 20 bytes at 0x41414141 is writable
    /// ```
    fn probe_memory(&self, va: i32, size: i32, perm: i32) -> bool {
        let mmap = self.get_memory_map(va);
        if mmap.as_ref().is_none() {
            return false;
        }
        let (map_va, map_size, map_perms, _) = mmap.unwrap();
        let map_end = map_va + map_size;
        if va + size > map_end {
            return false;
        }
        if (perm & map_perms) != perm {
            return false;
        }
        true
    }

    fn allocate_memory(&self, size: i32, perms: i32, suggest_addr: Option<i32>);

    fn add_memory_map(&self, map_va: i32, perms: i32, f_name: &str, data: Option<&[u8]>, align: Option<i32>);

    fn get_memory_maps(&self) -> Vec<(i32, i32, i32, Option<String>)>;

    fn read_memory_format(&self, _addr: i32, _fmt: &str) -> Vec<i32> {
        unimplemented!()
    }

    fn get_segment_info(&self, _va: i32) -> (i32, i64) {
        (0, 0xffffffff)
    }

    /// Read a number from memory of the given size.
    fn read_mem_value(&self, addr: i32, size: i32) -> Option<i64> {
        let bytes = self.read_memory(addr, size);
        None
    }

    /// Return a tuple of mapva,size,perms,filename for the memory
    /// map which contains the specified address (or None).
    fn get_memory_map(&self, va: i32) -> Option<(i32, i32, i32, Option<String>)> {
        for (map_va, size, perms, m_name) in self.get_memory_maps() {
            if map_va <= va && va < (map_va + size) {
                return Some((map_va, size, perms, m_name));
            }
        }
        None
    }

    fn is_valid_pointer(&self, va: i32) -> bool {
        self.get_memory_map(va).is_some()
    }

    /// Return the number of contiguous bytes that can be read from the
    /// specified va.
    fn get_max_read_size(&self, va: i32) -> i32 {
        let mut n_read= 0;
        let mut mmap = self.get_memory_map(va);
        while mmap.as_ref().is_some() {
            let (map_va, size, perms, m_name) = mmap.unwrap();
            if (perms & MM_READ) == 0 {
                break;
            }
            n_read += (map_va + size) - (va - n_read);
            mmap = self.get_memory_map(va + n_read);
        }
        n_read
    }

    fn is_readable(&self, va: i32) -> bool {
        let mmap = self.get_memory_map(va);
        if mmap.as_ref().is_none() {
            return false;
        }
        let (_, _, perms, _) = mmap.unwrap();
        (perms & MM_READ) != 0
    }

    fn is_writable(&self, va: i32) -> bool {
        let mmap = self.get_memory_map(va);
        if mmap.as_ref().is_none() {
            return false;
        }
        let (_, _, perms, _) = mmap.unwrap();
        (perms & MM_WRITE) != 0
    }

    fn is_executable(&self, va: i32) -> bool {
        let mmap = self.get_memory_map(va);
        if mmap.as_ref().is_none() {
            return false;
        }
        let (_, _, perms, _) = mmap.unwrap();
        (perms & MM_EXEC) != 0
    }

    fn is_shared(&self, va: i32) -> bool {
        let mmap = self.get_memory_map(va);
        if mmap.as_ref().is_none() {
            return false;
        }
        let (_, _, perms, _) = mmap.unwrap();
        (perms & MM_SHARED) != 0
    }

    fn parse_op_code(&self, va: Option<i32>, arch: Option<i32>) -> Result<OpCode> {
        let arch = arch.unwrap_or(ARCH_DEFAULT);
        let b = self.read_memory(va.unwrap(), 16).unwrap().as_bytes().to_vec();
        self.get_data().get_imem_archs()[arch as usize >> 16].arch_parse_opcode(b, Some(0), va)
    }
}

pub trait MemoryCache: Memory {}

pub trait MemoryObject: Memory {}