// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Context, DimType, Fold, Point, QPolynomial, Set, Space, Val};
use libc::uintptr_t;

/// Wraps `isl_qpolynomial_fold`.
pub struct QPolynomialFold {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_qpolynomial_fold_gist(fold: uintptr_t, context: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_empty(type_: Fold, space: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_get_ctx(fold: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_is_empty(fold: uintptr_t) -> i32;

    fn isl_qpolynomial_fold_get_domain_space(fold: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_alloc(type_: Fold, qp: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_is_nan(fold: uintptr_t) -> i32;

    fn isl_qpolynomial_fold_move_dims(fold: uintptr_t, dst_type: DimType, dst_pos: u32,
                                      src_type: DimType, src_pos: u32, n: u32)
                                      -> uintptr_t;

    fn isl_qpolynomial_fold_get_space(fold: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_free(fold: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_plain_is_equal(fold1: uintptr_t, fold2: uintptr_t) -> i32;

    fn isl_qpolynomial_fold_dump(fold: uintptr_t);

    fn isl_qpolynomial_fold_get_type(fold: uintptr_t) -> Fold;

    fn isl_qpolynomial_fold_copy(fold: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_fold(fold1: uintptr_t, fold2: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_scale_val(fold: uintptr_t, v: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_eval(fold: uintptr_t, pnt: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_gist_params(fold: uintptr_t, context: uintptr_t) -> uintptr_t;

    fn isl_qpolynomial_fold_scale_down_val(fold: uintptr_t, v: uintptr_t) -> uintptr_t;

}

impl Clone for QPolynomialFold {
    fn clone(&self) -> QPolynomialFold {
        self.copy()
    }
}

impl QPolynomialFold {
    /// Wraps `isl_qpolynomial_fold_gist`.
    pub fn gist(self, context: Set) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let mut context = context;
        context.do_not_free_on_drop();
        let context = context.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_gist(fold, context) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_empty`.
    pub fn empty(type_: Fold, space: Space) -> QPolynomialFold {
        let mut space = space;
        space.do_not_free_on_drop();
        let space = space.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_empty(type_, space) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_get_ctx(fold) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_is_empty`.
    pub fn is_empty(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_is_empty(fold) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_get_domain_space`.
    pub fn get_domain_space(&self) -> Space {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_get_domain_space(fold) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_alloc`.
    pub fn alloc(type_: Fold, qp: QPolynomial) -> QPolynomialFold {
        let mut qp = qp;
        qp.do_not_free_on_drop();
        let qp = qp.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_alloc(type_, qp) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_is_nan`.
    pub fn is_nan(&self) -> bool {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_is_nan(fold) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_move_dims`.
    pub fn move_dims(self, dst_type: DimType, dst_pos: u32, src_type: DimType, src_pos: u32,
                     n: u32)
                     -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let isl_rs_result = unsafe {
            isl_qpolynomial_fold_move_dims(fold, dst_type, dst_pos, src_type, src_pos, n)
        };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_get_space`.
    pub fn get_space(&self) -> Space {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_get_space(fold) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Space { ptr: isl_rs_result,
                                    should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_free`.
    pub fn free(self) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_free(fold) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_plain_is_equal`.
    pub fn plain_is_equal(&self, fold2: &QPolynomialFold) -> bool {
        let context_for_error_message = self.get_ctx();
        let fold1 = self;
        let fold1 = fold1.ptr;
        let fold2 = fold2.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_plain_is_equal(fold1, fold2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_dump`.
    pub fn dump(&self) {
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_dump(fold) };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_get_type`.
    pub fn get_type(&self) -> Fold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_get_type(fold) };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_copy`.
    pub fn copy(&self) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let fold = fold.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_copy(fold) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_fold`.
    pub fn fold(self, fold2: QPolynomialFold) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold1 = self;
        let mut fold1 = fold1;
        fold1.do_not_free_on_drop();
        let fold1 = fold1.ptr;
        let mut fold2 = fold2;
        fold2.do_not_free_on_drop();
        let fold2 = fold2.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_fold(fold1, fold2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_scale_val`.
    pub fn scale_val(self, v: Val) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_scale_val(fold, v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_eval`.
    pub fn eval(self, pnt: Point) -> Val {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let mut pnt = pnt;
        pnt.do_not_free_on_drop();
        let pnt = pnt.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_eval(fold, pnt) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_gist_params`.
    pub fn gist_params(self, context: Set) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let mut context = context;
        context.do_not_free_on_drop();
        let context = context.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_gist_params(fold, context) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_qpolynomial_fold_scale_down_val`.
    pub fn scale_down_val(self, v: Val) -> QPolynomialFold {
        let context_for_error_message = self.get_ctx();
        let fold = self;
        let mut fold = fold;
        fold.do_not_free_on_drop();
        let fold = fold.ptr;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_qpolynomial_fold_scale_down_val(fold, v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = QPolynomialFold { ptr: isl_rs_result,
                                              should_free_on_drop: true };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for QPolynomialFold {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_qpolynomial_fold_free(self.ptr);
            }
        }
    }
}