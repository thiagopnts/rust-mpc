#![feature(link_args)]

extern crate libc;

use libc::{c_void, c_char, c_int, uint8_t, c_long, free};
use std::mem;

#[repr(C)]
type mpc_val_t = c_void;

#[repr(C)]
type mpc_dtor_t = unsafe extern fn(*mut mpc_val_t);

#[repr(C)]
type mpc_ctor_t = unsafe extern fn() -> &'static mpc_val_t;

#[repr(C)]
type mpc_apply_t = unsafe extern fn(&mpc_val_t) -> &'static mpc_val_t;

#[repr(C)]
type mpc_apply_to_t = unsafe extern fn(&mpc_val_t) -> &'static mpc_val_t;

#[repr(C)]
type mpc_fold_t = unsafe extern fn(c_int, *mut *mut mpc_val_t) -> &'static mpc_val_t;

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
struct mpc_ast_t {
    tag: *mut c_char,
    contents: *mut c_char,
    state: mpc_state_t,
    children_num: c_int,
    children: *mut *mut mpc_ast_t,
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

enum MPCLang {
    MPCA_LANG_DEFAULT              = 0,
    MPCA_LANG_PREDICTIVE           = 1,
    MPCA_LANG_WHITESPACE_SENSITIVE = 2,
}

#[repr(C)]
struct mpc_pdata_t {
    pub data: [uint8_t, ..32u],
//  fail: mpc_pdata_fail_t,
//  lift: mpc_pdata_lift_t,
//  expect: mpc_pdata_expect_t,
//  anchor: mpc_pdata_anchor_t,
//  single: mpc_pdata_single_t,
//  range: mpc_pdata_range_t,
//  satisfy: mpc_pdata_satisfy_t,
//  string: mpc_pdata_string_t,
//  apply: mpc_pdata_apply_t,
//  apply_to: mpc_pdata_apply_to_t,
//  predict: mpc_pdata_predict_t,
//  not: mpc_pdata_not_t,
//  repeat: mpc_pdata_repeat_t,
//  and: mpc_pdata_and_t,
//  or: mpc_pdata_or_t,
}

impl mpc_pdata_t {
    pub fn fail(&self) -> *const mpc_pdata_fail_t {
        self.data.as_ptr() as *const _
    }

    pub fn lift(&self) -> *const mpc_pdata_lift_t {
        self.data.as_ptr() as *const _
    }

    pub fn expect(&self) -> *const mpc_pdata_expect_t {
        self.data.as_ptr() as *const _
    }

    pub fn anchor(&self) -> *const mpc_pdata_anchor_t {
        self.data.as_ptr() as *const _
    }

    pub fn single(&self) -> *const mpc_pdata_single_t {
        self.data.as_ptr() as *const _
    }

    pub fn range(&self) -> *const mpc_pdata_range_t {
        self.data.as_ptr() as *const _
    }

    pub fn satisfy(&self) -> *const mpc_pdata_satisfy_t {
        self.data.as_ptr() as *const _
    }

    pub fn string(&self) -> *const mpc_pdata_string_t {
        self.data.as_ptr() as *const _
    }

    pub fn apply(&self) -> *const mpc_pdata_apply_t {
        self.data.as_ptr() as *const _
    }

    pub fn apply_to(&self) -> *const mpc_pdata_apply_to_t {
        self.data.as_ptr() as *const _
    }

    pub fn predict(&self) -> *const mpc_pdata_predict_t {
        self.data.as_ptr() as *const _
    }

    pub fn not(&self) -> *const mpc_pdata_not_t {
        self.data.as_ptr() as *const _
    }

    pub fn repeat(&self) -> *const mpc_pdata_repeat_t {
        self.data.as_ptr() as *const _
    }

    pub fn and(&self) -> *const mpc_pdata_and_t {
        self.data.as_ptr() as *const _
    }

    pub fn or(&self) -> *const mpc_pdata_or_t {
        self.data.as_ptr() as *const _
    }
}

#[repr(C)]
struct mpc_result_t {
    pub data: [uint8_t, ..8u],
}

impl mpc_result_t {
    pub fn error(&self) -> *const mpc_err_t {
        self.data.as_ptr() as *const _
    }

    pub fn output(&self) -> *const mpc_val_t {
        self.data.as_ptr() as *const _
    }
}

#[repr(C)]
struct mpc_state_t {
    pos: c_long,
    row: c_long,
    col: c_long,
}

#[repr(C)]
struct mpc_err_t {
    state: mpc_state_t,
    expected_num: c_int,
    filename: *mut c_char,
    failure: *mut c_char,
    expected: *mut *mut c_char,
    recieved: c_char,
}

#[repr(C)]
struct mpc_parser_t {
    pub retained: c_char,
    pub name: *mut c_char,
    pub _type: c_char,
    pub data: mpc_pdata_t,
}

#[link_args = "./src/mpc.c -ledit -lm"]
extern {
    fn mpc_new(name: *const c_char) -> &mpc_parser_t;
    fn mpc_or(n: c_int, ...) -> &mpc_parser_t;
    fn mpc_sym(name: *const c_char) -> &mpc_parser_t;
    fn mpc_and(n: c_int, f: mpc_fold_t, ...) -> &mpc_parser_t;
    fn mpc_many(f: mpc_fold_t, parser: &mpc_parser_t) -> &mpc_parser_t;
    fn mpcf_strfold(n: c_int, xs: *mut *mut mpc_val_t) -> &mpc_val_t;
    fn mpc_parse(filename: *const c_char, string: *const c_char, p: &mpc_parser_t, r: &mpc_result_t) -> c_int;
    fn mpc_ast_print(a: *const mpc_ast_t);
    fn mpca_lang(flags: c_int, grammar: *const c_char, ...) -> &mpc_parser_t;
}

#[test]
fn it_works() {
    let adjective = unsafe {
        mpc_or(
            4,
            mpc_sym("wow".to_c_str().as_ptr()),
            mpc_sym("many".to_c_str().as_ptr()),
            mpc_sym("so".to_c_str().as_ptr()),
            mpc_sym("such".to_c_str().as_ptr())
        )
    };

    let noun = unsafe {
        mpc_or(
            5,
            mpc_sym("lisp".to_c_str().as_ptr()),
            mpc_sym("language".to_c_str().as_ptr()),
            mpc_sym("book".to_c_str().as_ptr()),
            mpc_sym("build".to_c_str().as_ptr()),
            mpc_sym("c".to_c_str().as_ptr())
        )
    };

//let number = unsafe { mpc_new("number".to_c_str().as_ptr()) };
}
