#![feature(link_args)]

extern crate libc;

use libc::{c_void, c_char, c_int};

#[repr(C)]
type mpc_val_t = c_void;

#[repr(C)]
type mpc_dtor_t = fn(*mut mpc_val_t);

#[repr(C)]
type mpc_ctor_t = fn() -> *mut mpc_val_t;

#[repr(C)]
type mpc_apply_t = fn(*mut mpc_val_t) -> *mut mpc_val_t;

#[repr(C)]
type mpc_apply_to_t = fn(*mut mpc_val_t) -> *mut mpc_val_t;

#[repr(C)]
type mpc_fold_t = fn(c_int, *mut *mut mpc_val_t) -> *mut mpc_val_t;

#[repr(C)]
struct mpc_pdata_fail_t {
    m: *mut c_char
}

#[repr(C)]
struct mpc_pdata_lift_t {
    lf: mpc_ctor_t,
    x: *mut c_void,
}

#[repr(C)]
struct mpc_pdata_expect_t {
    x: *mut mpc_parser_t,
    m: *mut c_char,
}

#[repr(C)]
struct mpc_pdata_anchor_t {
    f: fn(c_char, c_char) -> c_int,
}

#[repr(C)]
struct mpc_pdata_single_t {
    x: c_char,
}

#[repr(C)]
struct mpc_pdata_range_t {
    x: c_char,
    y: c_char,
}

#[repr(C)]
struct mpc_pdata_satisfy_t {
    f: fn(c_char) -> c_int,
}

#[repr(C)]
struct mpc_pdata_string_t {
    x: *mut c_char,
}

#[repr(C)]
struct mpc_pdata_apply_t {
    x: *mut mpc_parser_t,
    f: mpc_apply_t,
}

#[repr(C)]
struct mpc_pdata_apply_to_t {
    x: *mut mpc_parser_t,
    f: mpc_apply_to_t,
    d: *mut c_void,
}

#[repr(C)]
struct mpc_pdata_predict_t {
    x: *mut mpc_parser_t,
}

#[repr(C)]
struct mpc_pdata_not_t {
    x: *mut mpc_parser_t,
    dx: mpc_dtor_t,
    lf: mpc_dtor_t,
}

#[repr(C)]
struct mpc_pdata_repeat_t {
    n: c_int,
    f: mpc_fold_t,
    x: *mut mpc_parser_t,
    dx: mpc_dtor_t,
}

#[repr(C)]
struct mpc_pdata_or_t {
    n: c_int,
    xs: *mut *mut mpc_parser_t,
}

#[repr(C)]
struct mpc_pdata_and_t {
    n: c_int,
    f: mpc_fold_t,
    xs: *mut *mut mpc_parser_t,
}

#[repr(C)]
struct mpc_pdata_t {
  fail: mpc_pdata_fail_t,
  left: mpc_pdata_lift_t,
  expect: mpc_pdata_expect_t,
  anchor: mpc_pdata_anchor_t,
  single: mpc_pdata_single_t,
  range: mpc_pdata_range_t,
  satisfy: mpc_pdata_satisfy_t,
  string: mpc_pdata_string_t,
  apply: mpc_pdata_apply_t,
  apply_to: mpc_pdata_apply_to_t,
  predict: mpc_pdata_predict_t,
  not: mpc_pdata_not_t,
  repeat: mpc_pdata_repeat_t,
  and: mpc_pdata_and_t,
  or: mpc_pdata_or_t,
}

#[repr(C)]
struct mpc_parser_t {
    retained: c_char,
    name: *mut c_char,
    parser_type: c_char,
    data: mpc_pdata_t,
}

#[link_args = "./src/mpc.c -ledit -lm"]
extern {
    fn mpc_new(name: *const c_char) -> mpc_parser_t;
}

#[test]
fn it_works() {
    let number = unsafe { mpc_new("number".to_c_str().as_ptr()) };
}
