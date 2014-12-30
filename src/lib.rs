#![feature(link_args)]
#![allow(dead_code)]
#![allow(raw_pointer_deriving)]
#![allow(unused_imports)]

extern crate libc;

use libc::{c_void, c_char, c_int, uint32_t, uint8_t, c_long, free, puts};
use std::mem;

#[repr(C)]
pub type mpc_val_t = c_void;

#[repr(C)]
pub type mpc_dtor_t = unsafe extern fn(*mut mpc_val_t);

#[repr(C)]
pub type mpc_ctor_t = unsafe extern fn() -> *mut mpc_val_t;

#[repr(C)]
pub type mpc_apply_t = unsafe extern fn(*mut mpc_val_t) -> *mut  mpc_val_t;

#[repr(C)]
pub type mpc_apply_to_t = unsafe extern fn(*mut mpc_val_t) -> *mut mpc_val_t;

#[repr(C)]
pub type mpc_fold_t = unsafe extern fn(c_int, *mut *mut mpc_val_t) -> *mut mpc_val_t;

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_fail_t {
   pub m: *mut c_char
}

#[repr(C)]
struct mpc_pdata_lift_t {
   pub lf: mpc_ctor_t,
   pub x: *mut c_void,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_expect_t {
   pub x: *mut mpc_parser_t,
   pub m: *mut c_char,
}

#[repr(C)]
struct mpc_pdata_anchor_t {
    pub f: unsafe extern fn(c_char, c_char) -> c_int,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_single_t {
   pub x: c_char,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_range_t {
   pub x: c_char,
   pub y: c_char,
}

#[repr(C)]
struct mpc_pdata_satisfy_t {
   pub f: unsafe extern fn(c_char) -> c_int,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_string_t {
    pub x: *mut c_char,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_ast_t {
  pub tag: *mut c_char,
  pub contents: *mut c_char,
  pub state: mpc_state_t,
  pub children_num: c_int,
  pub children: *mut *mut mpc_ast_t,
}

#[repr(C)]
struct mpc_pdata_apply_t {
   pub x: *mut mpc_parser_t,
   pub f: mpc_apply_t,
}

#[repr(C)]
struct mpc_pdata_apply_to_t {
    x: *mut mpc_parser_t,
    f: mpc_apply_to_t,
    d: *mut c_void,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_predict_t {
   pub x: *mut mpc_parser_t,
}

#[repr(C)]
struct mpc_pdata_not_t {
   pub x: *mut mpc_parser_t,
   pub dx: mpc_dtor_t,
   pub lf: mpc_ctor_t,
}

#[repr(C)]
struct mpc_pdata_repeat_t {
   pub n: c_int,
   pub f: mpc_fold_t,
   pub x: *mut mpc_parser_t,
   pub dx: mpc_dtor_t,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_pdata_or_t {
   pub n: c_int,
   pub xs: *mut *mut mpc_parser_t,
}

#[repr(C)]
struct mpc_pdata_and_t {
   pub n: c_int,
   pub f: mpc_fold_t,
   pub xs: *mut *mut mpc_parser_t,
   pub dxs: *mut mpc_dtor_t,
}

#[repr(C)]
struct mpc_pdata_t {
    pub fail: mpc_pdata_fail_t,
    pub lift: mpc_pdata_lift_t,
    pub expect: mpc_pdata_expect_t,
    pub anchor: mpc_pdata_anchor_t,
    pub single: mpc_pdata_single_t,
    pub range: mpc_pdata_range_t,
    pub satisfy: mpc_pdata_satisfy_t,
    pub string: mpc_pdata_string_t,
    pub apply: mpc_pdata_apply_t,
    pub apply_to: mpc_pdata_apply_to_t,
    pub predict: mpc_pdata_predict_t,
    pub not: mpc_pdata_not_t,
    pub repeat: mpc_pdata_repeat_t,
    pub and: mpc_pdata_and_t,
    pub or: mpc_pdata_or_t,
}


#[repr(C)]
#[deriving(Show)]
struct mpc_result_t {
    pub error: *mut mpc_err_t,
    pub output: *mut mpc_val_t,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_state_t {
   pub pos: c_long,
   pub row: c_long,
   pub col: c_long,
}

#[repr(C)]
#[deriving(Show)]
struct mpc_err_t {
   pub state: mpc_state_t,
   pub expected_num: c_int,
   pub filename: *mut c_char,
   pub failure: *mut c_char,
   pub expected: *mut *mut c_char,
   pub recieved: c_char,
}

#[repr(C)]
struct mpc_parser_t {
    pub retained: c_char,
    pub name: *mut c_char,
    // actually this is a c_char but i'm setting to *mut to align mpc_parser_t
    pub _type: c_char,
    pub data: mpc_pdata_t,
}

#[link_args = "./src/mpc.c -ledit -lm"]
extern {
    fn mpc_new(name: *const c_char) -> *mut mpc_parser_t;
    fn mpc_or(n: c_int, ...) -> *mut mpc_parser_t;
    fn mpc_sym(name: *const c_char) -> *mut mpc_parser_t;
    fn mpc_and(n: c_int, f: mpc_fold_t, ...) -> *mut mpc_parser_t;
    fn mpc_many(f: mpc_fold_t, parser: *mut mpc_parser_t) -> *mut mpc_parser_t;
    fn mpcf_strfold(n: c_int, xs: *mut *mut mpc_val_t) -> *mut mpc_val_t;
    fn mpc_parse(filename: *const c_char, string: *const c_char, p: *mut mpc_parser_t, r: *mut mpc_result_t) -> c_int;
    fn mpc_parse_contents(filename: *const c_char, p: *mut mpc_parser_t, r: *mut mpc_result_t) -> c_int;
    fn mpc_ast_print(a: *mut mpc_ast_t);
    fn mpca_lang(flags: c_int, grammar: *const c_char, ...) -> *mut mpc_parser_t;
    fn mpc_cleanup(n: c_int, ...);
}

#[test]
fn it_works() {
//    unsafe {
//        let adj = mpc_or(4,
//                         mpc_sym("wow".to_c_str().as_mut_ptr()),
//                         mpc_sym("so".to_c_str().as_mut_ptr()),
//                         mpc_sym("many".to_c_str().as_mut_ptr()),
//                         mpc_sym("such".to_c_str().as_mut_ptr()),
//                        );
//        let noun = mpc_or(5,
//                         mpc_sym("lisp".to_c_str().as_mut_ptr()),
//                         mpc_sym("book".to_c_str().as_mut_ptr()),
//                         mpc_sym("c".to_c_str().as_mut_ptr()),
//                         mpc_sym("language".to_c_str().as_mut_ptr()),
//                         mpc_sym("build".to_c_str().as_mut_ptr()),
//                        );
//        let phrase = mpc_and(2,
//                        mpcf_strfold,
//                        adj, noun, free,
//                        );
//        let doge = mpc_many(mpcf_strfold, phrase);
//        let mut r = mpc_result_t { data: [1u8, ..8] };
//        let ret = mpc_parse(
//            "<stdin>".to_c_str().as_ptr(),
//            "wow lisp such language so c".to_c_str().as_ptr(),
//            doge,
//            &mut r,
//        );
//        println!("n: {}", (*((*adj).data.or() as *mut mpc_pdata_or_t)).n);
//        println!("Rust pointer: {:p}", &(*adj).data);
//        println!("Rust fail pointer: {:p}", &(*adj).data.fail());
//        println!("Rust lift pointer: {:p}", &(*adj).data.lift());
//    };

    unsafe {
        let adj = mpc_or(4,
            mpc_sym("wow".to_c_str().as_ptr()),
            mpc_sym("so".to_c_str().as_ptr()),
            mpc_sym("many".to_c_str().as_ptr()),
            mpc_sym("such".to_c_str().as_ptr()),
            mpc_sym("such".to_c_str().as_ptr()),
        );
    };
}
