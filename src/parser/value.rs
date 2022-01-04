
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::copy_nonoverlapping as copy;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use crate::*;

//* NOTE(IMPORTANT): DON'T DERIVE CLONE, IT WILL FREE TWICE AND WILL CREATE TWO POINTERS TO THE SAME DATA
pub(crate) struct Value<'a> {
    pub(crate) sname: &'a str,
    pub(crate) data: (*mut u8, Layout),
}

impl<'a> Value<'a> {
    
    pub(crate) fn make<const S: usize>(sname: &'a str, srct: &Structure, content: [u8; S]) -> Self {
        assert!(content.len() == srct.size as usize);
        let layout = Layout::from_size_align(srct.size as usize, srct.align as usize).aborts("Invalid memory layout.");
        let data = unsafe { alloc(layout) };
        unsafe { copy(content.as_ptr(), data, srct.size as usize) };
        Self { sname, data: (data, layout) }
    }

    // pub(crate) fn view(&self, field: usize) -> u64 {
        
    // }

    pub(crate) fn view(&self) -> u64 {
        unsafe { *(self.data.0 as *const u64) }
    }

}

impl Debug for Value<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_str("Value{..}")
    }
}

impl Drop for Value <'_>{
    fn drop(&mut self) {
        unsafe { dealloc(self.data.0, self.data.1) }
    }
}
