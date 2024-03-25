// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::Context;
use libc::uintptr_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Wraps `isl_val`.
pub struct Val {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_val_zero(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_one(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_negone(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_nan(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_infty(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_neginfty(ctx: uintptr_t) -> uintptr_t;

    fn isl_val_int_from_si(ctx: uintptr_t, i: i64) -> uintptr_t;

    fn isl_val_int_from_ui(ctx: uintptr_t, u: u64) -> uintptr_t;

    fn isl_val_copy(v: uintptr_t) -> uintptr_t;

    fn isl_val_free(v: uintptr_t) -> uintptr_t;

    fn isl_val_get_ctx(val: uintptr_t) -> uintptr_t;

    fn isl_val_get_hash(val: uintptr_t) -> u32;

    fn isl_val_get_num_si(v: uintptr_t) -> i64;

    fn isl_val_get_den_si(v: uintptr_t) -> i64;

    fn isl_val_get_den_val(v: uintptr_t) -> uintptr_t;

    fn isl_val_get_d(v: uintptr_t) -> f64;

    fn isl_val_n_abs_num_chunks(v: uintptr_t, size: usize) -> i32;

    fn isl_val_set_si(v: uintptr_t, i: i64) -> uintptr_t;

    fn isl_val_abs(v: uintptr_t) -> uintptr_t;

    fn isl_val_neg(v: uintptr_t) -> uintptr_t;

    fn isl_val_inv(v: uintptr_t) -> uintptr_t;

    fn isl_val_floor(v: uintptr_t) -> uintptr_t;

    fn isl_val_ceil(v: uintptr_t) -> uintptr_t;

    fn isl_val_trunc(v: uintptr_t) -> uintptr_t;

    fn isl_val_2exp(v: uintptr_t) -> uintptr_t;

    fn isl_val_pow2(v: uintptr_t) -> uintptr_t;

    fn isl_val_min(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_max(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_add(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_add_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_sub(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_sub_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_mul(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_mul_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_div(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_div_ui(v1: uintptr_t, v2: u64) -> uintptr_t;

    fn isl_val_mod(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_gcd(v1: uintptr_t, v2: uintptr_t) -> uintptr_t;

    fn isl_val_sgn(v: uintptr_t) -> i32;

    fn isl_val_is_zero(v: uintptr_t) -> i32;

    fn isl_val_is_one(v: uintptr_t) -> i32;

    fn isl_val_is_negone(v: uintptr_t) -> i32;

    fn isl_val_is_nonneg(v: uintptr_t) -> i32;

    fn isl_val_is_nonpos(v: uintptr_t) -> i32;

    fn isl_val_is_pos(v: uintptr_t) -> i32;

    fn isl_val_is_neg(v: uintptr_t) -> i32;

    fn isl_val_is_int(v: uintptr_t) -> i32;

    fn isl_val_is_rat(v: uintptr_t) -> i32;

    fn isl_val_is_nan(v: uintptr_t) -> i32;

    fn isl_val_is_infty(v: uintptr_t) -> i32;

    fn isl_val_is_neginfty(v: uintptr_t) -> i32;

    fn isl_val_cmp_si(v: uintptr_t, i: i64) -> i32;

    fn isl_val_lt(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_le(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_gt(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_gt_si(v: uintptr_t, i: i64) -> i32;

    fn isl_val_ge(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_eq(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_eq_si(v: uintptr_t, i: i64) -> i32;

    fn isl_val_ne(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_abs_eq(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_is_divisible_by(v1: uintptr_t, v2: uintptr_t) -> i32;

    fn isl_val_read_from_str(ctx: uintptr_t, str_: *const c_char) -> uintptr_t;

    fn isl_val_dump(v: uintptr_t);

    fn isl_val_to_str(v: uintptr_t) -> *const c_char;

}

impl Clone for Val {
    fn clone(&self) -> Val {
        self.copy()
    }
}

impl core::ops::Add for Val {
    type Output = Val;

    fn add(self, rhs: Val) -> Val {
        self.add(rhs)
    }
}

impl core::ops::Sub for Val {
    type Output = Val;

    fn sub(self, rhs: Val) -> Val {
        self.sub(rhs)
    }
}

impl core::ops::Mul for Val {
    type Output = Val;

    fn mul(self, rhs: Val) -> Val {
        self.mul(rhs)
    }
}

impl core::ops::Div for Val {
    type Output = Val;

    fn div(self, rhs: Val) -> Val {
        self.div(rhs)
    }
}

impl core::ops::Rem for Val {
    type Output = Val;

    fn rem(self, rhs: Val) -> Val {
        self.mod_(rhs)
    }
}

impl Val {
    /// Wraps `isl_val_zero`.
    pub fn zero(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_zero(ctx) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_one`.
    pub fn one(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_one(ctx) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_negone`.
    pub fn negone(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_negone(ctx) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_nan`.
    pub fn nan(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_nan(ctx) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_infty`.
    pub fn infty(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_infty(ctx) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_neginfty`.
    pub fn neginfty(ctx: &Context) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_neginfty(ctx) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_int_from_si`.
    pub fn int_from_si(ctx: &Context, i: i64) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_int_from_si(ctx, i) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_int_from_ui`.
    pub fn int_from_ui(ctx: &Context, u: u64) -> Val {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_val_int_from_ui(ctx, u) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_copy`.
    pub fn copy(&self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_copy(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_free`.
    pub fn free(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_free(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let val = self;
        let val = val.ptr;
        let isl_rs_result = unsafe { isl_val_get_ctx(val) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_val_get_hash`.
    pub fn get_hash(&self) -> u32 {
        let val = self;
        let val = val.ptr;
        let isl_rs_result = unsafe { isl_val_get_hash(val) };
        isl_rs_result
    }

    /// Wraps `isl_val_get_num_si`.
    pub fn get_num_si(&self) -> i64 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_num_si(v) };
        isl_rs_result
    }

    /// Wraps `isl_val_get_den_si`.
    pub fn get_den_si(&self) -> i64 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_den_si(v) };
        isl_rs_result
    }

    /// Wraps `isl_val_get_den_val`.
    pub fn get_den_val(&self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_den_val(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_get_d`.
    pub fn get_d(&self) -> f64 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_get_d(v) };
        isl_rs_result
    }

    /// Wraps `isl_val_n_abs_num_chunks`.
    pub fn n_abs_num_chunks(&self, size: usize) -> i32 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_n_abs_num_chunks(v, size) };
        isl_rs_result
    }

    /// Wraps `isl_val_set_si`.
    pub fn set_si(self, i: i64) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_set_si(v, i) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_abs`.
    pub fn abs(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_abs(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_neg`.
    pub fn neg(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_neg(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_inv`.
    pub fn inv(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_inv(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_floor`.
    pub fn floor(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_floor(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_ceil`.
    pub fn ceil(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_ceil(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_trunc`.
    pub fn trunc(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_trunc(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_2exp`.
    pub fn to_exp(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_2exp(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_pow2`.
    pub fn pow2(self) -> Val {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_pow2(v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_min`.
    pub fn min(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_min(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_max`.
    pub fn max(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_max(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_add`.
    pub fn add(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_add(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_add_ui`.
    pub fn add_ui(self, v2: u64) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_add_ui(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_sub`.
    pub fn sub(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_sub(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_sub_ui`.
    pub fn sub_ui(self, v2: u64) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_sub_ui(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_mul`.
    pub fn mul(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_mul(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_mul_ui`.
    pub fn mul_ui(self, v2: u64) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_mul_ui(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_div`.
    pub fn div(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_div(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_div_ui`.
    pub fn div_ui(self, v2: u64) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let isl_rs_result = unsafe { isl_val_div_ui(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_mod`.
    pub fn mod_(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_mod(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_gcd`.
    pub fn gcd(self, v2: Val) -> Val {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let mut v1 = v1;
        v1.do_not_free_on_drop();
        let v1 = v1.ptr;
        let mut v2 = v2;
        v2.do_not_free_on_drop();
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_gcd(v1, v2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_sgn`.
    pub fn sgn(&self) -> i32 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_sgn(v) };
        isl_rs_result
    }

    /// Wraps `isl_val_is_zero`.
    pub fn is_zero(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_zero(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_one`.
    pub fn is_one(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_one(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_negone`.
    pub fn is_negone(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_negone(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_nonneg`.
    pub fn is_nonneg(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_nonneg(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_nonpos`.
    pub fn is_nonpos(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_nonpos(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_pos`.
    pub fn is_pos(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_pos(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_neg`.
    pub fn is_neg(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_neg(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_int`.
    pub fn is_int(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_int(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_rat`.
    pub fn is_rat(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_rat(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_nan`.
    pub fn is_nan(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_nan(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_infty`.
    pub fn is_infty(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_infty(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_neginfty`.
    pub fn is_neginfty(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_is_neginfty(v) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_cmp_si`.
    pub fn cmp_si(&self, i: i64) -> i32 {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_cmp_si(v, i) };
        isl_rs_result
    }

    /// Wraps `isl_val_lt`.
    pub fn lt(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_lt(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_le`.
    pub fn le(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_le(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_gt`.
    pub fn gt(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_gt(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_gt_si`.
    pub fn gt_si(&self, i: i64) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_gt_si(v, i) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_ge`.
    pub fn ge(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_ge(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_eq`.
    pub fn eq(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_eq(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_eq_si`.
    pub fn eq_si(&self, i: i64) -> bool {
        let context_for_error_message = self.get_ctx();
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_eq_si(v, i) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_ne`.
    pub fn ne(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_ne(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_abs_eq`.
    pub fn abs_eq(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_abs_eq(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_is_divisible_by`.
    pub fn is_divisible_by(&self, v2: &Val) -> bool {
        let context_for_error_message = self.get_ctx();
        let v1 = self;
        let v1 = v1.ptr;
        let v2 = v2.ptr;
        let isl_rs_result = unsafe { isl_val_is_divisible_by(v1, v2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_val_read_from_str`.
    pub fn read_from_str(ctx: &Context, str_: &str) -> Val {
        let ctx = ctx.ptr;
        let str_ = CString::new(str_).unwrap();
        let str_ = str_.as_ptr();
        let isl_rs_result = unsafe { isl_val_read_from_str(ctx, str_) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_val_dump`.
    pub fn dump(&self) {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_dump(v) };
        isl_rs_result
    }

    /// Wraps `isl_val_to_str`.
    pub fn to_str(&self) -> &str {
        let v = self;
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_val_to_str(v) };
        let isl_rs_result = unsafe { CStr::from_ptr(isl_rs_result) };
        let isl_rs_result = isl_rs_result.to_str().unwrap();
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_val_free(self.ptr);
            }
        }
    }
}
