#![feature(link_args)]
#![allow(dead_code)]
#![allow(raw_pointer_deriving)]
#![allow(unused_imports)]

extern crate libc;
extern crate readline;

use std::{mem,ptr};
use std::ffi::CString;
use libc::{free};
use readline::{readline, add_history};

mod mpc;

#[test]
fn it_works() {
    unsafe {
        let adj = mpc::mpc_or(4,
                         mpc::mpc_sym(CString::from_slice("wow".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("so".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("many".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("such".as_bytes()).as_ptr()),
                        );
        let noun = mpc::mpc_or(5,
                         mpc::mpc_sym(CString::from_slice("lisp".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("book".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("c".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("language".as_bytes()).as_ptr()),
                         mpc::mpc_sym(CString::from_slice("build".as_bytes()).as_ptr()),
                        );
        let phrase = mpc::mpc_and(2,
                        Some(mpc::mpcf_strfold),
                        adj, noun, free,
                        );
        let doge = mpc::mpc_many(Some(mpc::mpcf_strfold), phrase);
        let input = readline("> ").unwrap();
        add_history(input.as_slice());
        let mut r = mpc::mpc_result_new();
        let ret = mpc::mpc_parse(
            CString::from_slice("<stdin>".as_bytes()).as_ptr(),
            CString::from_slice("wow lisp such language so c".as_bytes()).as_ptr(),
            doge,
            &mut r,
        );
        mpc::mpc_ast_print(r.output() as *mut mpc::mpc_ast_t);
        //println!("tag? {}", (*((*r).output() as *mut mpc::mpc_ast_t)).children_num);
        }
}
