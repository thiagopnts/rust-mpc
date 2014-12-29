#![feature(link_args)]
#![allow(dead_code)]
#![allow(raw_pointer_deriving)]
#![allow(unused_imports)]

extern crate libc;

use libc::{c_void, c_char, c_int, uint8_t, c_long, free, puts};
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
#[deriving(Show)]
struct mpc_pdata_t {
    pub data: [uint8_t, ..32u],
}

impl mpc_pdata_t {
    pub fn fail(&mut self) -> *mut [uint8_t, ..8u] {
        unsafe { mem::transmute(self) }
    }

    pub fn lift(&mut self) -> *mut [uint8_t, ..16u] {
        unsafe { mem::transmute(self) }
    }

    pub fn expect(&mut self) -> *mut [uint8_t, ..16u] {
        unsafe { mem::transmute(self) }
    }

    pub fn anchor(&mut self) -> *mut [uint8_t, ..8u] {
        unsafe { mem::transmute(self) }
    }

    pub fn single(&mut self) -> *mut [uint8_t, ..1u]{
        unsafe { mem::transmute(self) }
    }

    pub fn range(&mut self) -> *mut [uint8_t, ..2u] {
        unsafe { mem::transmute(self) }
    }

    pub fn satisfy(&mut self) -> *mut [uint8_t, ..8u]{
        unsafe { mem::transmute(self) }
    }

    pub fn string(&mut self) -> *mut [uint8_t, ..8u]{
        unsafe { mem::transmute(self) }
    }

    pub fn apply(&mut self) -> *mut [uint8_t, ..16u]{
        unsafe { mem::transmute(self) }
    }

    pub fn apply_to(&mut self) -> *mut [uint8_t, ..24u]{
        unsafe { mem::transmute(self) }
    }

    pub fn predict(&mut self) -> *mut [uint8_t, ..8u]{
        unsafe { mem::transmute(self) }
    }

    pub fn not(&mut self) -> *mut [uint8_t, ..24u]{
        unsafe { mem::transmute(self) }
    }

    pub fn repeat(&mut self) -> *mut [uint8_t, ..32u]{
        unsafe { mem::transmute(self) }
    }

    pub fn and(&mut self) -> *mut [uint8_t, ..32u]{
        unsafe { mem::transmute(self) }
    }

    pub fn or(&mut self) -> *mut [uint8_t, ..16u]{
        unsafe { mem::transmute(self) }
    }
}

#[repr(C)]
#[deriving(Show)]
struct mpc_result_t {
    pub data: [uint8_t, ..8u],
}

impl mpc_result_t {
    pub fn error(&mut self) -> *mut [uint8_t, ..64] {
        unsafe { mem::transmute(self) }
    }

    pub fn output(&mut self) -> *mut [uint8_t, ..1] {
        unsafe { mem::transmute(self) }
    }
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
#[deriving(Show)]
struct mpc_parser_t {
    pub retained: c_char,
    pub name: *mut c_char,
    // actually this is a c_char but i'm setting to *mut to align mpc_parser_t
    pub _type: *mut c_char,
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
   println!("pdata or size: {}", mem::size_of::<mpc_pdata_t>());
    unsafe {
        let adj = mpc_or(2,
                         mpc_sym("wow".to_c_str().as_mut_ptr()),
                         mpc_sym("haha".to_c_str().as_mut_ptr())
                        );
        println!("n: {}", (*((*adj).data.or() as *mut mpc_pdata_or_t)).n);
        (*((*adj).data.or() as *mut mpc_pdata_or_t)).n = 1;
        println!("n: {}", (*((*adj).data.or() as *mut mpc_pdata_or_t)).n);
    };

}
