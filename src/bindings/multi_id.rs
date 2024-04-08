// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Context, Id, IdList, Space};
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_multi_id`.
pub struct MultiId {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_multi_id_flat_range_product(multi1: uintptr_t, multi2: uintptr_t) -> uintptr_t;

    fn isl_multi_id_reset_user(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_set_at(multi: uintptr_t, pos: i32, el: uintptr_t) -> uintptr_t;

    fn isl_multi_id_get_list(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_range_product(multi1: uintptr_t, multi2: uintptr_t) -> uintptr_t;

    fn isl_multi_id_to_str(mi: uintptr_t) -> *const c_char;

    fn isl_multi_id_plain_is_equal(multi1: uintptr_t, multi2: uintptr_t) -> i32;

    fn isl_multi_id_dump(mi: uintptr_t);

    fn isl_multi_id_free(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_get_ctx(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_get_space(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_get_at(multi: uintptr_t, pos: i32) -> uintptr_t;

    fn isl_multi_id_flatten_range(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_range_is_wrapping(multi: uintptr_t) -> i32;

    fn isl_multi_id_get_id(multi: uintptr_t, pos: i32) -> uintptr_t;

    fn isl_multi_id_range_factor_domain(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_size(multi: uintptr_t) -> i32;

    fn isl_multi_id_get_domain_space(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_set_id(multi: uintptr_t, pos: i32, el: uintptr_t) -> uintptr_t;

    fn isl_multi_id_factor_range(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_range_factor_range(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_from_id_list(space: uintptr_t, list: uintptr_t) -> uintptr_t;

    fn isl_multi_id_align_params(multi: uintptr_t, model: uintptr_t) -> uintptr_t;

    fn isl_multi_id_from_range(multi: uintptr_t) -> uintptr_t;

    fn isl_multi_id_read_from_str(ctx: uintptr_t, str_: *const c_char) -> uintptr_t;

    fn isl_multi_id_range_splice(multi1: uintptr_t, pos: u32, multi2: uintptr_t) -> uintptr_t;

    fn isl_multi_id_copy(multi: uintptr_t) -> uintptr_t;

}

impl Clone for MultiId {
    fn clone(&self) -> MultiId {
        self.copy()
    }
}

impl MultiId {
    /// Wraps `isl_multi_id_flat_range_product`.
    pub fn flat_range_product(self, multi2: MultiId) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi1 = self;
        let mut multi1 = multi1;
        multi1.do_not_free_on_drop();
        let multi1 = multi1.ptr;
        let mut multi2 = multi2;
        multi2.do_not_free_on_drop();
        let multi2 = multi2.ptr;
        let isl_rs_result = unsafe { isl_multi_id_flat_range_product(multi1, multi2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_reset_user`.
    pub fn reset_user(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_reset_user(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_set_at`.
    pub fn set_at(self, pos: i32, el: Id) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_multi_id_set_at(multi, pos, el) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_get_list`.
    pub fn get_list(&self) -> IdList {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_get_list(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = IdList { ptr: isl_rs_result,
                                     should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_range_product`.
    pub fn range_product(self, multi2: MultiId) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi1 = self;
        let mut multi1 = multi1;
        multi1.do_not_free_on_drop();
        let multi1 = multi1.ptr;
        let mut multi2 = multi2;
        multi2.do_not_free_on_drop();
        let multi2 = multi2.ptr;
        let isl_rs_result = unsafe { isl_multi_id_range_product(multi1, multi2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_to_str`.
    pub fn to_str(&self) -> &str {
        let mi = self;
        let mi = mi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_to_str(mi) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Wraps `isl_multi_id_plain_is_equal`.
    pub fn plain_is_equal(&self, multi2: &MultiId) -> bool {
        let context_for_error_message = self.get_ctx();
        let multi1 = self;
        let multi1 = multi1.ptr;
        let multi2 = multi2.ptr;
        let isl_rs_result = unsafe { isl_multi_id_plain_is_equal(multi1, multi2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_dump`.
    pub fn dump(&self) {
        let mi = self;
        let mi = mi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_dump(mi) };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_free`.
    pub fn free(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_free(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_get_ctx(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_multi_id_get_space`.
    pub fn get_space(&self) -> Space {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_get_space(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_get_at`.
    pub fn get_at(&self, pos: i32) -> Id {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_get_at(multi, pos) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Id { ptr: isl_rs_result,
                                 should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_flatten_range`.
    pub fn flatten_range(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_flatten_range(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_range_is_wrapping`.
    pub fn range_is_wrapping(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_range_is_wrapping(multi) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_get_id`.
    pub fn get_id(&self, pos: i32) -> Id {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_get_id(multi, pos) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Id { ptr: isl_rs_result,
                                 should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_range_factor_domain`.
    pub fn range_factor_domain(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_range_factor_domain(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_size`.
    pub fn size(&self) -> i32 {
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_size(multi) };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_get_domain_space`.
    pub fn get_domain_space(&self) -> Space {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_get_domain_space(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_set_id`.
    pub fn set_id(self, pos: i32, el: Id) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_multi_id_set_id(multi, pos, el) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_factor_range`.
    pub fn factor_range(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_factor_range(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_range_factor_range`.
    pub fn range_factor_range(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_range_factor_range(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_from_id_list`.
    pub fn from_id_list(space: Space, list: IdList) -> MultiId {
        let mut space = space;
        space.do_not_free_on_drop();
        let space = space.ptr;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_multi_id_from_id_list(space, list) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_align_params`.
    pub fn align_params(self, model: Space) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let mut model = model;
        model.do_not_free_on_drop();
        let model = model.ptr;
        let isl_rs_result = unsafe { isl_multi_id_align_params(multi, model) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_from_range`.
    pub fn from_range(self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let mut multi = multi;
        multi.do_not_free_on_drop();
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_from_range(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_read_from_str`.
    pub fn read_from_str(ctx: &Context, str_: &str) -> MultiId {
        let ctx = ctx.ptr;
        let str_ = CString::new(str_).unwrap();
        let str_ = str_.as_ptr();
        let isl_rs_result = unsafe { isl_multi_id_read_from_str(ctx, str_) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_range_splice`.
    pub fn range_splice(self, pos: u32, multi2: MultiId) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi1 = self;
        let mut multi1 = multi1;
        multi1.do_not_free_on_drop();
        let multi1 = multi1.ptr;
        let mut multi2 = multi2;
        multi2.do_not_free_on_drop();
        let multi2 = multi2.ptr;
        let isl_rs_result = unsafe { isl_multi_id_range_splice(multi1, pos, multi2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_multi_id_copy`.
    pub fn copy(&self) -> MultiId {
        let context_for_error_message = self.get_ctx();
        let multi = self;
        let multi = multi.ptr;
        let isl_rs_result = unsafe { isl_multi_id_copy(multi) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiId { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for MultiId {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_multi_id_free(self.ptr);
            }
        }
    }
}
