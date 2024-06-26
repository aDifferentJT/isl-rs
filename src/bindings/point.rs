// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Context, DimType, MultiVal, Space, Val};
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_point`.
pub struct Point {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_point_add_ui(pnt: uintptr_t, type_: DimType, pos: i32, val: u32) -> uintptr_t;

    fn isl_point_get_coordinate_val(pnt: uintptr_t, type_: DimType, pos: i32) -> uintptr_t;

    fn isl_point_copy(pnt: uintptr_t) -> uintptr_t;

    fn isl_point_void(space: uintptr_t) -> uintptr_t;

    fn isl_point_is_void(pnt: uintptr_t) -> i32;

    fn isl_point_to_str(pnt: uintptr_t) -> *const c_char;

    fn isl_point_zero(space: uintptr_t) -> uintptr_t;

    fn isl_point_dump(pnt: uintptr_t);

    fn isl_point_get_space(pnt: uintptr_t) -> uintptr_t;

    fn isl_point_sub_ui(pnt: uintptr_t, type_: DimType, pos: i32, val: u32) -> uintptr_t;

    fn isl_point_get_ctx(pnt: uintptr_t) -> uintptr_t;

    fn isl_point_free(pnt: uintptr_t) -> uintptr_t;

    fn isl_point_set_coordinate_val(pnt: uintptr_t, type_: DimType, pos: i32, v: uintptr_t)
                                    -> uintptr_t;

    fn isl_point_get_multi_val(pnt: uintptr_t) -> uintptr_t;

}

impl Clone for Point {
    fn clone(&self) -> Point {
        self.copy()
    }
}

impl Point {
    /// Wraps `isl_point_add_ui`.
    pub fn add_ui(self, type_: DimType, pos: i32, val: u32) -> Point {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let mut pnt = pnt;
        pnt.do_not_free_on_drop();
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_add_ui(pnt, type_, pos, val) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_get_coordinate_val`.
    pub fn get_coordinate_val(&self, type_: DimType, pos: i32) -> Val {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_get_coordinate_val(pnt, type_, pos) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_copy`.
    pub fn copy(&self) -> Point {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_copy(pnt) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_void`.
    pub fn void(space: Space) -> Point {
        let mut space = space;
        space.do_not_free_on_drop();
        let space = space.ptr;
        let isl_rs_result = unsafe { isl_point_void(space) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_is_void`.
    pub fn is_void(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_is_void(pnt) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_point_to_str`.
    pub fn to_str(&self) -> &str {
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_to_str(pnt) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Wraps `isl_point_zero`.
    pub fn zero(space: Space) -> Point {
        let mut space = space;
        space.do_not_free_on_drop();
        let space = space.ptr;
        let isl_rs_result = unsafe { isl_point_zero(space) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_dump`.
    pub fn dump(&self) {
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_dump(pnt) };
        isl_rs_result
    }

    /// Wraps `isl_point_get_space`.
    pub fn get_space(&self) -> Space {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_get_space(pnt) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_sub_ui`.
    pub fn sub_ui(self, type_: DimType, pos: i32, val: u32) -> Point {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let mut pnt = pnt;
        pnt.do_not_free_on_drop();
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_sub_ui(pnt, type_, pos, val) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_get_ctx(pnt) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_point_free`.
    pub fn free(self) -> Point {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let mut pnt = pnt;
        pnt.do_not_free_on_drop();
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_free(pnt) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_set_coordinate_val`.
    pub fn set_coordinate_val(self, type_: DimType, pos: i32, v: Val) -> Point {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let mut pnt = pnt;
        pnt.do_not_free_on_drop();
        let pnt = pnt.ptr;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_point_set_coordinate_val(pnt, type_, pos, v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Point { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_point_get_multi_val`.
    pub fn get_multi_val(&self) -> MultiVal {
        let context_for_error_message = self.get_ctx();
        let pnt = self;
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_point_get_multi_val(pnt) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = MultiVal { ptr: isl_rs_result,
                                       should_free_on_drop: true };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_point_free(self.ptr);
            }
        }
    }
}
