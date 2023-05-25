// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Aff, Context, DimType, LocalSpace, Space, Val};
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_constraint`.
pub struct Constraint {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_constraint_get_ctx(c: uintptr_t) -> uintptr_t;

    fn isl_constraint_alloc_equality(ls: uintptr_t) -> uintptr_t;

    fn isl_constraint_alloc_inequality(ls: uintptr_t) -> uintptr_t;

    fn isl_constraint_copy(c: uintptr_t) -> uintptr_t;

    fn isl_constraint_free(c: uintptr_t) -> uintptr_t;

    fn isl_constraint_is_equal(constraint1: uintptr_t, constraint2: uintptr_t) -> i32;

    fn isl_constraint_get_space(constraint: uintptr_t) -> uintptr_t;

    fn isl_constraint_get_local_space(constraint: uintptr_t) -> uintptr_t;

    fn isl_constraint_dim(constraint: uintptr_t, type_: DimType) -> i32;

    fn isl_constraint_involves_dims(constraint: uintptr_t, type_: DimType, first: u32, n: u32)
                                    -> i32;

    fn isl_constraint_get_dim_name(constraint: uintptr_t, type_: DimType, pos: u32)
                                   -> *const c_char;

    fn isl_constraint_get_constant_val(constraint: uintptr_t) -> uintptr_t;

    fn isl_constraint_get_coefficient_val(constraint: uintptr_t, type_: DimType, pos: i32)
                                          -> uintptr_t;

    fn isl_constraint_set_constant_si(constraint: uintptr_t, v: i32) -> uintptr_t;

    fn isl_constraint_set_constant_val(constraint: uintptr_t, v: uintptr_t) -> uintptr_t;

    fn isl_constraint_set_coefficient_si(constraint: uintptr_t, type_: DimType, pos: i32, v: i32)
                                         -> uintptr_t;

    fn isl_constraint_set_coefficient_val(constraint: uintptr_t, type_: DimType, pos: i32,
                                          v: uintptr_t)
                                          -> uintptr_t;

    fn isl_constraint_get_div(constraint: uintptr_t, pos: i32) -> uintptr_t;

    fn isl_constraint_negate(constraint: uintptr_t) -> uintptr_t;

    fn isl_constraint_is_equality(constraint: uintptr_t) -> i32;

    fn isl_constraint_is_div_constraint(constraint: uintptr_t) -> i32;

    fn isl_constraint_is_lower_bound(constraint: uintptr_t, type_: DimType, pos: u32) -> i32;

    fn isl_constraint_is_upper_bound(constraint: uintptr_t, type_: DimType, pos: u32) -> i32;

    fn isl_constraint_get_bound(constraint: uintptr_t, type_: DimType, pos: i32) -> uintptr_t;

    fn isl_constraint_get_aff(constraint: uintptr_t) -> uintptr_t;

    fn isl_constraint_plain_cmp(c1: uintptr_t, c2: uintptr_t) -> i32;

    fn isl_constraint_cmp_last_non_zero(c1: uintptr_t, c2: uintptr_t) -> i32;

    fn isl_constraint_dump(c: uintptr_t);

}

impl Constraint {
    /// Wraps `isl_constraint_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let c = self;
        let c = c.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_ctx(c) };
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_constraint_alloc_equality`.
    pub fn alloc_equality(ls: LocalSpace) -> Constraint {
        let mut ls = ls;
        ls.do_not_free_on_drop();
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_constraint_alloc_equality(ls) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_alloc_inequality`.
    pub fn alloc_inequality(ls: LocalSpace) -> Constraint {
        let mut ls = ls;
        ls.do_not_free_on_drop();
        let ls = ls.ptr;
        let isl_rs_result = unsafe { isl_constraint_alloc_inequality(ls) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_copy`.
    pub fn copy(&self) -> Constraint {
        let c = self;
        let c = c.ptr;
        let isl_rs_result = unsafe { isl_constraint_copy(c) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_free`.
    pub fn free(self) -> Constraint {
        let c = self;
        let mut c = c;
        c.do_not_free_on_drop();
        let c = c.ptr;
        let isl_rs_result = unsafe { isl_constraint_free(c) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_is_equal`.
    pub fn is_equal(&self, constraint2: &Constraint) -> i32 {
        let constraint1 = self;
        let constraint1 = constraint1.ptr;
        let constraint2 = constraint2.ptr;
        let isl_rs_result = unsafe { isl_constraint_is_equal(constraint1, constraint2) };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_space`.
    pub fn get_space(&self) -> Space {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_space(constraint) };
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_local_space`.
    pub fn get_local_space(&self) -> LocalSpace {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_local_space(constraint) };
        let isl_rs_result = LocalSpace { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_dim`.
    pub fn dim(&self, type_: DimType) -> i32 {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_dim(constraint, type_) };
        isl_rs_result
    }

    /// Wraps `isl_constraint_involves_dims`.
    pub fn involves_dims(&self, type_: DimType, first: u32, n: u32) -> bool {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_involves_dims(constraint, type_, first, n) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_dim_name`.
    pub fn get_dim_name(&self, type_: DimType, pos: u32) -> &str {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_dim_name(constraint, type_, pos) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_constant_val`.
    pub fn get_constant_val(&self) -> Val {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_constant_val(constraint) };
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_coefficient_val`.
    pub fn get_coefficient_val(&self, type_: DimType, pos: i32) -> Val {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_coefficient_val(constraint, type_, pos) };
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_set_constant_si`.
    pub fn set_constant_si(self, v: i32) -> Constraint {
        let constraint = self;
        let mut constraint = constraint;
        constraint.do_not_free_on_drop();
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_set_constant_si(constraint, v) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_set_constant_val`.
    pub fn set_constant_val(self, v: Val) -> Constraint {
        let constraint = self;
        let mut constraint = constraint;
        constraint.do_not_free_on_drop();
        let constraint = constraint.ptr;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_constraint_set_constant_val(constraint, v) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_set_coefficient_si`.
    pub fn set_coefficient_si(self, type_: DimType, pos: i32, v: i32) -> Constraint {
        let constraint = self;
        let mut constraint = constraint;
        constraint.do_not_free_on_drop();
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_set_coefficient_si(constraint, type_, pos, v) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_set_coefficient_val`.
    pub fn set_coefficient_val(self, type_: DimType, pos: i32, v: Val) -> Constraint {
        let constraint = self;
        let mut constraint = constraint;
        constraint.do_not_free_on_drop();
        let constraint = constraint.ptr;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result =
            unsafe { isl_constraint_set_coefficient_val(constraint, type_, pos, v) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_div`.
    pub fn get_div(&self, pos: i32) -> Aff {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_div(constraint, pos) };
        let isl_rs_result = Aff { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_negate`.
    pub fn negate(self) -> Constraint {
        let constraint = self;
        let mut constraint = constraint;
        constraint.do_not_free_on_drop();
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_negate(constraint) };
        let isl_rs_result = Constraint { ptr: isl_rs_result,
                                         should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_is_equality`.
    pub fn is_equality(&self) -> bool {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_is_equality(constraint) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_constraint_is_div_constraint`.
    pub fn is_div_constraint(&self) -> bool {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_is_div_constraint(constraint) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_constraint_is_lower_bound`.
    pub fn is_lower_bound(&self, type_: DimType, pos: u32) -> bool {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_is_lower_bound(constraint, type_, pos) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_constraint_is_upper_bound`.
    pub fn is_upper_bound(&self, type_: DimType, pos: u32) -> bool {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_is_upper_bound(constraint, type_, pos) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("Got isl_bool = -1"),
        };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_bound`.
    pub fn get_bound(&self, type_: DimType, pos: i32) -> Aff {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_bound(constraint, type_, pos) };
        let isl_rs_result = Aff { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_get_aff`.
    pub fn get_aff(&self) -> Aff {
        let constraint = self;
        let constraint = constraint.ptr;
        let isl_rs_result = unsafe { isl_constraint_get_aff(constraint) };
        let isl_rs_result = Aff { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_constraint_plain_cmp`.
    pub fn plain_cmp(&self, c2: &Constraint) -> i32 {
        let c1 = self;
        let c1 = c1.ptr;
        let c2 = c2.ptr;
        let isl_rs_result = unsafe { isl_constraint_plain_cmp(c1, c2) };
        isl_rs_result
    }

    /// Wraps `isl_constraint_cmp_last_non_zero`.
    pub fn cmp_last_non_zero(&self, c2: &Constraint) -> i32 {
        let c1 = self;
        let c1 = c1.ptr;
        let c2 = c2.ptr;
        let isl_rs_result = unsafe { isl_constraint_cmp_last_non_zero(c1, c2) };
        isl_rs_result
    }

    /// Wraps `isl_constraint_dump`.
    pub fn dump(&self) {
        let c = self;
        let c = c.ptr;
        let isl_rs_result = unsafe { isl_constraint_dump(c) };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for Constraint {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_constraint_free(self.ptr);
            }
        }
    }
}
