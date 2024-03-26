// Automatically generated by isl_bindings_generator.
// LICENSE: MIT

use crate::bindings::{Context, Val, Vec};
use libc::uintptr_t;

/// Wraps `isl_mat`.
pub struct Mat {
    pub ptr: uintptr_t,
    pub should_free_on_drop: bool,
}

extern "C" {

    fn isl_mat_get_ctx(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_alloc(ctx: uintptr_t, n_row: u32, n_col: u32) -> uintptr_t;

    fn isl_mat_extend(mat: uintptr_t, n_row: u32, n_col: u32) -> uintptr_t;

    fn isl_mat_identity(ctx: uintptr_t, n_row: u32) -> uintptr_t;

    fn isl_mat_copy(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_free(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_rows(mat: uintptr_t) -> i32;

    fn isl_mat_cols(mat: uintptr_t) -> i32;

    fn isl_mat_get_element_val(mat: uintptr_t, row: i32, col: i32) -> uintptr_t;

    fn isl_mat_set_element_si(mat: uintptr_t, row: i32, col: i32, v: i32) -> uintptr_t;

    fn isl_mat_set_element_val(mat: uintptr_t, row: i32, col: i32, v: uintptr_t) -> uintptr_t;

    fn isl_mat_swap_cols(mat: uintptr_t, i: u32, j: u32) -> uintptr_t;

    fn isl_mat_swap_rows(mat: uintptr_t, i: u32, j: u32) -> uintptr_t;

    fn isl_mat_vec_product(mat: uintptr_t, vec: uintptr_t) -> uintptr_t;

    fn isl_mat_vec_inverse_product(mat: uintptr_t, vec: uintptr_t) -> uintptr_t;

    fn isl_mat_aff_direct_sum(left: uintptr_t, right: uintptr_t) -> uintptr_t;

    fn isl_mat_diagonal(mat1: uintptr_t, mat2: uintptr_t) -> uintptr_t;

    fn isl_mat_lin_to_aff(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_inverse_product(left: uintptr_t, right: uintptr_t) -> uintptr_t;

    fn isl_mat_product(left: uintptr_t, right: uintptr_t) -> uintptr_t;

    fn isl_mat_transpose(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_right_inverse(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_right_kernel(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_normalize(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_normalize_row(mat: uintptr_t, row: i32) -> uintptr_t;

    fn isl_mat_drop_cols(mat: uintptr_t, col: u32, n: u32) -> uintptr_t;

    fn isl_mat_drop_rows(mat: uintptr_t, row: u32, n: u32) -> uintptr_t;

    fn isl_mat_insert_cols(mat: uintptr_t, col: u32, n: u32) -> uintptr_t;

    fn isl_mat_insert_rows(mat: uintptr_t, row: u32, n: u32) -> uintptr_t;

    fn isl_mat_move_cols(mat: uintptr_t, dst_col: u32, src_col: u32, n: u32) -> uintptr_t;

    fn isl_mat_add_rows(mat: uintptr_t, n: u32) -> uintptr_t;

    fn isl_mat_insert_zero_cols(mat: uintptr_t, first: u32, n: u32) -> uintptr_t;

    fn isl_mat_add_zero_cols(mat: uintptr_t, n: u32) -> uintptr_t;

    fn isl_mat_insert_zero_rows(mat: uintptr_t, row: u32, n: u32) -> uintptr_t;

    fn isl_mat_add_zero_rows(mat: uintptr_t, n: u32) -> uintptr_t;

    fn isl_mat_col_add(mat: uintptr_t, dst_col: i32, src_col: i32);

    fn isl_mat_unimodular_complete(M: uintptr_t, row: i32) -> uintptr_t;

    fn isl_mat_row_basis(mat: uintptr_t) -> uintptr_t;

    fn isl_mat_row_basis_extension(mat1: uintptr_t, mat2: uintptr_t) -> uintptr_t;

    fn isl_mat_from_row_vec(vec: uintptr_t) -> uintptr_t;

    fn isl_mat_concat(top: uintptr_t, bot: uintptr_t) -> uintptr_t;

    fn isl_mat_vec_concat(top: uintptr_t, bot: uintptr_t) -> uintptr_t;

    fn isl_mat_is_equal(mat1: uintptr_t, mat2: uintptr_t) -> i32;

    fn isl_mat_has_linearly_independent_rows(mat1: uintptr_t, mat2: uintptr_t) -> i32;

    fn isl_mat_rank(mat: uintptr_t) -> i32;

    fn isl_mat_initial_non_zero_cols(mat: uintptr_t) -> i32;

    fn isl_mat_dump(mat: uintptr_t);

}

impl Clone for Mat {
    fn clone(&self) -> Mat {
        self.copy()
    }
}

impl PartialEq for Mat {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl Eq for Mat {}

impl Mat {
    /// Wraps `isl_mat_get_ctx`.
    pub fn get_ctx(&self) -> Context {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_get_ctx(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Context { ptr: isl_rs_result,
                                      should_free_on_drop: true };
        let mut isl_rs_result = isl_rs_result;
        isl_rs_result.do_not_free_on_drop();
        isl_rs_result
    }

    /// Wraps `isl_mat_alloc`.
    pub fn alloc(ctx: &Context, n_row: u32, n_col: u32) -> Mat {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_mat_alloc(ctx, n_row, n_col) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_extend`.
    pub fn extend(self, n_row: u32, n_col: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_extend(mat, n_row, n_col) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_identity`.
    pub fn identity(ctx: &Context, n_row: u32) -> Mat {
        let ctx = ctx.ptr;
        let isl_rs_result = unsafe { isl_mat_identity(ctx, n_row) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_copy`.
    pub fn copy(&self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_copy(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_free`.
    pub fn free(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_free(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_rows`.
    pub fn rows(&self) -> i32 {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_rows(mat) };
        isl_rs_result
    }

    /// Wraps `isl_mat_cols`.
    pub fn cols(&self) -> i32 {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_cols(mat) };
        isl_rs_result
    }

    /// Wraps `isl_mat_get_element_val`.
    pub fn get_element_val(&self, row: i32, col: i32) -> Val {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_get_element_val(mat, row, col) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Val { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_set_element_si`.
    pub fn set_element_si(self, row: i32, col: i32, v: i32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_set_element_si(mat, row, col, v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_set_element_val`.
    pub fn set_element_val(self, row: i32, col: i32, v: Val) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let mut v = v;
        v.do_not_free_on_drop();
        let v = v.ptr;
        let isl_rs_result = unsafe { isl_mat_set_element_val(mat, row, col, v) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_swap_cols`.
    pub fn swap_cols(self, i: u32, j: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_swap_cols(mat, i, j) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_swap_rows`.
    pub fn swap_rows(self, i: u32, j: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_swap_rows(mat, i, j) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_vec_product`.
    pub fn vec_product(self, vec: Vec) -> Vec {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let mut vec = vec;
        vec.do_not_free_on_drop();
        let vec = vec.ptr;
        let isl_rs_result = unsafe { isl_mat_vec_product(mat, vec) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Vec { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_vec_inverse_product`.
    pub fn vec_inverse_product(self, vec: Vec) -> Vec {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let mut vec = vec;
        vec.do_not_free_on_drop();
        let vec = vec.ptr;
        let isl_rs_result = unsafe { isl_mat_vec_inverse_product(mat, vec) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Vec { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_aff_direct_sum`.
    pub fn aff_direct_sum(self, right: Mat) -> Mat {
        let context_for_error_message = self.get_ctx();
        let left = self;
        let mut left = left;
        left.do_not_free_on_drop();
        let left = left.ptr;
        let mut right = right;
        right.do_not_free_on_drop();
        let right = right.ptr;
        let isl_rs_result = unsafe { isl_mat_aff_direct_sum(left, right) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_diagonal`.
    pub fn diagonal(self, mat2: Mat) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat1 = self;
        let mut mat1 = mat1;
        mat1.do_not_free_on_drop();
        let mat1 = mat1.ptr;
        let mut mat2 = mat2;
        mat2.do_not_free_on_drop();
        let mat2 = mat2.ptr;
        let isl_rs_result = unsafe { isl_mat_diagonal(mat1, mat2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_lin_to_aff`.
    pub fn lin_to_aff(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_lin_to_aff(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_inverse_product`.
    pub fn inverse_product(self, right: Mat) -> Mat {
        let context_for_error_message = self.get_ctx();
        let left = self;
        let mut left = left;
        left.do_not_free_on_drop();
        let left = left.ptr;
        let mut right = right;
        right.do_not_free_on_drop();
        let right = right.ptr;
        let isl_rs_result = unsafe { isl_mat_inverse_product(left, right) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_product`.
    pub fn product(self, right: Mat) -> Mat {
        let context_for_error_message = self.get_ctx();
        let left = self;
        let mut left = left;
        left.do_not_free_on_drop();
        let left = left.ptr;
        let mut right = right;
        right.do_not_free_on_drop();
        let right = right.ptr;
        let isl_rs_result = unsafe { isl_mat_product(left, right) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_transpose`.
    pub fn transpose(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_transpose(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_right_inverse`.
    pub fn right_inverse(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_right_inverse(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_right_kernel`.
    pub fn right_kernel(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_right_kernel(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_normalize`.
    pub fn normalize(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_normalize(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_normalize_row`.
    pub fn normalize_row(self, row: i32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_normalize_row(mat, row) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_drop_cols`.
    pub fn drop_cols(self, col: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_drop_cols(mat, col, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_drop_rows`.
    pub fn drop_rows(self, row: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_drop_rows(mat, row, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_insert_cols`.
    pub fn insert_cols(self, col: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_insert_cols(mat, col, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_insert_rows`.
    pub fn insert_rows(self, row: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_insert_rows(mat, row, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_move_cols`.
    pub fn move_cols(self, dst_col: u32, src_col: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_move_cols(mat, dst_col, src_col, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_add_rows`.
    pub fn add_rows(self, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_add_rows(mat, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_insert_zero_cols`.
    pub fn insert_zero_cols(self, first: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_insert_zero_cols(mat, first, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_add_zero_cols`.
    pub fn add_zero_cols(self, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_add_zero_cols(mat, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_insert_zero_rows`.
    pub fn insert_zero_rows(self, row: u32, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_insert_zero_rows(mat, row, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_add_zero_rows`.
    pub fn add_zero_rows(self, n: u32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_add_zero_rows(mat, n) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_col_add`.
    pub fn col_add(&self, dst_col: i32, src_col: i32) {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_col_add(mat, dst_col, src_col) };
        isl_rs_result
    }

    /// Wraps `isl_mat_unimodular_complete`.
    pub fn unimodular_complete(self, row: i32) -> Mat {
        let context_for_error_message = self.get_ctx();
        let M = self;
        let mut M = M;
        M.do_not_free_on_drop();
        let M = M.ptr;
        let isl_rs_result = unsafe { isl_mat_unimodular_complete(M, row) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_row_basis`.
    pub fn row_basis(self) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat = self;
        let mut mat = mat;
        mat.do_not_free_on_drop();
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_row_basis(mat) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_row_basis_extension`.
    pub fn row_basis_extension(self, mat2: Mat) -> Mat {
        let context_for_error_message = self.get_ctx();
        let mat1 = self;
        let mut mat1 = mat1;
        mat1.do_not_free_on_drop();
        let mat1 = mat1.ptr;
        let mut mat2 = mat2;
        mat2.do_not_free_on_drop();
        let mat2 = mat2.ptr;
        let isl_rs_result = unsafe { isl_mat_row_basis_extension(mat1, mat2) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_from_row_vec`.
    pub fn from_row_vec(vec: Vec) -> Mat {
        let mut vec = vec;
        vec.do_not_free_on_drop();
        let vec = vec.ptr;
        let isl_rs_result = unsafe { isl_mat_from_row_vec(vec) };
        if isl_rs_result == 0 {
            panic!("ISL error");
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_concat`.
    pub fn concat(self, bot: Mat) -> Mat {
        let context_for_error_message = self.get_ctx();
        let top = self;
        let mut top = top;
        top.do_not_free_on_drop();
        let top = top.ptr;
        let mut bot = bot;
        bot.do_not_free_on_drop();
        let bot = bot.ptr;
        let isl_rs_result = unsafe { isl_mat_concat(top, bot) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_vec_concat`.
    pub fn vec_concat(self, bot: Vec) -> Mat {
        let context_for_error_message = self.get_ctx();
        let top = self;
        let mut top = top;
        top.do_not_free_on_drop();
        let top = top.ptr;
        let mut bot = bot;
        bot.do_not_free_on_drop();
        let bot = bot.ptr;
        let isl_rs_result = unsafe { isl_mat_vec_concat(top, bot) };
        if isl_rs_result == 0 {
            panic!("ISL error: {}", context_for_error_message.last_error_msg());
        }
        let isl_rs_result = Mat { ptr: isl_rs_result,
                                  should_free_on_drop: true };
        isl_rs_result
    }

    /// Wraps `isl_mat_is_equal`.
    pub fn is_equal(&self, mat2: &Mat) -> bool {
        let context_for_error_message = self.get_ctx();
        let mat1 = self;
        let mat1 = mat1.ptr;
        let mat2 = mat2.ptr;
        let isl_rs_result = unsafe { isl_mat_is_equal(mat1, mat2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_mat_has_linearly_independent_rows`.
    pub fn has_linearly_independent_rows(&self, mat2: &Mat) -> bool {
        let context_for_error_message = self.get_ctx();
        let mat1 = self;
        let mat1 = mat1.ptr;
        let mat2 = mat2.ptr;
        let isl_rs_result = unsafe { isl_mat_has_linearly_independent_rows(mat1, mat2) };
        let isl_rs_result = match isl_rs_result {
            0 => false,
            1 => true,
            _ => panic!("ISL error: {}", context_for_error_message.last_error_msg()),
        };
        isl_rs_result
    }

    /// Wraps `isl_mat_rank`.
    pub fn rank(&self) -> i32 {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_rank(mat) };
        isl_rs_result
    }

    /// Wraps `isl_mat_initial_non_zero_cols`.
    pub fn initial_non_zero_cols(&self) -> i32 {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_initial_non_zero_cols(mat) };
        isl_rs_result
    }

    /// Wraps `isl_mat_dump`.
    pub fn dump(&self) {
        let mat = self;
        let mat = mat.ptr;
        let isl_rs_result = unsafe { isl_mat_dump(mat) };
        isl_rs_result
    }

    /// Does not call isl_xxx_free() on being dropped. (For internal use only.)
    pub fn do_not_free_on_drop(&mut self) {
        self.should_free_on_drop = false;
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        if self.should_free_on_drop {
            unsafe {
                isl_mat_free(self.ptr);
            }
        }
    }
}
