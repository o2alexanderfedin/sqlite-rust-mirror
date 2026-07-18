use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Hash {
    pub(crate) htsize: u32,
    pub(crate) count: u32,
    pub(crate) first: *mut HashElem,
    pub(crate) ht: *mut Ht,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct HashElem {
    pub(crate) next: *mut HashElem,
    pub(crate) prev: *mut HashElem,
    pub(crate) data: *mut (),
    pub(crate) p_key: *const i8,
    pub(crate) h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Ht {
    pub(crate) count: u32,
    pub(crate) chain: *mut HashElem,
}
