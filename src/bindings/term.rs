// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Aff, Context, DimType, Val};
use libc::uintptr_t;

/// Wraps `isl_term`.
pub struct Term {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_term_get_div(term: uintptr_t, pos: u32) -> uintptr_t;

    fn isl_term_free(term: uintptr_t) -> uintptr_t;

    fn isl_term_get_coefficient_val(term: uintptr_t) -> uintptr_t;

    fn isl_term_dim(term: uintptr_t, type_: DimType) -> i32;

    fn isl_term_copy(term: uintptr_t) -> uintptr_t;

    fn isl_term_get_ctx(term: uintptr_t) -> uintptr_t;

    fn isl_term_get_exp(term: uintptr_t, type_: DimType, pos: u32) -> i32;

}

impl Clone for Term {
    fn clone(&self) -> Term {
        self.copy()
    }
}

impl Term {
    /// Wraps `isl_term_get_div`.
    pub fn get_div(&self, pos: u32) -> Aff {
        let context_for_error_message = self.get_ctx();
        let term = self;
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_get_div(term, pos) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Aff { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_term_free`.
    pub fn free(self) -> Term {
        let context_for_error_message = self.get_ctx();
        let term = self;
        let mut term = term;
        term.do_not_free_on_drop();
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_free(term) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Term { ptr: isl_rs_result,
                                   should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_term_get_coefficient_val`.
    pub fn get_coefficient_val(&self) -> Val {
        let context_for_error_message = self.get_ctx();
        let term = self;
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_get_coefficient_val(term) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_term_dim`.
    pub fn dim(&self, type_: DimType) -> i32 {
        let term = self;
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_dim(term, type_) };
        isl_rs_result
    }

    /// Wraps `isl_term_copy`.
    pub fn copy(&self) -> Term {
        let context_for_error_message = self.get_ctx();
        let term = self;
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_copy(term) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Term { ptr: isl_rs_result,
                                   should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_term_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let term = self;
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_get_ctx(term) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_term_get_exp`.
    pub fn get_exp(&self, type_: DimType, pos: u32) -> i32 {
        let term = self;
        let term = term.ptr;
        let isl_rs_result = unsafe { isl_term_get_exp(term, type_, pos) };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_term_free(self.ptr);
            }
        }
    }
}
