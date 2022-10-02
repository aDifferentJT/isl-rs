// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Aff, BasicMap, Context, DimType, Id, Space};
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_local_space`.
pub struct LocalSpace {
    pub ptr: uintptr_t,
}

extern "C" {

    fn isl_local_space_get_ctx(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_from_space(space: uintptr_t) -> uintptr_t;

    fn isl_local_space_copy(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_free(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_is_params(ls: uintptr_t) -> i32;

    fn isl_local_space_is_set(ls: uintptr_t) -> i32;

    fn isl_local_space_set_tuple_id(ls: uintptr_t, type_: DimType, id: uintptr_t) -> uintptr_t;

    fn isl_local_space_dim(ls: uintptr_t, type_: DimType) -> i32;

    fn isl_local_space_has_dim_name(ls: uintptr_t, type_: DimType, pos: u32) -> i32;

    fn isl_local_space_get_dim_name(ls: uintptr_t, type_: DimType, pos: u32) -> *const c_char;

    fn isl_local_space_set_dim_name(ls: uintptr_t, type_: DimType, pos: u32, s: *const c_char)
                                    -> uintptr_t;

    fn isl_local_space_has_dim_id(ls: uintptr_t, type_: DimType, pos: u32) -> i32;

    fn isl_local_space_get_dim_id(ls: uintptr_t, type_: DimType, pos: u32) -> uintptr_t;

    fn isl_local_space_set_dim_id(ls: uintptr_t, type_: DimType, pos: u32, id: uintptr_t)
                                  -> uintptr_t;

    fn isl_local_space_get_space(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_get_div(ls: uintptr_t, pos: i32) -> uintptr_t;

    fn isl_local_space_find_dim_by_name(ls: uintptr_t, type_: DimType, name: *const c_char) -> i32;

    fn isl_local_space_domain(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_range(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_from_domain(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_add_dims(ls: uintptr_t, type_: DimType, n: u32) -> uintptr_t;

    fn isl_local_space_drop_dims(ls: uintptr_t, type_: DimType, first: u32, n: u32) -> uintptr_t;

    fn isl_local_space_insert_dims(ls: uintptr_t, type_: DimType, first: u32, n: u32) -> uintptr_t;

    fn isl_local_space_set_from_params(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_intersect(ls1: uintptr_t, ls2: uintptr_t) -> uintptr_t;

    fn isl_local_space_wrap(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_is_equal(ls1: uintptr_t, ls2: uintptr_t) -> i32;

    fn isl_local_space_lifting(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_flatten_domain(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_flatten_range(ls: uintptr_t) -> uintptr_t;

    fn isl_local_space_dump(ls: uintptr_t);

}

impl LocalSpace {
    /// Wraps `isl_local_space_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_get_ctx(ls) };
        let isl_rs_result = Context { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_from_space`.
    pub fn from_space(space: Space) -> LocalSpace {
        let space = space.ptr;
        let isl_rs_result = unsafe { isl_local_space_from_space(space) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_copy`.
    pub fn copy(&self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_copy(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_free`.
    pub fn free(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_free(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_is_params`.
    pub fn is_params(&self) -> bool {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_is_params(ls) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_local_space_is_set`.
    pub fn is_set(&self) -> bool {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_is_set(ls) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_local_space_set_tuple_id`.
    pub fn set_tuple_id(self, type_: DimType, id: Id) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let id = id.ptr;
        let isl_rs_result = unsafe { isl_local_space_set_tuple_id(ls, type_, id) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_dim`.
    pub fn dim(&self, type_: DimType) -> i32 {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_dim(ls, type_) };
        isl_rs_result
    }

    /// Wraps `isl_local_space_has_dim_name`.
    pub fn has_dim_name(&self, type_: DimType, pos: u32) -> bool {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_has_dim_name(ls, type_, pos) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_local_space_get_dim_name`.
    pub fn get_dim_name(&self, type_: DimType, pos: u32) -> &str {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_get_dim_name(ls, type_, pos) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Wraps `isl_local_space_set_dim_name`.
    pub fn set_dim_name(self, type_: DimType, pos: u32, s: &str) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let s = CString::new(s).unwrap();
        let s = s.as_ptr();
        let isl_rs_result = unsafe { isl_local_space_set_dim_name(ls, type_, pos, s) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_has_dim_id`.
    pub fn has_dim_id(&self, type_: DimType, pos: u32) -> bool {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_has_dim_id(ls, type_, pos) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_local_space_get_dim_id`.
    pub fn get_dim_id(&self, type_: DimType, pos: u32) -> Id {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_get_dim_id(ls, type_, pos) };
        let isl_rs_result = Id { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_set_dim_id`.
    pub fn set_dim_id(self, type_: DimType, pos: u32, id: Id) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let id = id.ptr;
        let isl_rs_result = unsafe { isl_local_space_set_dim_id(ls, type_, pos, id) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_get_space`.
    pub fn get_space(&self) -> Space {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_get_space(ls) };
        let isl_rs_result = Space { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_get_div`.
    pub fn get_div(&self, pos: i32) -> Aff {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_get_div(ls, pos) };
        let isl_rs_result = Aff { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_find_dim_by_name`.
    pub fn find_dim_by_name(&self, type_: DimType, name: &str) -> i32 {
        let ls = self;
        let ls = ls.ptr;
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let isl_rs_result = unsafe { isl_local_space_find_dim_by_name(ls, type_, name) };
        isl_rs_result
    }

    /// Wraps `isl_local_space_domain`.
    pub fn domain(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_domain(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_range`.
    pub fn range(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_range(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_from_domain`.
    pub fn from_domain(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_from_domain(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_add_dims`.
    pub fn add_dims(self, type_: DimType, n: u32) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_add_dims(ls, type_, n) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_drop_dims`.
    pub fn drop_dims(self, type_: DimType, first: u32, n: u32) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_drop_dims(ls, type_, first, n) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_insert_dims`.
    pub fn insert_dims(self, type_: DimType, first: u32, n: u32) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_insert_dims(ls, type_, first, n) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_set_from_params`.
    pub fn set_from_params(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_set_from_params(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_intersect`.
    pub fn intersect(self, ls2: LocalSpace) -> LocalSpace {
        let ls1 = self;
        let ls1 = ls1.ptr;
        let ls2 = ls2.ptr;
        let isl_rs_result = unsafe { isl_local_space_intersect(ls1, ls2) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_wrap`.
    pub fn wrap(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_wrap(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_is_equal`.
    pub fn is_equal(&self, ls2: &LocalSpace) -> bool {
        let ls1 = self;
        let ls1 = ls1.ptr;
        let ls2 = ls2.ptr;
        let isl_rs_result = unsafe { isl_local_space_is_equal(ls1, ls2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_local_space_lifting`.
    pub fn lifting(self) -> BasicMap {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_lifting(ls) };
        let isl_rs_result = BasicMap { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_flatten_domain`.
    pub fn flatten_domain(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_flatten_domain(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_flatten_range`.
    pub fn flatten_range(self) -> LocalSpace {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_flatten_range(ls) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result };
        isl_rs_result
    }

    /// Wraps `isl_local_space_dump`.
    pub fn dump(&self) {
        let ls = self;
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_local_space_dump(ls) };
        isl_rs_result
    }
}

impl Drop for LocalSpace {
    fn drop(&mut self) {
        unsafe {
            isl_local_space_free(self.ptr);
        }
    }
}
