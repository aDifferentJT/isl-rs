// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Context, Val};
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_val_list`.
pub struct ValList {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_val_list_get_ctx(list: uintptr_t) -> uintptr_t;

    fn isl_val_list_from_val(el: uintptr_t) -> uintptr_t;

    fn isl_val_list_alloc(ctx: uintptr_t, n: i32) -> uintptr_t;

    fn isl_val_list_copy(list: uintptr_t) -> uintptr_t;

    fn isl_val_list_free(list: uintptr_t) -> uintptr_t;

    fn isl_val_list_add(list: uintptr_t, el: uintptr_t) -> uintptr_t;

    fn isl_val_list_insert(list: uintptr_t, pos: u32, el: uintptr_t) -> uintptr_t;

    fn isl_val_list_drop(list: uintptr_t, first: u32, n: u32) -> uintptr_t;

    fn isl_val_list_clear(list: uintptr_t) -> uintptr_t;

    fn isl_val_list_swap(list: uintptr_t, pos1: u32, pos2: u32) -> uintptr_t;

    fn isl_val_list_reverse(list: uintptr_t) -> uintptr_t;

    fn isl_val_list_concat(list1: uintptr_t, list2: uintptr_t) -> uintptr_t;

    fn isl_val_list_size(list: uintptr_t) -> i32;

    fn isl_val_list_n_val(list: uintptr_t) -> i32;

    fn isl_val_list_get_at(list: uintptr_t, index: i32) -> uintptr_t;

    fn isl_val_list_get_val(list: uintptr_t, index: i32) -> uintptr_t;

    fn isl_val_list_set_at(list: uintptr_t, index: i32, el: uintptr_t) -> uintptr_t;

    fn isl_val_list_set_val(list: uintptr_t, index: i32, el: uintptr_t) -> uintptr_t;

    fn isl_val_list_to_str(list: uintptr_t) -> *const c_char;

    fn isl_val_list_dump(list: uintptr_t);

    fn isl_val_list_read_from_str(ctx: uintptr_t, str_: *const c_char) -> uintptr_t;

}

impl Clone for ValList {
    fn clone(&self) -> ValList {
        self.copy()
    }
}

impl FromIterator<Val> for ValList {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = Val>
    {
        let mut iter = iter.into_iter().peekable();
        let ctx = iter.peek().unwrap().get_ctx();
        let (size, _) = iter.size_hint();
        let mut res = Self::alloc(&ctx, size as _);
        for x in iter {
            res = res.add(x);
        }
        res
    }
}

impl ValList {
    /// Wraps `isl_val_list_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_get_ctx(list) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_val_list_from_val`.
    pub fn from_val(el: Val) -> ValList {
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_val_list_from_val(el) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_alloc`.
    pub fn alloc(ctx: &Context, n: i32) -> ValList {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_list_alloc(ctx, n) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_copy`.
    pub fn copy(&self) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_copy(list) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_free`.
    pub fn free(self) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_free(list) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_add`.
    pub fn add(self, el: Val) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_val_list_add(list, el) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_insert`.
    pub fn insert(self, pos: u32, el: Val) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_val_list_insert(list, pos, el) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_drop`.
    pub fn drop(self, first: u32, n: u32) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_drop(list, first, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_clear`.
    pub fn clear(self) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_clear(list) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_swap`.
    pub fn swap(self, pos1: u32, pos2: u32) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_swap(list, pos1, pos2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_reverse`.
    pub fn reverse(self) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_reverse(list) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_concat`.
    pub fn concat(self, list2: ValList) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list1 = self;
        let mut list1 = list1;
        list1.do_not_free_on_drop();
        let list1 = list1.ptr;
        let mut list2 = list2;
        list2.do_not_free_on_drop();
        let list2 = list2.ptr;
        let isl_rs_result = unsafe { isl_val_list_concat(list1, list2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_size`.
    pub fn size(&self) -> i32 {
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_size(list) };
        isl_rs_result
    }

    /// Wraps `isl_val_list_n_val`.
    pub fn n_val(&self) -> i32 {
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_n_val(list) };
        isl_rs_result
    }

    /// Wraps `isl_val_list_get_at`.
    pub fn get_at(&self, index: i32) -> Val {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_get_at(list, index) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_get_val`.
    pub fn get_val(&self, index: i32) -> Val {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_get_val(list, index) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_set_at`.
    pub fn set_at(self, index: i32, el: Val) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_val_list_set_at(list, index, el) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_set_val`.
    pub fn set_val(self, index: i32, el: Val) -> ValList {
        let context_for_error_message = self.get_ctx();
        let list = self;
        let mut list = list;
        list.do_not_free_on_drop();
        let list = list.ptr;
        let mut el = el;
        el.do_not_free_on_drop();
        let el = el.ptr;
        let isl_rs_result = unsafe { isl_val_list_set_val(list, index, el) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_list_to_str`.
    pub fn to_str(&self) -> &str {
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_to_str(list) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Wraps `isl_val_list_dump`.
    pub fn dump(&self) {
        let list = self;
        let list = list.ptr;
        let isl_rs_result = unsafe { isl_val_list_dump(list) };
        isl_rs_result
    }

    /// Wraps `isl_val_list_read_from_str`.
    pub fn read_from_str(ctx: &Context, str_: &str) -> ValList {
        let ctx = ctx.ptr;
        let str_ = CString::new(str_).unwrap();
        let str_ = str_.as_ptr();
        let isl_rs_result = unsafe { isl_val_list_read_from_str(ctx, str_) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = ValList { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for ValList {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_val_list_free(self.ptr);
            }
        }
    }
}
