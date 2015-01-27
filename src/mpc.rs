pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_longlong;
pub type __uint64_t = ::libc::c_ulonglong;
pub type __darwin_intptr_t = ::libc::c_long;
pub type __darwin_natural_t = ::libc::c_uint;
pub type __darwin_ct_rune_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 16us],
}
impl Union_Unnamed1 {
    pub unsafe fn __mbstate8(&mut self) -> *mut [::libc::c_char; 128us] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn _mbstateL(&mut self) -> *mut ::libc::c_longlong {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Union_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type __mbstate_t = Union_Unnamed1;
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::libc::c_long;
pub type __darwin_size_t = ::libc::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::libc::c_int;
pub type __darwin_clock_t = ::libc::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::libc::c_long;
pub type __darwin_time_t = ::libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::libc::c_uint;
pub type __darwin_fsfilcnt_t = ::libc::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::libc::c_uchar; 16us];
pub type __darwin_uuid_string_t = [::libc::c_char; 37us];
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<extern "C" fn
                                             (arg1: *mut ::libc::c_void)
                                             -> ()>,
    pub __arg: *mut ::libc::c_void,
    pub __next: *mut Struct___darwin_pthread_handler_rec,
}
impl ::std::default::Default for Struct___darwin_pthread_handler_rec {
    fn default() -> Struct___darwin_pthread_handler_rec {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_attr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 56us],
}
impl ::std::default::Default for Struct__opaque_pthread_attr_t {
    fn default() -> Struct__opaque_pthread_attr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_cond_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 40us],
}
impl ::std::default::Default for Struct__opaque_pthread_cond_t {
    fn default() -> Struct__opaque_pthread_cond_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_condattr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 8us],
}
impl ::std::default::Default for Struct__opaque_pthread_condattr_t {
    fn default() -> Struct__opaque_pthread_condattr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_mutex_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 56us],
}
impl ::std::default::Default for Struct__opaque_pthread_mutex_t {
    fn default() -> Struct__opaque_pthread_mutex_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_mutexattr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 8us],
}
impl ::std::default::Default for Struct__opaque_pthread_mutexattr_t {
    fn default() -> Struct__opaque_pthread_mutexattr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_once_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 8us],
}
impl ::std::default::Default for Struct__opaque_pthread_once_t {
    fn default() -> Struct__opaque_pthread_once_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_rwlock_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 192us],
}
impl ::std::default::Default for Struct__opaque_pthread_rwlock_t {
    fn default() -> Struct__opaque_pthread_rwlock_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_rwlockattr_t {
    pub __sig: ::libc::c_long,
    pub __opaque: [::libc::c_char; 16us],
}
impl ::std::default::Default for Struct__opaque_pthread_rwlockattr_t {
    fn default() -> Struct__opaque_pthread_rwlockattr_t {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__opaque_pthread_t {
    pub __sig: ::libc::c_long,
    pub __cleanup_stack: *mut Struct___darwin_pthread_handler_rec,
    pub __opaque: [::libc::c_char; 8176us],
}
impl ::std::default::Default for Struct__opaque_pthread_t {
    fn default() -> Struct__opaque_pthread_t {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __darwin_pthread_attr_t = Struct__opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = Struct__opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = Struct__opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::libc::c_ulong;
pub type __darwin_pthread_mutex_t = Struct__opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = Struct__opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = Struct__opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = Struct__opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = Struct__opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut Struct__opaque_pthread_t;
pub type __darwin_nl_item = ::libc::c_int;
pub type __darwin_wctrans_t = ::libc::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const P_ALL: ::libc::c_uint = 0;
pub const P_PID: ::libc::c_uint = 1;
pub const P_PGID: ::libc::c_uint = 2;
pub type idtype_t = Enum_Unnamed2;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_i386_thread_state {
    pub __eax: ::libc::c_uint,
    pub __ebx: ::libc::c_uint,
    pub __ecx: ::libc::c_uint,
    pub __edx: ::libc::c_uint,
    pub __edi: ::libc::c_uint,
    pub __esi: ::libc::c_uint,
    pub __ebp: ::libc::c_uint,
    pub __esp: ::libc::c_uint,
    pub __ss: ::libc::c_uint,
    pub __eflags: ::libc::c_uint,
    pub __eip: ::libc::c_uint,
    pub __cs: ::libc::c_uint,
    pub __ds: ::libc::c_uint,
    pub __es: ::libc::c_uint,
    pub __fs: ::libc::c_uint,
    pub __gs: ::libc::c_uint,
}
impl ::std::default::Default for Struct___darwin_i386_thread_state {
    fn default() -> Struct___darwin_i386_thread_state {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_fp_control {
    pub _bindgen_bitfield_1_: ::libc::c_ushort,
}
impl ::std::default::Default for Struct___darwin_fp_control {
    fn default() -> Struct___darwin_fp_control {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __darwin_fp_control_t = Struct___darwin_fp_control;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_fp_status {
    pub _bindgen_bitfield_1_: ::libc::c_ushort,
}
impl ::std::default::Default for Struct___darwin_fp_status {
    fn default() -> Struct___darwin_fp_status {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __darwin_fp_status_t = Struct___darwin_fp_status;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_mmst_reg {
    pub __mmst_reg: [::libc::c_char; 10us],
    pub __mmst_rsrv: [::libc::c_char; 6us],
}
impl ::std::default::Default for Struct___darwin_mmst_reg {
    fn default() -> Struct___darwin_mmst_reg {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_xmm_reg {
    pub __xmm_reg: [::libc::c_char; 16us],
}
impl ::std::default::Default for Struct___darwin_xmm_reg {
    fn default() -> Struct___darwin_xmm_reg {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_i386_float_state {
    pub __fpu_reserved: [::libc::c_int; 2us],
    pub __fpu_fcw: Struct___darwin_fp_control,
    pub __fpu_fsw: Struct___darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: Struct___darwin_mmst_reg,
    pub __fpu_stmm1: Struct___darwin_mmst_reg,
    pub __fpu_stmm2: Struct___darwin_mmst_reg,
    pub __fpu_stmm3: Struct___darwin_mmst_reg,
    pub __fpu_stmm4: Struct___darwin_mmst_reg,
    pub __fpu_stmm5: Struct___darwin_mmst_reg,
    pub __fpu_stmm6: Struct___darwin_mmst_reg,
    pub __fpu_stmm7: Struct___darwin_mmst_reg,
    pub __fpu_xmm0: Struct___darwin_xmm_reg,
    pub __fpu_xmm1: Struct___darwin_xmm_reg,
    pub __fpu_xmm2: Struct___darwin_xmm_reg,
    pub __fpu_xmm3: Struct___darwin_xmm_reg,
    pub __fpu_xmm4: Struct___darwin_xmm_reg,
    pub __fpu_xmm5: Struct___darwin_xmm_reg,
    pub __fpu_xmm6: Struct___darwin_xmm_reg,
    pub __fpu_xmm7: Struct___darwin_xmm_reg,
    pub __fpu_rsrv4: [::libc::c_char; 224us],
    pub __fpu_reserved1: ::libc::c_int,
}
impl ::std::default::Default for Struct___darwin_i386_float_state {
    fn default() -> Struct___darwin_i386_float_state {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_i386_avx_state {
    pub __fpu_reserved: [::libc::c_int; 2us],
    pub __fpu_fcw: Struct___darwin_fp_control,
    pub __fpu_fsw: Struct___darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: Struct___darwin_mmst_reg,
    pub __fpu_stmm1: Struct___darwin_mmst_reg,
    pub __fpu_stmm2: Struct___darwin_mmst_reg,
    pub __fpu_stmm3: Struct___darwin_mmst_reg,
    pub __fpu_stmm4: Struct___darwin_mmst_reg,
    pub __fpu_stmm5: Struct___darwin_mmst_reg,
    pub __fpu_stmm6: Struct___darwin_mmst_reg,
    pub __fpu_stmm7: Struct___darwin_mmst_reg,
    pub __fpu_xmm0: Struct___darwin_xmm_reg,
    pub __fpu_xmm1: Struct___darwin_xmm_reg,
    pub __fpu_xmm2: Struct___darwin_xmm_reg,
    pub __fpu_xmm3: Struct___darwin_xmm_reg,
    pub __fpu_xmm4: Struct___darwin_xmm_reg,
    pub __fpu_xmm5: Struct___darwin_xmm_reg,
    pub __fpu_xmm6: Struct___darwin_xmm_reg,
    pub __fpu_xmm7: Struct___darwin_xmm_reg,
    pub __fpu_rsrv4: [::libc::c_char; 224us],
    pub __fpu_reserved1: ::libc::c_int,
    pub __avx_reserved1: [::libc::c_char; 64us],
    pub __fpu_ymmh0: Struct___darwin_xmm_reg,
    pub __fpu_ymmh1: Struct___darwin_xmm_reg,
    pub __fpu_ymmh2: Struct___darwin_xmm_reg,
    pub __fpu_ymmh3: Struct___darwin_xmm_reg,
    pub __fpu_ymmh4: Struct___darwin_xmm_reg,
    pub __fpu_ymmh5: Struct___darwin_xmm_reg,
    pub __fpu_ymmh6: Struct___darwin_xmm_reg,
    pub __fpu_ymmh7: Struct___darwin_xmm_reg,
}
impl ::std::default::Default for Struct___darwin_i386_avx_state {
    fn default() -> Struct___darwin_i386_avx_state {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_i386_exception_state {
    pub __trapno: __uint16_t,
    pub __cpu: __uint16_t,
    pub __err: __uint32_t,
    pub __faultvaddr: __uint32_t,
}
impl ::std::default::Default for Struct___darwin_i386_exception_state {
    fn default() -> Struct___darwin_i386_exception_state {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_x86_debug_state32 {
    pub __dr0: ::libc::c_uint,
    pub __dr1: ::libc::c_uint,
    pub __dr2: ::libc::c_uint,
    pub __dr3: ::libc::c_uint,
    pub __dr4: ::libc::c_uint,
    pub __dr5: ::libc::c_uint,
    pub __dr6: ::libc::c_uint,
    pub __dr7: ::libc::c_uint,
}
impl ::std::default::Default for Struct___darwin_x86_debug_state32 {
    fn default() -> Struct___darwin_x86_debug_state32 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_x86_thread_state64 {
    pub __rax: __uint64_t,
    pub __rbx: __uint64_t,
    pub __rcx: __uint64_t,
    pub __rdx: __uint64_t,
    pub __rdi: __uint64_t,
    pub __rsi: __uint64_t,
    pub __rbp: __uint64_t,
    pub __rsp: __uint64_t,
    pub __r8: __uint64_t,
    pub __r9: __uint64_t,
    pub __r10: __uint64_t,
    pub __r11: __uint64_t,
    pub __r12: __uint64_t,
    pub __r13: __uint64_t,
    pub __r14: __uint64_t,
    pub __r15: __uint64_t,
    pub __rip: __uint64_t,
    pub __rflags: __uint64_t,
    pub __cs: __uint64_t,
    pub __fs: __uint64_t,
    pub __gs: __uint64_t,
}
impl ::std::default::Default for Struct___darwin_x86_thread_state64 {
    fn default() -> Struct___darwin_x86_thread_state64 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_x86_float_state64 {
    pub __fpu_reserved: [::libc::c_int; 2us],
    pub __fpu_fcw: Struct___darwin_fp_control,
    pub __fpu_fsw: Struct___darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: Struct___darwin_mmst_reg,
    pub __fpu_stmm1: Struct___darwin_mmst_reg,
    pub __fpu_stmm2: Struct___darwin_mmst_reg,
    pub __fpu_stmm3: Struct___darwin_mmst_reg,
    pub __fpu_stmm4: Struct___darwin_mmst_reg,
    pub __fpu_stmm5: Struct___darwin_mmst_reg,
    pub __fpu_stmm6: Struct___darwin_mmst_reg,
    pub __fpu_stmm7: Struct___darwin_mmst_reg,
    pub __fpu_xmm0: Struct___darwin_xmm_reg,
    pub __fpu_xmm1: Struct___darwin_xmm_reg,
    pub __fpu_xmm2: Struct___darwin_xmm_reg,
    pub __fpu_xmm3: Struct___darwin_xmm_reg,
    pub __fpu_xmm4: Struct___darwin_xmm_reg,
    pub __fpu_xmm5: Struct___darwin_xmm_reg,
    pub __fpu_xmm6: Struct___darwin_xmm_reg,
    pub __fpu_xmm7: Struct___darwin_xmm_reg,
    pub __fpu_xmm8: Struct___darwin_xmm_reg,
    pub __fpu_xmm9: Struct___darwin_xmm_reg,
    pub __fpu_xmm10: Struct___darwin_xmm_reg,
    pub __fpu_xmm11: Struct___darwin_xmm_reg,
    pub __fpu_xmm12: Struct___darwin_xmm_reg,
    pub __fpu_xmm13: Struct___darwin_xmm_reg,
    pub __fpu_xmm14: Struct___darwin_xmm_reg,
    pub __fpu_xmm15: Struct___darwin_xmm_reg,
    pub __fpu_rsrv4: [::libc::c_char; 96us],
    pub __fpu_reserved1: ::libc::c_int,
}
impl ::std::default::Default for Struct___darwin_x86_float_state64 {
    fn default() -> Struct___darwin_x86_float_state64 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_x86_avx_state64 {
    pub __fpu_reserved: [::libc::c_int; 2us],
    pub __fpu_fcw: Struct___darwin_fp_control,
    pub __fpu_fsw: Struct___darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: Struct___darwin_mmst_reg,
    pub __fpu_stmm1: Struct___darwin_mmst_reg,
    pub __fpu_stmm2: Struct___darwin_mmst_reg,
    pub __fpu_stmm3: Struct___darwin_mmst_reg,
    pub __fpu_stmm4: Struct___darwin_mmst_reg,
    pub __fpu_stmm5: Struct___darwin_mmst_reg,
    pub __fpu_stmm6: Struct___darwin_mmst_reg,
    pub __fpu_stmm7: Struct___darwin_mmst_reg,
    pub __fpu_xmm0: Struct___darwin_xmm_reg,
    pub __fpu_xmm1: Struct___darwin_xmm_reg,
    pub __fpu_xmm2: Struct___darwin_xmm_reg,
    pub __fpu_xmm3: Struct___darwin_xmm_reg,
    pub __fpu_xmm4: Struct___darwin_xmm_reg,
    pub __fpu_xmm5: Struct___darwin_xmm_reg,
    pub __fpu_xmm6: Struct___darwin_xmm_reg,
    pub __fpu_xmm7: Struct___darwin_xmm_reg,
    pub __fpu_xmm8: Struct___darwin_xmm_reg,
    pub __fpu_xmm9: Struct___darwin_xmm_reg,
    pub __fpu_xmm10: Struct___darwin_xmm_reg,
    pub __fpu_xmm11: Struct___darwin_xmm_reg,
    pub __fpu_xmm12: Struct___darwin_xmm_reg,
    pub __fpu_xmm13: Struct___darwin_xmm_reg,
    pub __fpu_xmm14: Struct___darwin_xmm_reg,
    pub __fpu_xmm15: Struct___darwin_xmm_reg,
    pub __fpu_rsrv4: [::libc::c_char; 96us],
    pub __fpu_reserved1: ::libc::c_int,
    pub __avx_reserved1: [::libc::c_char; 64us],
    pub __fpu_ymmh0: Struct___darwin_xmm_reg,
    pub __fpu_ymmh1: Struct___darwin_xmm_reg,
    pub __fpu_ymmh2: Struct___darwin_xmm_reg,
    pub __fpu_ymmh3: Struct___darwin_xmm_reg,
    pub __fpu_ymmh4: Struct___darwin_xmm_reg,
    pub __fpu_ymmh5: Struct___darwin_xmm_reg,
    pub __fpu_ymmh6: Struct___darwin_xmm_reg,
    pub __fpu_ymmh7: Struct___darwin_xmm_reg,
    pub __fpu_ymmh8: Struct___darwin_xmm_reg,
    pub __fpu_ymmh9: Struct___darwin_xmm_reg,
    pub __fpu_ymmh10: Struct___darwin_xmm_reg,
    pub __fpu_ymmh11: Struct___darwin_xmm_reg,
    pub __fpu_ymmh12: Struct___darwin_xmm_reg,
    pub __fpu_ymmh13: Struct___darwin_xmm_reg,
    pub __fpu_ymmh14: Struct___darwin_xmm_reg,
    pub __fpu_ymmh15: Struct___darwin_xmm_reg,
}
impl ::std::default::Default for Struct___darwin_x86_avx_state64 {
    fn default() -> Struct___darwin_x86_avx_state64 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_x86_exception_state64 {
    pub __trapno: __uint16_t,
    pub __cpu: __uint16_t,
    pub __err: __uint32_t,
    pub __faultvaddr: __uint64_t,
}
impl ::std::default::Default for Struct___darwin_x86_exception_state64 {
    fn default() -> Struct___darwin_x86_exception_state64 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_x86_debug_state64 {
    pub __dr0: __uint64_t,
    pub __dr1: __uint64_t,
    pub __dr2: __uint64_t,
    pub __dr3: __uint64_t,
    pub __dr4: __uint64_t,
    pub __dr5: __uint64_t,
    pub __dr6: __uint64_t,
    pub __dr7: __uint64_t,
}
impl ::std::default::Default for Struct___darwin_x86_debug_state64 {
    fn default() -> Struct___darwin_x86_debug_state64 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_mcontext32 {
    pub __es: Struct___darwin_i386_exception_state,
    pub __ss: Struct___darwin_i386_thread_state,
    pub __fs: Struct___darwin_i386_float_state,
}
impl ::std::default::Default for Struct___darwin_mcontext32 {
    fn default() -> Struct___darwin_mcontext32 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_mcontext_avx32 {
    pub __es: Struct___darwin_i386_exception_state,
    pub __ss: Struct___darwin_i386_thread_state,
    pub __fs: Struct___darwin_i386_avx_state,
}
impl ::std::default::Default for Struct___darwin_mcontext_avx32 {
    fn default() -> Struct___darwin_mcontext_avx32 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_mcontext64 {
    pub __es: Struct___darwin_x86_exception_state64,
    pub __ss: Struct___darwin_x86_thread_state64,
    pub __fs: Struct___darwin_x86_float_state64,
}
impl ::std::default::Default for Struct___darwin_mcontext64 {
    fn default() -> Struct___darwin_mcontext64 {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_mcontext_avx64 {
    pub __es: Struct___darwin_x86_exception_state64,
    pub __ss: Struct___darwin_x86_thread_state64,
    pub __fs: Struct___darwin_x86_avx_state64,
}
impl ::std::default::Default for Struct___darwin_mcontext_avx64 {
    fn default() -> Struct___darwin_mcontext_avx64 {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type mcontext_t = *mut Struct___darwin_mcontext64;
pub type pthread_attr_t = __darwin_pthread_attr_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_sigaltstack {
    pub ss_sp: *mut ::libc::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::libc::c_int,
}
impl ::std::default::Default for Struct___darwin_sigaltstack {
    fn default() -> Struct___darwin_sigaltstack {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type stack_t = Struct___darwin_sigaltstack;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___darwin_ucontext {
    pub uc_onstack: ::libc::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: Struct___darwin_sigaltstack,
    pub uc_link: *mut Struct___darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut Struct___darwin_mcontext64,
}
impl ::std::default::Default for Struct___darwin_ucontext {
    fn default() -> Struct___darwin_ucontext {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ucontext_t = Struct___darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type size_t = __darwin_size_t;
pub type uid_t = __darwin_uid_t;
#[repr(C)]
#[derive(Copy)]
pub struct Union_sigval {
    pub _bindgen_data_: [u64; 1us],
}
impl Union_sigval {
    pub unsafe fn sival_int(&mut self) -> *mut ::libc::c_int {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn sival_ptr(&mut self) -> *mut *mut ::libc::c_void {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_sigval {
    fn default() -> Union_sigval { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigevent {
    pub sigev_notify: ::libc::c_int,
    pub sigev_signo: ::libc::c_int,
    pub sigev_value: Union_sigval,
    pub sigev_notify_function: ::std::option::Option<extern "C" fn
                                                         (arg1: Union_sigval)
                                                         -> ()>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
impl ::std::default::Default for Struct_sigevent {
    fn default() -> Struct_sigevent { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___siginfo {
    pub si_signo: ::libc::c_int,
    pub si_errno: ::libc::c_int,
    pub si_code: ::libc::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::libc::c_int,
    pub si_addr: *mut ::libc::c_void,
    pub si_value: Union_sigval,
    pub si_band: ::libc::c_long,
    pub __pad: [::libc::c_ulong; 7us],
}
impl ::std::default::Default for Struct___siginfo {
    fn default() -> Struct___siginfo { unsafe { ::std::mem::zeroed() } }
}
pub type siginfo_t = Struct___siginfo;
#[repr(C)]
#[derive(Copy)]
pub struct Union___sigaction_u {
    pub _bindgen_data_: [u64; 1us],
}
impl Union___sigaction_u {
    pub unsafe fn __sa_handler(&mut self)
     -> *mut ::std::option::Option<extern "C" fn(arg1: ::libc::c_int) -> ()> {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn __sa_sigaction(&mut self)
     ->
         *mut ::std::option::Option<extern "C" fn
                                        (arg1: ::libc::c_int,
                                         arg2: *mut Struct___siginfo,
                                         arg3: *mut ::libc::c_void) -> ()> {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union___sigaction_u {
    fn default() -> Union___sigaction_u { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___sigaction {
    pub __sigaction_u: Union___sigaction_u,
    pub sa_tramp: ::std::option::Option<extern "C" fn
                                            (arg1: *mut ::libc::c_void,
                                             arg2: ::libc::c_int,
                                             arg3: ::libc::c_int,
                                             arg4: *mut siginfo_t,
                                             arg5: *mut ::libc::c_void)
                                            -> ()>,
    pub sa_mask: sigset_t,
    pub sa_flags: ::libc::c_int,
}
impl ::std::default::Default for Struct___sigaction {
    fn default() -> Struct___sigaction { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigaction {
    pub __sigaction_u: Union___sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::libc::c_int,
}
impl ::std::default::Default for Struct_sigaction {
    fn default() -> Struct_sigaction { unsafe { ::std::mem::zeroed() } }
}
pub type sig_t =
    ::std::option::Option<extern "C" fn(arg1: ::libc::c_int) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigvec {
    pub sv_handler: ::std::option::Option<extern "C" fn(arg1: ::libc::c_int)
                                              -> ()>,
    pub sv_mask: ::libc::c_int,
    pub sv_flags: ::libc::c_int,
}
impl ::std::default::Default for Struct_sigvec {
    fn default() -> Struct_sigvec { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigstack {
    pub ss_sp: *mut ::libc::c_char,
    pub ss_onstack: ::libc::c_int,
}
impl ::std::default::Default for Struct_sigstack {
    fn default() -> Struct_sigstack { unsafe { ::std::mem::zeroed() } }
}
pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_longlong;
pub type uint8_t = ::libc::c_uchar;
pub type uint16_t = ::libc::c_ushort;
pub type uint32_t = ::libc::c_uint;
pub type uint64_t = ::libc::c_ulonglong;
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type intptr_t = __darwin_intptr_t;
pub type uintptr_t = ::libc::c_ulong;
pub type intmax_t = ::libc::c_long;
pub type uintmax_t = ::libc::c_ulong;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
impl ::std::default::Default for Struct_timeval {
    fn default() -> Struct_timeval { unsafe { ::std::mem::zeroed() } }
}
pub type rlim_t = __uint64_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rusage {
    pub ru_utime: Struct_timeval,
    pub ru_stime: Struct_timeval,
    pub ru_maxrss: ::libc::c_long,
    pub ru_ixrss: ::libc::c_long,
    pub ru_idrss: ::libc::c_long,
    pub ru_isrss: ::libc::c_long,
    pub ru_minflt: ::libc::c_long,
    pub ru_majflt: ::libc::c_long,
    pub ru_nswap: ::libc::c_long,
    pub ru_inblock: ::libc::c_long,
    pub ru_oublock: ::libc::c_long,
    pub ru_msgsnd: ::libc::c_long,
    pub ru_msgrcv: ::libc::c_long,
    pub ru_nsignals: ::libc::c_long,
    pub ru_nvcsw: ::libc::c_long,
    pub ru_nivcsw: ::libc::c_long,
}
impl ::std::default::Default for Struct_rusage {
    fn default() -> Struct_rusage { unsafe { ::std::mem::zeroed() } }
}
pub type rusage_info_t = *mut ::libc::c_void;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rusage_info_v0 {
    pub ri_uuid: [uint8_t; 16us],
    pub ri_user_time: uint64_t,
    pub ri_system_time: uint64_t,
    pub ri_pkg_idle_wkups: uint64_t,
    pub ri_interrupt_wkups: uint64_t,
    pub ri_pageins: uint64_t,
    pub ri_wired_size: uint64_t,
    pub ri_resident_size: uint64_t,
    pub ri_phys_footprint: uint64_t,
    pub ri_proc_start_abstime: uint64_t,
    pub ri_proc_exit_abstime: uint64_t,
}
impl ::std::default::Default for Struct_rusage_info_v0 {
    fn default() -> Struct_rusage_info_v0 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rusage_info_v1 {
    pub ri_uuid: [uint8_t; 16us],
    pub ri_user_time: uint64_t,
    pub ri_system_time: uint64_t,
    pub ri_pkg_idle_wkups: uint64_t,
    pub ri_interrupt_wkups: uint64_t,
    pub ri_pageins: uint64_t,
    pub ri_wired_size: uint64_t,
    pub ri_resident_size: uint64_t,
    pub ri_phys_footprint: uint64_t,
    pub ri_proc_start_abstime: uint64_t,
    pub ri_proc_exit_abstime: uint64_t,
    pub ri_child_user_time: uint64_t,
    pub ri_child_system_time: uint64_t,
    pub ri_child_pkg_idle_wkups: uint64_t,
    pub ri_child_interrupt_wkups: uint64_t,
    pub ri_child_pageins: uint64_t,
    pub ri_child_elapsed_abstime: uint64_t,
}
impl ::std::default::Default for Struct_rusage_info_v1 {
    fn default() -> Struct_rusage_info_v1 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rusage_info_v2 {
    pub ri_uuid: [uint8_t; 16us],
    pub ri_user_time: uint64_t,
    pub ri_system_time: uint64_t,
    pub ri_pkg_idle_wkups: uint64_t,
    pub ri_interrupt_wkups: uint64_t,
    pub ri_pageins: uint64_t,
    pub ri_wired_size: uint64_t,
    pub ri_resident_size: uint64_t,
    pub ri_phys_footprint: uint64_t,
    pub ri_proc_start_abstime: uint64_t,
    pub ri_proc_exit_abstime: uint64_t,
    pub ri_child_user_time: uint64_t,
    pub ri_child_system_time: uint64_t,
    pub ri_child_pkg_idle_wkups: uint64_t,
    pub ri_child_interrupt_wkups: uint64_t,
    pub ri_child_pageins: uint64_t,
    pub ri_child_elapsed_abstime: uint64_t,
    pub ri_diskio_bytesread: uint64_t,
    pub ri_diskio_byteswritten: uint64_t,
}
impl ::std::default::Default for Struct_rusage_info_v2 {
    fn default() -> Struct_rusage_info_v2 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rusage_info_v3 {
    pub ri_uuid: [uint8_t; 16us],
    pub ri_user_time: uint64_t,
    pub ri_system_time: uint64_t,
    pub ri_pkg_idle_wkups: uint64_t,
    pub ri_interrupt_wkups: uint64_t,
    pub ri_pageins: uint64_t,
    pub ri_wired_size: uint64_t,
    pub ri_resident_size: uint64_t,
    pub ri_phys_footprint: uint64_t,
    pub ri_proc_start_abstime: uint64_t,
    pub ri_proc_exit_abstime: uint64_t,
    pub ri_child_user_time: uint64_t,
    pub ri_child_system_time: uint64_t,
    pub ri_child_pkg_idle_wkups: uint64_t,
    pub ri_child_interrupt_wkups: uint64_t,
    pub ri_child_pageins: uint64_t,
    pub ri_child_elapsed_abstime: uint64_t,
    pub ri_diskio_bytesread: uint64_t,
    pub ri_diskio_byteswritten: uint64_t,
    pub ri_cpu_time_qos_default: uint64_t,
    pub ri_cpu_time_qos_maintenance: uint64_t,
    pub ri_cpu_time_qos_background: uint64_t,
    pub ri_cpu_time_qos_utility: uint64_t,
    pub ri_cpu_time_qos_legacy: uint64_t,
    pub ri_cpu_time_qos_user_initiated: uint64_t,
    pub ri_cpu_time_qos_user_interactive: uint64_t,
    pub ri_billed_system_time: uint64_t,
    pub ri_serviced_system_time: uint64_t,
}
impl ::std::default::Default for Struct_rusage_info_v3 {
    fn default() -> Struct_rusage_info_v3 { unsafe { ::std::mem::zeroed() } }
}
pub type rusage_info_current = Struct_rusage_info_v3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
impl ::std::default::Default for Struct_rlimit {
    fn default() -> Struct_rlimit { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_proc_rlimit_control_wakeupmon {
    pub wm_flags: uint32_t,
    pub wm_rate: int32_t,
}
impl ::std::default::Default for Struct_proc_rlimit_control_wakeupmon {
    fn default() -> Struct_proc_rlimit_control_wakeupmon {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_wait {
    pub _bindgen_data_: [u32; 1us],
}
impl Union_wait {
    pub unsafe fn w_status(&mut self) -> *mut ::libc::c_int {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn w_T(&mut self) -> *mut Struct_Unnamed3 {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn w_S(&mut self) -> *mut Struct_Unnamed4 {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_wait {
    fn default() -> Union_wait { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Struct_Unnamed3 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Struct_Unnamed4 { unsafe { ::std::mem::zeroed() } }
}
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
pub type wchar_t = __darwin_wchar_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub quot: ::libc::c_int,
    pub rem: ::libc::c_int,
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Struct_Unnamed5 { unsafe { ::std::mem::zeroed() } }
}
pub type div_t = Struct_Unnamed5;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub quot: ::libc::c_long,
    pub rem: ::libc::c_long,
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Struct_Unnamed6 { unsafe { ::std::mem::zeroed() } }
}
pub type ldiv_t = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub quot: ::libc::c_longlong,
    pub rem: ::libc::c_longlong,
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Struct_Unnamed7 { unsafe { ::std::mem::zeroed() } }
}
pub type lldiv_t = Struct_Unnamed7;
pub type u_int8_t = ::libc::c_uchar;
pub type u_int16_t = ::libc::c_ushort;
pub type u_int32_t = ::libc::c_uint;
pub type u_int64_t = ::libc::c_ulonglong;
pub type register_t = int64_t;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = int64_t;
pub type user_long_t = int64_t;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = int64_t;
pub type user_off_t = int64_t;
pub type syscall_arg_t = u_int64_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
pub type va_list = __builtin_va_list;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___sbuf {
    pub _base: *mut ::libc::c_uchar,
    pub _size: ::libc::c_int,
}
impl ::std::default::Default for Struct___sbuf {
    fn default() -> Struct___sbuf { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct___sFILEX { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct___sFILE {
    pub _p: *mut ::libc::c_uchar,
    pub _r: ::libc::c_int,
    pub _w: ::libc::c_int,
    pub _flags: ::libc::c_short,
    pub _file: ::libc::c_short,
    pub _bf: Struct___sbuf,
    pub _lbfsize: ::libc::c_int,
    pub _cookie: *mut ::libc::c_void,
    pub _close: ::std::option::Option<extern "C" fn(arg1: *mut ::libc::c_void)
                                          -> ::libc::c_int>,
    pub _read: ::std::option::Option<extern "C" fn
                                         (arg1: *mut ::libc::c_void,
                                          arg2: *mut ::libc::c_char,
                                          arg3: ::libc::c_int)
                                         -> ::libc::c_int>,
    pub _seek: ::std::option::Option<extern "C" fn
                                         (arg1: *mut ::libc::c_void,
                                          arg2: fpos_t, arg3: ::libc::c_int)
                                         -> fpos_t>,
    pub _write: ::std::option::Option<extern "C" fn
                                          (arg1: *mut ::libc::c_void,
                                           arg2: *const ::libc::c_char,
                                           arg3: ::libc::c_int)
                                          -> ::libc::c_int>,
    pub _ub: Struct___sbuf,
    pub _extra: *mut Struct___sFILEX,
    pub _ur: ::libc::c_int,
    pub _ubuf: [::libc::c_uchar; 3us],
    pub _nbuf: [::libc::c_uchar; 1us],
    pub _lb: Struct___sbuf,
    pub _blksize: ::libc::c_int,
    pub _offset: fpos_t,
}
impl ::std::default::Default for Struct___sFILE {
    fn default() -> Struct___sFILE { unsafe { ::std::mem::zeroed() } }
}
pub type FILE = Struct___sFILE;
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
pub type __gnuc_va_list = __builtin_va_list;
pub type rsize_t = __darwin_size_t;
pub type errno_t = ::libc::c_int;
pub type float_t = ::libc::c_float;
pub type double_t = ::libc::c_double;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___float2 {
    pub __sinval: ::libc::c_float,
    pub __cosval: ::libc::c_float,
}
impl ::std::default::Default for Struct___float2 {
    fn default() -> Struct___float2 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___double2 {
    pub __sinval: ::libc::c_double,
    pub __cosval: ::libc::c_double,
}
impl ::std::default::Default for Struct___double2 {
    fn default() -> Struct___double2 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_exception {
    pub _type: ::libc::c_int,
    pub name: *mut ::libc::c_char,
    pub arg1: ::libc::c_double,
    pub arg2: ::libc::c_double,
    pub retval: ::libc::c_double,
}
impl ::std::default::Default for Struct_exception {
    fn default() -> Struct_exception { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub pos: ::libc::c_long,
    pub row: ::libc::c_long,
    pub col: ::libc::c_long,
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Struct_Unnamed8 { unsafe { ::std::mem::zeroed() } }
}
pub type mpc_state_t = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub state: mpc_state_t,
    pub expected_num: ::libc::c_int,
    pub filename: *mut ::libc::c_char,
    pub failure: *mut ::libc::c_char,
    pub expected: *mut *mut ::libc::c_char,
    pub recieved: ::libc::c_char,
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Struct_Unnamed9 { unsafe { ::std::mem::zeroed() } }
}
pub type mpc_err_t = Struct_Unnamed9;
pub type mpc_val_t = ::libc::c_void;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed10 {
    pub _bindgen_data_: [u64; 1us],
}
impl Union_Unnamed10 {
    pub unsafe fn error(&mut self) -> *mut *mut mpc_err_t {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn output(&mut self) -> *mut *mut mpc_val_t {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed10 {
    fn default() -> Union_Unnamed10 { unsafe { ::std::mem::zeroed() } }
}

pub fn mpc_result_new() -> mpc_result_t {
    unsafe {::std::mem::zeroed() }
}
pub type mpc_result_t = Union_Unnamed10;
pub enum Struct_mpc_parser_t { }
pub type mpc_parser_t = Struct_mpc_parser_t;
pub type mpc_dtor_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut mpc_val_t) -> ()>;
pub type mpc_ctor_t =
    ::std::option::Option<unsafe extern "C" fn() -> *mut mpc_val_t>;
pub type mpc_apply_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut mpc_val_t)
                              -> *mut mpc_val_t>;
pub type mpc_apply_to_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut mpc_val_t,
                               arg2: *mut ::libc::c_void) -> *mut mpc_val_t>;
pub type mpc_fold_t =
    ::std::option::Option<unsafe extern "C" fn
                              (arg1: ::libc::c_int, arg2: *mut *mut mpc_val_t)
                              -> *mut mpc_val_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_mpc_ast_t {
    pub tag: *mut ::libc::c_char,
    pub contents: *mut ::libc::c_char,
    pub state: mpc_state_t,
    pub children_num: ::libc::c_int,
    pub children: *mut *mut Struct_mpc_ast_t,
}
impl ::std::default::Default for Struct_mpc_ast_t {
    fn default() -> Struct_mpc_ast_t { unsafe { ::std::mem::zeroed() } }
}
pub type mpc_ast_t = Struct_mpc_ast_t;
pub type Enum_Unnamed11 = ::libc::c_uint;
pub const MPCA_LANG_DEFAULT: ::libc::c_uint = 0;
pub const MPCA_LANG_PREDICTIVE: ::libc::c_uint = 1;
pub const MPCA_LANG_WHITESPACE_SENSITIVE: ::libc::c_uint = 2;
pub type __builtin_va_list = [__va_list_tag; 1us];
pub type __va_list_tag = Struct___va_list_tag;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct___va_list_tag {
    fn default() -> Struct___va_list_tag { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut __mb_cur_max: ::libc::c_int;
    pub static mut suboptarg: *mut ::libc::c_char;
    pub static mut __stdinp: *mut FILE;
    pub static mut __stdoutp: *mut FILE;
    pub static mut __stderrp: *mut FILE;
    pub static sys_nerr: ::libc::c_int;
    pub static mut sys_errlist: *const *const ::libc::c_char;
    pub static mut signgam: ::libc::c_int;
}
#[link_args = "./src/mpc.c -ledit -lm"]
extern "C" {
    pub fn signal(arg1: ::libc::c_int,
                  arg2:
                      ::std::option::Option<extern "C" fn(arg1: ::libc::c_int)
                                                -> ()>)
     ->
         ::std::option::Option<extern "C" fn
                                   (arg1: ::libc::c_int,
                                    arg2:
                                        ::std::option::Option<extern "C" fn
                                                                  (arg1:
                                                                       ::libc::c_int)
                                                                  -> ()>)
                                   -> ()>;
    pub fn getpriority(arg1: ::libc::c_int, arg2: id_t) -> ::libc::c_int;
    pub fn getiopolicy_np(arg1: ::libc::c_int, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn getrlimit(arg1: ::libc::c_int, arg2: *mut Struct_rlimit)
     -> ::libc::c_int;
    pub fn getrusage(arg1: ::libc::c_int, arg2: *mut Struct_rusage)
     -> ::libc::c_int;
    pub fn setpriority(arg1: ::libc::c_int, arg2: id_t, arg3: ::libc::c_int)
     -> ::libc::c_int;
    pub fn setiopolicy_np(arg1: ::libc::c_int, arg2: ::libc::c_int,
                          arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn setrlimit(arg1: ::libc::c_int, arg2: *const Struct_rlimit)
     -> ::libc::c_int;
    pub fn wait(arg1: *mut ::libc::c_int) -> pid_t;
    pub fn waitpid(arg1: pid_t, arg2: *mut ::libc::c_int, arg3: ::libc::c_int)
     -> pid_t;
    pub fn waitid(arg1: idtype_t, arg2: id_t, arg3: *mut siginfo_t,
                  arg4: ::libc::c_int) -> ::libc::c_int;
    pub fn wait3(arg1: *mut ::libc::c_int, arg2: ::libc::c_int,
                 arg3: *mut Struct_rusage) -> pid_t;
    pub fn wait4(arg1: pid_t, arg2: *mut ::libc::c_int, arg3: ::libc::c_int,
                 arg4: *mut Struct_rusage) -> pid_t;
    pub fn alloca(arg1: size_t) -> *mut ::libc::c_void;
    pub fn abort() -> ();
    pub fn abs(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn atexit(arg1: ::std::option::Option<extern "C" fn() -> ()>)
     -> ::libc::c_int;
    pub fn atof(arg1: *const ::libc::c_char) -> ::libc::c_double;
    pub fn atoi(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn atol(arg1: *const ::libc::c_char) -> ::libc::c_long;
    pub fn atoll(arg1: *const ::libc::c_char) -> ::libc::c_longlong;
    pub fn bsearch(arg1: *const ::libc::c_void, arg2: *const ::libc::c_void,
                   arg3: size_t, arg4: size_t,
                   arg5:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *const ::libc::c_void,
                                                  arg2: *const ::libc::c_void)
                                                 -> ::libc::c_int>)
     -> *mut ::libc::c_void;
    pub fn calloc(arg1: size_t, arg2: size_t) -> *mut ::libc::c_void;
    pub fn div(arg1: ::libc::c_int, arg2: ::libc::c_int) -> div_t;
    pub fn exit(arg1: ::libc::c_int) -> ();
    pub fn free(arg1: *mut ::libc::c_void) -> ();
    pub fn getenv(arg1: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn labs(arg1: ::libc::c_long) -> ::libc::c_long;
    pub fn ldiv(arg1: ::libc::c_long, arg2: ::libc::c_long) -> ldiv_t;
    pub fn llabs(arg1: ::libc::c_longlong) -> ::libc::c_longlong;
    pub fn lldiv(arg1: ::libc::c_longlong, arg2: ::libc::c_longlong)
     -> lldiv_t;
    pub fn malloc(arg1: size_t) -> *mut ::libc::c_void;
    pub fn mblen(arg1: *const ::libc::c_char, arg2: size_t) -> ::libc::c_int;
    pub fn mbstowcs(arg1: *mut wchar_t, arg2: *const ::libc::c_char,
                    arg3: size_t) -> size_t;
    pub fn mbtowc(arg1: *mut wchar_t, arg2: *const ::libc::c_char,
                  arg3: size_t) -> ::libc::c_int;
    pub fn posix_memalign(arg1: *mut *mut ::libc::c_void, arg2: size_t,
                          arg3: size_t) -> ::libc::c_int;
    pub fn qsort(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                 arg4:
                     ::std::option::Option<extern "C" fn
                                               (arg1: *const ::libc::c_void,
                                                arg2: *const ::libc::c_void)
                                               -> ::libc::c_int>) -> ();
    pub fn rand() -> ::libc::c_int;
    pub fn realloc(arg1: *mut ::libc::c_void, arg2: size_t)
     -> *mut ::libc::c_void;
    pub fn srand(arg1: ::libc::c_uint) -> ();
    pub fn strtod(arg1: *const ::libc::c_char, arg2: *mut *mut ::libc::c_char)
     -> ::libc::c_double;
    pub fn strtof(arg1: *const ::libc::c_char, arg2: *mut *mut ::libc::c_char)
     -> ::libc::c_float;
    pub fn strtol(arg1: *const ::libc::c_char, arg2: *mut *mut ::libc::c_char,
                  arg3: ::libc::c_int) -> ::libc::c_long;
    pub fn strtold(arg1: *const ::libc::c_char,
                   arg2: *mut *mut ::libc::c_char) -> ::libc::c_double;
    pub fn strtoll(arg1: *const ::libc::c_char,
                   arg2: *mut *mut ::libc::c_char, arg3: ::libc::c_int)
     -> ::libc::c_longlong;
    pub fn strtoul(arg1: *const ::libc::c_char,
                   arg2: *mut *mut ::libc::c_char, arg3: ::libc::c_int)
     -> ::libc::c_ulong;
    pub fn strtoull(arg1: *const ::libc::c_char,
                    arg2: *mut *mut ::libc::c_char, arg3: ::libc::c_int)
     -> ::libc::c_ulonglong;
    pub fn system(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn wcstombs(arg1: *mut ::libc::c_char, arg2: *const wchar_t,
                    arg3: size_t) -> size_t;
    pub fn wctomb(arg1: *mut ::libc::c_char, arg2: wchar_t) -> ::libc::c_int;
    pub fn _Exit(arg1: ::libc::c_int) -> ();
    pub fn a64l(arg1: *const ::libc::c_char) -> ::libc::c_long;
    pub fn drand48() -> ::libc::c_double;
    pub fn ecvt(arg1: ::libc::c_double, arg2: ::libc::c_int,
                arg3: *mut ::libc::c_int, arg4: *mut ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn erand48(arg1: *mut ::libc::c_ushort) -> ::libc::c_double;
    pub fn fcvt(arg1: ::libc::c_double, arg2: ::libc::c_int,
                arg3: *mut ::libc::c_int, arg4: *mut ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn gcvt(arg1: ::libc::c_double, arg2: ::libc::c_int,
                arg3: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn getsubopt(arg1: *mut *mut ::libc::c_char,
                     arg2: *const *mut ::libc::c_char,
                     arg3: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn grantpt(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn initstate(arg1: ::libc::c_uint, arg2: *mut ::libc::c_char,
                     arg3: size_t) -> *mut ::libc::c_char;
    pub fn jrand48(arg1: *mut ::libc::c_ushort) -> ::libc::c_long;
    pub fn l64a(arg1: ::libc::c_long) -> *mut ::libc::c_char;
    pub fn lcong48(arg1: *mut ::libc::c_ushort) -> ();
    pub fn lrand48() -> ::libc::c_long;
    pub fn mktemp(arg1: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn mkstemp(arg1: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn mrand48() -> ::libc::c_long;
    pub fn nrand48(arg1: *mut ::libc::c_ushort) -> ::libc::c_long;
    pub fn posix_openpt(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn ptsname(arg1: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn putenv(arg1: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn random() -> ::libc::c_long;
    pub fn rand_r(arg1: *mut ::libc::c_uint) -> ::libc::c_int;
    pub fn realpath(arg1: *const ::libc::c_char, arg2: *mut ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn seed48(arg1: *mut ::libc::c_ushort) -> *mut ::libc::c_ushort;
    pub fn setenv(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char,
                  arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn setkey(arg1: *const ::libc::c_char) -> ();
    pub fn setstate(arg1: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn srand48(arg1: ::libc::c_long) -> ();
    pub fn srandom(arg1: ::libc::c_uint) -> ();
    pub fn unlockpt(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn unsetenv(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn arc4random() -> u_int32_t;
    pub fn arc4random_addrandom(arg1: *mut ::libc::c_uchar,
                                arg2: ::libc::c_int) -> ();
    pub fn arc4random_buf(arg1: *mut ::libc::c_void, arg2: size_t) -> ();
    pub fn arc4random_stir() -> ();
    pub fn arc4random_uniform(arg1: u_int32_t) -> u_int32_t;
    pub fn atexit_b(arg1: ::libc::c_void) -> ::libc::c_int;
    pub fn bsearch_b(arg1: *const ::libc::c_void, arg2: *const ::libc::c_void,
                     arg3: size_t, arg4: size_t, arg5: ::libc::c_void)
     -> *mut ::libc::c_void;
    pub fn cgetcap(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn cgetclose() -> ::libc::c_int;
    pub fn cgetent(arg1: *mut *mut ::libc::c_char,
                   arg2: *mut *mut ::libc::c_char,
                   arg3: *const ::libc::c_char) -> ::libc::c_int;
    pub fn cgetfirst(arg1: *mut *mut ::libc::c_char,
                     arg2: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn cgetmatch(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn cgetnext(arg1: *mut *mut ::libc::c_char,
                    arg2: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn cgetnum(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn cgetset(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn cgetstr(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn cgetustr(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                    arg3: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn daemon(arg1: ::libc::c_int, arg2: ::libc::c_int) -> ::libc::c_int;
    pub fn devname(arg1: dev_t, arg2: mode_t) -> *mut ::libc::c_char;
    pub fn devname_r(arg1: dev_t, arg2: mode_t, buf: *mut ::libc::c_char,
                     len: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn getbsize(arg1: *mut ::libc::c_int, arg2: *mut ::libc::c_long)
     -> *mut ::libc::c_char;
    pub fn getloadavg(arg1: *mut ::libc::c_double, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn getprogname() -> *const ::libc::c_char;
    pub fn heapsort(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                    arg4:
                        ::std::option::Option<extern "C" fn
                                                  (arg1:
                                                       *const ::libc::c_void,
                                                   arg2:
                                                       *const ::libc::c_void)
                                                  -> ::libc::c_int>)
     -> ::libc::c_int;
    pub fn heapsort_b(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                      arg4: ::libc::c_void) -> ::libc::c_int;
    pub fn mergesort(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                     arg4:
                         ::std::option::Option<extern "C" fn
                                                   (arg1:
                                                        *const ::libc::c_void,
                                                    arg2:
                                                        *const ::libc::c_void)
                                                   -> ::libc::c_int>)
     -> ::libc::c_int;
    pub fn mergesort_b(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                       arg4: ::libc::c_void) -> ::libc::c_int;
    pub fn psort(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                 arg4:
                     ::std::option::Option<extern "C" fn
                                               (arg1: *const ::libc::c_void,
                                                arg2: *const ::libc::c_void)
                                               -> ::libc::c_int>) -> ();
    pub fn psort_b(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                   arg4: ::libc::c_void) -> ();
    pub fn psort_r(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                   arg4: *mut ::libc::c_void,
                   arg5:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *mut ::libc::c_void,
                                                  arg2: *const ::libc::c_void,
                                                  arg3: *const ::libc::c_void)
                                                 -> ::libc::c_int>) -> ();
    pub fn qsort_b(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                   arg4: ::libc::c_void) -> ();
    pub fn qsort_r(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                   arg4: *mut ::libc::c_void,
                   arg5:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *mut ::libc::c_void,
                                                  arg2: *const ::libc::c_void,
                                                  arg3: *const ::libc::c_void)
                                                 -> ::libc::c_int>) -> ();
    pub fn radixsort(arg1: *mut *const ::libc::c_uchar, arg2: ::libc::c_int,
                     arg3: *const ::libc::c_uchar, arg4: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn setprogname(arg1: *const ::libc::c_char) -> ();
    pub fn sradixsort(arg1: *mut *const ::libc::c_uchar, arg2: ::libc::c_int,
                      arg3: *const ::libc::c_uchar, arg4: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn sranddev() -> ();
    pub fn srandomdev() -> ();
    pub fn reallocf(arg1: *mut ::libc::c_void, arg2: size_t)
     -> *mut ::libc::c_void;
    pub fn strtoq(arg1: *const ::libc::c_char, arg2: *mut *mut ::libc::c_char,
                  arg3: ::libc::c_int) -> ::libc::c_longlong;
    pub fn strtouq(arg1: *const ::libc::c_char,
                   arg2: *mut *mut ::libc::c_char, arg3: ::libc::c_int)
     -> ::libc::c_ulonglong;
    pub fn valloc(arg1: size_t) -> *mut ::libc::c_void;
    pub fn renameat(arg1: ::libc::c_int, arg2: *const ::libc::c_char,
                    arg3: ::libc::c_int, arg4: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn clearerr(arg1: *mut FILE) -> ();
    pub fn fclose(arg1: *mut FILE) -> ::libc::c_int;
    pub fn feof(arg1: *mut FILE) -> ::libc::c_int;
    pub fn ferror(arg1: *mut FILE) -> ::libc::c_int;
    pub fn fflush(arg1: *mut FILE) -> ::libc::c_int;
    pub fn fgetc(arg1: *mut FILE) -> ::libc::c_int;
    pub fn fgetpos(arg1: *mut FILE, arg2: *mut fpos_t) -> ::libc::c_int;
    pub fn fgets(arg1: *mut ::libc::c_char, arg2: ::libc::c_int,
                 arg3: *mut FILE) -> *mut ::libc::c_char;
    pub fn fopen(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut FILE;
    pub fn fprintf(arg1: *mut FILE, arg2: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn fputc(arg1: ::libc::c_int, arg2: *mut FILE) -> ::libc::c_int;
    pub fn fputs(arg1: *const ::libc::c_char, arg2: *mut FILE)
     -> ::libc::c_int;
    pub fn fread(arg1: *mut ::libc::c_void, arg2: size_t, arg3: size_t,
                 arg4: *mut FILE) -> size_t;
    pub fn freopen(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: *mut FILE) -> *mut FILE;
    pub fn fscanf(arg1: *mut FILE, arg2: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn fseek(arg1: *mut FILE, arg2: ::libc::c_long, arg3: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fsetpos(arg1: *mut FILE, arg2: *const fpos_t) -> ::libc::c_int;
    pub fn ftell(arg1: *mut FILE) -> ::libc::c_long;
    pub fn fwrite(arg1: *const ::libc::c_void, arg2: size_t, arg3: size_t,
                  arg4: *mut FILE) -> size_t;
    pub fn getc(arg1: *mut FILE) -> ::libc::c_int;
    pub fn getchar() -> ::libc::c_int;
    pub fn gets(arg1: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn perror(arg1: *const ::libc::c_char) -> ();
    pub fn printf(arg1: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn putc(arg1: ::libc::c_int, arg2: *mut FILE) -> ::libc::c_int;
    pub fn putchar(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn puts(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn remove(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn rename(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn rewind(arg1: *mut FILE) -> ();
    pub fn scanf(arg1: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn setbuf(arg1: *mut FILE, arg2: *mut ::libc::c_char) -> ();
    pub fn setvbuf(arg1: *mut FILE, arg2: *mut ::libc::c_char,
                   arg3: ::libc::c_int, arg4: size_t) -> ::libc::c_int;
    pub fn sprintf(arg1: *mut ::libc::c_char,
                   arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn sscanf(arg1: *const ::libc::c_char,
                  arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(arg1: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn ungetc(arg1: ::libc::c_int, arg2: *mut FILE) -> ::libc::c_int;
    pub fn vfprintf(arg1: *mut FILE, arg2: *const ::libc::c_char,
                    arg3: va_list) -> ::libc::c_int;
    pub fn vprintf(arg1: *const ::libc::c_char, arg2: va_list)
     -> ::libc::c_int;
    pub fn vsprintf(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                    arg3: va_list) -> ::libc::c_int;
    pub fn ctermid(arg1: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn fdopen(arg1: ::libc::c_int, arg2: *const ::libc::c_char)
     -> *mut FILE;
    pub fn fileno(arg1: *mut FILE) -> ::libc::c_int;
    pub fn pclose(arg1: *mut FILE) -> ::libc::c_int;
    pub fn popen(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut FILE;
    pub fn __srget(arg1: *mut FILE) -> ::libc::c_int;
    pub fn __svfscanf(arg1: *mut FILE, arg2: *const ::libc::c_char,
                      arg3: va_list) -> ::libc::c_int;
    pub fn __swbuf(arg1: ::libc::c_int, arg2: *mut FILE) -> ::libc::c_int;
    pub fn __sputc(_c: ::libc::c_int, _p: *mut FILE) -> ::libc::c_int;
    pub fn flockfile(arg1: *mut FILE) -> ();
    pub fn ftrylockfile(arg1: *mut FILE) -> ::libc::c_int;
    pub fn funlockfile(arg1: *mut FILE) -> ();
    pub fn getc_unlocked(arg1: *mut FILE) -> ::libc::c_int;
    pub fn getchar_unlocked() -> ::libc::c_int;
    pub fn putc_unlocked(arg1: ::libc::c_int, arg2: *mut FILE)
     -> ::libc::c_int;
    pub fn putchar_unlocked(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn getw(arg1: *mut FILE) -> ::libc::c_int;
    pub fn putw(arg1: ::libc::c_int, arg2: *mut FILE) -> ::libc::c_int;
    pub fn tempnam(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn fseeko(arg1: *mut FILE, arg2: off_t, arg3: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ftello(arg1: *mut FILE) -> off_t;
    pub fn snprintf(arg1: *mut ::libc::c_char, arg2: size_t,
                    arg3: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn vfscanf(arg1: *mut FILE, arg2: *const ::libc::c_char,
                   arg3: va_list) -> ::libc::c_int;
    pub fn vscanf(arg1: *const ::libc::c_char, arg2: va_list)
     -> ::libc::c_int;
    pub fn vsnprintf(arg1: *mut ::libc::c_char, arg2: size_t,
                     arg3: *const ::libc::c_char, arg4: va_list)
     -> ::libc::c_int;
    pub fn vsscanf(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: va_list) -> ::libc::c_int;
    pub fn dprintf(arg1: ::libc::c_int, arg2: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn vdprintf(arg1: ::libc::c_int, arg2: *const ::libc::c_char,
                    arg3: va_list) -> ::libc::c_int;
    pub fn getdelim(arg1: *mut *mut ::libc::c_char, arg2: *mut size_t,
                    arg3: ::libc::c_int, arg4: *mut FILE) -> ssize_t;
    pub fn getline(arg1: *mut *mut ::libc::c_char, arg2: *mut size_t,
                   arg3: *mut FILE) -> ssize_t;
    pub fn asprintf(arg1: *mut *mut ::libc::c_char,
                    arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn ctermid_r(arg1: *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn fgetln(arg1: *mut FILE, arg2: *mut size_t) -> *mut ::libc::c_char;
    pub fn fmtcheck(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> *const ::libc::c_char;
    pub fn fpurge(arg1: *mut FILE) -> ::libc::c_int;
    pub fn setbuffer(arg1: *mut FILE, arg2: *mut ::libc::c_char,
                     arg3: ::libc::c_int) -> ();
    pub fn setlinebuf(arg1: *mut FILE) -> ::libc::c_int;
    pub fn vasprintf(arg1: *mut *mut ::libc::c_char,
                     arg2: *const ::libc::c_char, arg3: va_list)
     -> ::libc::c_int;
    pub fn zopen(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char,
                 arg3: ::libc::c_int) -> *mut FILE;
    pub fn funopen(arg1: *const ::libc::c_void,
                   arg2:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *mut ::libc::c_void,
                                                  arg2: *mut ::libc::c_char,
                                                  arg3: ::libc::c_int)
                                                 -> ::libc::c_int>,
                   arg3:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *mut ::libc::c_void,
                                                  arg2: *const ::libc::c_char,
                                                  arg3: ::libc::c_int)
                                                 -> ::libc::c_int>,
                   arg4:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *mut ::libc::c_void,
                                                  arg2: fpos_t,
                                                  arg3: ::libc::c_int)
                                                 -> fpos_t>,
                   arg5:
                       ::std::option::Option<extern "C" fn
                                                 (arg1: *mut ::libc::c_void)
                                                 -> ::libc::c_int>)
     -> *mut FILE;
    pub fn __sprintf_chk(arg1: *mut ::libc::c_char, arg2: ::libc::c_int,
                         arg3: size_t, arg4: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn __snprintf_chk(arg1: *mut ::libc::c_char, arg2: size_t,
                          arg3: ::libc::c_int, arg4: size_t,
                          arg5: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn __vsprintf_chk(arg1: *mut ::libc::c_char, arg2: ::libc::c_int,
                          arg3: size_t, arg4: *const ::libc::c_char,
                          arg5: va_list) -> ::libc::c_int;
    pub fn __vsnprintf_chk(arg1: *mut ::libc::c_char, arg2: size_t,
                           arg3: ::libc::c_int, arg4: size_t,
                           arg5: *const ::libc::c_char, arg6: va_list)
     -> ::libc::c_int;
    pub fn memchr(arg1: *const ::libc::c_void, arg2: ::libc::c_int,
                  arg3: size_t) -> *mut ::libc::c_void;
    pub fn memcmp(arg1: *const ::libc::c_void, arg2: *const ::libc::c_void,
                  arg3: size_t) -> ::libc::c_int;
    pub fn memcpy(arg1: *mut ::libc::c_void, arg2: *const ::libc::c_void,
                  arg3: size_t) -> *mut ::libc::c_void;
    pub fn memmove(arg1: *mut ::libc::c_void, arg2: *const ::libc::c_void,
                   arg3: size_t) -> *mut ::libc::c_void;
    pub fn memset(arg1: *mut ::libc::c_void, arg2: ::libc::c_int,
                  arg3: size_t) -> *mut ::libc::c_void;
    pub fn strcat(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn strchr(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn strcmp(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn strcoll(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn strcpy(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn strcspn(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> size_t;
    pub fn strerror(arg1: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn strlen(arg1: *const ::libc::c_char) -> size_t;
    pub fn strncat(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> *mut ::libc::c_char;
    pub fn strncmp(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> ::libc::c_int;
    pub fn strncpy(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> *mut ::libc::c_char;
    pub fn strpbrk(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn strrchr(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn strspn(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> size_t;
    pub fn strstr(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn strtok(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn strxfrm(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> size_t;
    pub fn strtok_r(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                    arg3: *mut *mut ::libc::c_char) -> *mut ::libc::c_char;
    pub fn strerror_r(arg1: ::libc::c_int, arg2: *mut ::libc::c_char,
                      arg3: size_t) -> ::libc::c_int;
    pub fn strdup(arg1: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn memccpy(arg1: *mut ::libc::c_void, arg2: *const ::libc::c_void,
                   arg3: ::libc::c_int, arg4: size_t) -> *mut ::libc::c_void;
    pub fn stpcpy(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn stpncpy(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> *mut ::libc::c_char;
    pub fn strndup(arg1: *const ::libc::c_char, arg2: size_t)
     -> *mut ::libc::c_char;
    pub fn strnlen(arg1: *const ::libc::c_char, arg2: size_t) -> size_t;
    pub fn strsignal(sig: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn memset_s(arg1: *mut ::libc::c_void, arg2: rsize_t,
                    arg3: ::libc::c_int, arg4: rsize_t) -> errno_t;
    pub fn memmem(arg1: *const ::libc::c_void, arg2: size_t,
                  arg3: *const ::libc::c_void, arg4: size_t)
     -> *mut ::libc::c_void;
    pub fn memset_pattern4(arg1: *mut ::libc::c_void,
                           arg2: *const ::libc::c_void, arg3: size_t) -> ();
    pub fn memset_pattern8(arg1: *mut ::libc::c_void,
                           arg2: *const ::libc::c_void, arg3: size_t) -> ();
    pub fn memset_pattern16(arg1: *mut ::libc::c_void,
                            arg2: *const ::libc::c_void, arg3: size_t) -> ();
    pub fn strcasestr(arg1: *const ::libc::c_char,
                      arg2: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn strnstr(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> *mut ::libc::c_char;
    pub fn strlcat(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> size_t;
    pub fn strlcpy(arg1: *mut ::libc::c_char, arg2: *const ::libc::c_char,
                   arg3: size_t) -> size_t;
    pub fn strmode(arg1: ::libc::c_int, arg2: *mut ::libc::c_char) -> ();
    pub fn strsep(arg1: *mut *mut ::libc::c_char, arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn swab(arg1: *const ::libc::c_void, arg2: *mut ::libc::c_void,
                arg3: ssize_t) -> ();
    pub fn bcmp(arg1: *const ::libc::c_void, arg2: *const ::libc::c_void,
                arg3: size_t) -> ::libc::c_int;
    pub fn bcopy(arg1: *const ::libc::c_void, arg2: *mut ::libc::c_void,
                 arg3: size_t) -> ();
    pub fn bzero(arg1: *mut ::libc::c_void, arg2: size_t) -> ();
    pub fn index(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn rindex(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn ffs(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn strcasecmp(arg1: *const ::libc::c_char,
                      arg2: *const ::libc::c_char) -> ::libc::c_int;
    pub fn strncasecmp(arg1: *const ::libc::c_char,
                       arg2: *const ::libc::c_char, arg3: size_t)
     -> ::libc::c_int;
    pub fn ffsl(arg1: ::libc::c_long) -> ::libc::c_int;
    pub fn ffsll(arg1: ::libc::c_longlong) -> ::libc::c_int;
    pub fn fls(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn flsl(arg1: ::libc::c_long) -> ::libc::c_int;
    pub fn flsll(arg1: ::libc::c_longlong) -> ::libc::c_int;
    pub fn __math_errhandling() -> ::libc::c_int;
    pub fn __fpclassifyf(arg1: ::libc::c_float) -> ::libc::c_int;
    pub fn __fpclassifyd(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn __fpclassifyl(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isfinitef(__x: ::libc::c_float) -> ::libc::c_int;
    pub fn __inline_isfinited(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isfinitel(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isinff(__x: ::libc::c_float) -> ::libc::c_int;
    pub fn __inline_isinfd(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isinfl(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isnanf(__x: ::libc::c_float) -> ::libc::c_int;
    pub fn __inline_isnand(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isnanl(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isnormalf(__x: ::libc::c_float) -> ::libc::c_int;
    pub fn __inline_isnormald(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_isnormall(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_signbitf(__x: ::libc::c_float) -> ::libc::c_int;
    pub fn __inline_signbitd(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn __inline_signbitl(__x: ::libc::c_double) -> ::libc::c_int;
    pub fn acosf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn acos(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn acosl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn asinf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn asin(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn asinl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn atanf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn atan(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn atanl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn atan2f(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn atan2(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn atan2l(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn cosf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn cos(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn cosl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn sinf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn sin(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn sinl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tanf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn tan(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tanl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn acoshf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn acosh(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn acoshl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn asinhf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn asinh(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn asinhl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn atanhf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn atanh(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn atanhl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn coshf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn cosh(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn coshl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn sinhf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn sinh(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn sinhl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tanhf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn tanh(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tanhl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn expf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn exp(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn expl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn exp2f(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn exp2(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn exp2l(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn expm1f(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn expm1(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn expm1l(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn logf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn log(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn logl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log10f(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn log10(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log10l(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log2f(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn log2(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log2l(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log1pf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn log1p(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn log1pl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn logbf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn logb(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn logbl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn modff(arg1: ::libc::c_float, arg2: *mut ::libc::c_float)
     -> ::libc::c_float;
    pub fn modf(arg1: ::libc::c_double, arg2: *mut ::libc::c_double)
     -> ::libc::c_double;
    pub fn modfl(arg1: ::libc::c_double, arg2: *mut ::libc::c_double)
     -> ::libc::c_double;
    pub fn ldexpf(arg1: ::libc::c_float, arg2: ::libc::c_int)
     -> ::libc::c_float;
    pub fn ldexp(arg1: ::libc::c_double, arg2: ::libc::c_int)
     -> ::libc::c_double;
    pub fn ldexpl(arg1: ::libc::c_double, arg2: ::libc::c_int)
     -> ::libc::c_double;
    pub fn frexpf(arg1: ::libc::c_float, arg2: *mut ::libc::c_int)
     -> ::libc::c_float;
    pub fn frexp(arg1: ::libc::c_double, arg2: *mut ::libc::c_int)
     -> ::libc::c_double;
    pub fn frexpl(arg1: ::libc::c_double, arg2: *mut ::libc::c_int)
     -> ::libc::c_double;
    pub fn ilogbf(arg1: ::libc::c_float) -> ::libc::c_int;
    pub fn ilogb(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn ilogbl(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn scalbnf(arg1: ::libc::c_float, arg2: ::libc::c_int)
     -> ::libc::c_float;
    pub fn scalbn(arg1: ::libc::c_double, arg2: ::libc::c_int)
     -> ::libc::c_double;
    pub fn scalbnl(arg1: ::libc::c_double, arg2: ::libc::c_int)
     -> ::libc::c_double;
    pub fn scalblnf(arg1: ::libc::c_float, arg2: ::libc::c_long)
     -> ::libc::c_float;
    pub fn scalbln(arg1: ::libc::c_double, arg2: ::libc::c_long)
     -> ::libc::c_double;
    pub fn scalblnl(arg1: ::libc::c_double, arg2: ::libc::c_long)
     -> ::libc::c_double;
    pub fn fabsf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn fabs(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn fabsl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn cbrtf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn cbrt(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn cbrtl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn hypotf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn hypot(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn hypotl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn powf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn pow(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn powl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn sqrtf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn sqrt(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn sqrtl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn erff(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn erf(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn erfl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn erfcf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn erfc(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn erfcl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn lgammaf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn lgamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn lgammal(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tgammaf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn tgamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn tgammal(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn ceilf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn ceil(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn ceill(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn floorf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn floor(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn floorl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn nearbyintf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn nearbyint(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn nearbyintl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn rintf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn rint(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn rintl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn lrintf(arg1: ::libc::c_float) -> ::libc::c_long;
    pub fn lrint(arg1: ::libc::c_double) -> ::libc::c_long;
    pub fn lrintl(arg1: ::libc::c_double) -> ::libc::c_long;
    pub fn roundf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn round(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn roundl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn lroundf(arg1: ::libc::c_float) -> ::libc::c_long;
    pub fn lround(arg1: ::libc::c_double) -> ::libc::c_long;
    pub fn lroundl(arg1: ::libc::c_double) -> ::libc::c_long;
    pub fn llrintf(arg1: ::libc::c_float) -> ::libc::c_longlong;
    pub fn llrint(arg1: ::libc::c_double) -> ::libc::c_longlong;
    pub fn llrintl(arg1: ::libc::c_double) -> ::libc::c_longlong;
    pub fn llroundf(arg1: ::libc::c_float) -> ::libc::c_longlong;
    pub fn llround(arg1: ::libc::c_double) -> ::libc::c_longlong;
    pub fn llroundl(arg1: ::libc::c_double) -> ::libc::c_longlong;
    pub fn truncf(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn trunc(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn truncl(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn fmodf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn fmod(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fmodl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn remainderf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn remainder(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn remainderl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn remquof(arg1: ::libc::c_float, arg2: ::libc::c_float,
                   arg3: *mut ::libc::c_int) -> ::libc::c_float;
    pub fn remquo(arg1: ::libc::c_double, arg2: ::libc::c_double,
                  arg3: *mut ::libc::c_int) -> ::libc::c_double;
    pub fn remquol(arg1: ::libc::c_double, arg2: ::libc::c_double,
                   arg3: *mut ::libc::c_int) -> ::libc::c_double;
    pub fn copysignf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn copysign(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn copysignl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn nanf(arg1: *const ::libc::c_char) -> ::libc::c_float;
    pub fn nan(arg1: *const ::libc::c_char) -> ::libc::c_double;
    pub fn nanl(arg1: *const ::libc::c_char) -> ::libc::c_double;
    pub fn nextafterf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn nextafter(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn nextafterl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn nexttoward(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn nexttowardf(arg1: ::libc::c_float, arg2: ::libc::c_double)
     -> ::libc::c_float;
    pub fn nexttowardl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fdimf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn fdim(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fdiml(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fmaxf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn fmax(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fmaxl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fminf(arg1: ::libc::c_float, arg2: ::libc::c_float)
     -> ::libc::c_float;
    pub fn fmin(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fminl(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn fmaf(arg1: ::libc::c_float, arg2: ::libc::c_float,
                arg3: ::libc::c_float) -> ::libc::c_float;
    pub fn fma(arg1: ::libc::c_double, arg2: ::libc::c_double,
               arg3: ::libc::c_double) -> ::libc::c_double;
    pub fn fmal(arg1: ::libc::c_double, arg2: ::libc::c_double,
                arg3: ::libc::c_double) -> ::libc::c_double;
    pub fn __inff() -> ::libc::c_float;
    pub fn __inf() -> ::libc::c_double;
    pub fn __infl() -> ::libc::c_double;
    pub fn __nan() -> ::libc::c_float;
    pub fn __exp10f(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn __exp10(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn __sincosf(__x: ::libc::c_float, __sinp: *mut ::libc::c_float,
                     __cosp: *mut ::libc::c_float) -> ();
    pub fn __sincos(__x: ::libc::c_double, __sinp: *mut ::libc::c_double,
                    __cosp: *mut ::libc::c_double) -> ();
    pub fn __cospif(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn __cospi(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn __sinpif(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn __sinpi(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn __tanpif(arg1: ::libc::c_float) -> ::libc::c_float;
    pub fn __tanpi(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn __sincospif(__x: ::libc::c_float, __sinp: *mut ::libc::c_float,
                       __cosp: *mut ::libc::c_float) -> ();
    pub fn __sincospi(__x: ::libc::c_double, __sinp: *mut ::libc::c_double,
                      __cosp: *mut ::libc::c_double) -> ();
    pub fn __sincosf_stret(arg1: ::libc::c_float) -> Struct___float2;
    pub fn __sincos_stret(arg1: ::libc::c_double) -> Struct___double2;
    pub fn __sincospif_stret(arg1: ::libc::c_float) -> Struct___float2;
    pub fn __sincospi_stret(arg1: ::libc::c_double) -> Struct___double2;
    pub fn j0(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn j1(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn jn(arg1: ::libc::c_int, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn y0(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn y1(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn yn(arg1: ::libc::c_int, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn scalb(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn rinttol(arg1: ::libc::c_double) -> ::libc::c_long;
    pub fn roundtol(arg1: ::libc::c_double) -> ::libc::c_long;
    pub fn drem(arg1: ::libc::c_double, arg2: ::libc::c_double)
     -> ::libc::c_double;
    pub fn finite(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn gamma(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn significand(arg1: ::libc::c_double) -> ::libc::c_double;
    pub fn matherr(arg1: *mut Struct_exception) -> ::libc::c_int;
    pub fn __error() -> *mut ::libc::c_int;
    pub fn mpc_err_delete(e: *mut mpc_err_t) -> ();
    pub fn mpc_err_string(e: *mut mpc_err_t) -> *mut ::libc::c_char;
    pub fn mpc_err_print(e: *mut mpc_err_t) -> ();
    pub fn mpc_err_print_to(e: *mut mpc_err_t, f: *mut FILE) -> ();
    pub fn mpc_parse(filename: *const ::libc::c_char,
                     string: *const ::libc::c_char, p: *mut mpc_parser_t,
                     r: *mut mpc_result_t) -> ::libc::c_int;
    pub fn mpc_parse_file(filename: *const ::libc::c_char, file: *mut FILE,
                          p: *mut mpc_parser_t, r: *mut mpc_result_t)
     -> ::libc::c_int;
    pub fn mpc_parse_pipe(filename: *const ::libc::c_char, pipe: *mut FILE,
                          p: *mut mpc_parser_t, r: *mut mpc_result_t)
     -> ::libc::c_int;
    pub fn mpc_parse_contents(filename: *const ::libc::c_char,
                              p: *mut mpc_parser_t, r: *mut mpc_result_t)
     -> ::libc::c_int;
    pub fn mpc_new(name: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_define(p: *mut mpc_parser_t, a: *mut mpc_parser_t)
     -> *mut mpc_parser_t;
    pub fn mpc_undefine(p: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_delete(p: *mut mpc_parser_t) -> ();
    pub fn mpc_cleanup(n: ::libc::c_int, ...) -> ();
    pub fn mpc_any() -> *mut mpc_parser_t;
    pub fn mpc_char(c: ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_range(s: ::libc::c_char, e: ::libc::c_char)
     -> *mut mpc_parser_t;
    pub fn mpc_oneof(s: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_noneof(s: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_satisfy(f:
                           ::std::option::Option<extern "C" fn
                                                     (arg1: ::libc::c_char)
                                                     -> ::libc::c_int>)
     -> *mut mpc_parser_t;
    pub fn mpc_string(s: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_pass() -> *mut mpc_parser_t;
    pub fn mpc_fail(m: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_failf(fmt: *const ::libc::c_char, ...) -> *mut mpc_parser_t;
    pub fn mpc_lift(f: mpc_ctor_t) -> *mut mpc_parser_t;
    pub fn mpc_lift_val(x: *mut mpc_val_t) -> *mut mpc_parser_t;
    pub fn mpc_anchor(f:
                          ::std::option::Option<extern "C" fn
                                                    (arg1: ::libc::c_char,
                                                     arg2: ::libc::c_char)
                                                    -> ::libc::c_int>)
     -> *mut mpc_parser_t;
    pub fn mpc_state() -> *mut mpc_parser_t;
    pub fn mpc_expect(a: *mut mpc_parser_t, e: *const ::libc::c_char)
     -> *mut mpc_parser_t;
    pub fn mpc_expectf(a: *mut mpc_parser_t, fmt: *const ::libc::c_char, ...)
     -> *mut mpc_parser_t;
    pub fn mpc_apply(a: *mut mpc_parser_t, f: mpc_apply_t)
     -> *mut mpc_parser_t;
    pub fn mpc_apply_to(a: *mut mpc_parser_t, f: mpc_apply_to_t,
                        x: *mut ::libc::c_void) -> *mut mpc_parser_t;
    pub fn mpc_not(a: *mut mpc_parser_t, da: mpc_dtor_t) -> *mut mpc_parser_t;
    pub fn mpc_not_lift(a: *mut mpc_parser_t, da: mpc_dtor_t, lf: mpc_ctor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_maybe(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_maybe_lift(a: *mut mpc_parser_t, lf: mpc_ctor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_many(f: mpc_fold_t, a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_many1(f: mpc_fold_t, a: *mut mpc_parser_t)
     -> *mut mpc_parser_t;
    pub fn mpc_count(n: ::libc::c_int, f: mpc_fold_t, a: *mut mpc_parser_t,
                     da: mpc_dtor_t) -> *mut mpc_parser_t;
    pub fn mpc_or(n: ::libc::c_int, ...) -> *mut mpc_parser_t;
    pub fn mpc_and(n: ::libc::c_int, f: mpc_fold_t, ...) -> *mut mpc_parser_t;
    pub fn mpc_predictive(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_eoi() -> *mut mpc_parser_t;
    pub fn mpc_soi() -> *mut mpc_parser_t;
    pub fn mpc_boundary() -> *mut mpc_parser_t;
    pub fn mpc_whitespace() -> *mut mpc_parser_t;
    pub fn mpc_whitespaces() -> *mut mpc_parser_t;
    pub fn mpc_blank() -> *mut mpc_parser_t;
    pub fn mpc_newline() -> *mut mpc_parser_t;
    pub fn mpc_tab() -> *mut mpc_parser_t;
    pub fn mpc_escape() -> *mut mpc_parser_t;
    pub fn mpc_digit() -> *mut mpc_parser_t;
    pub fn mpc_hexdigit() -> *mut mpc_parser_t;
    pub fn mpc_octdigit() -> *mut mpc_parser_t;
    pub fn mpc_digits() -> *mut mpc_parser_t;
    pub fn mpc_hexdigits() -> *mut mpc_parser_t;
    pub fn mpc_octdigits() -> *mut mpc_parser_t;
    pub fn mpc_lower() -> *mut mpc_parser_t;
    pub fn mpc_upper() -> *mut mpc_parser_t;
    pub fn mpc_alpha() -> *mut mpc_parser_t;
    pub fn mpc_underscore() -> *mut mpc_parser_t;
    pub fn mpc_alphanum() -> *mut mpc_parser_t;
    pub fn mpc_int() -> *mut mpc_parser_t;
    pub fn mpc_hex() -> *mut mpc_parser_t;
    pub fn mpc_oct() -> *mut mpc_parser_t;
    pub fn mpc_number() -> *mut mpc_parser_t;
    pub fn mpc_real() -> *mut mpc_parser_t;
    pub fn mpc_float() -> *mut mpc_parser_t;
    pub fn mpc_char_lit() -> *mut mpc_parser_t;
    pub fn mpc_string_lit() -> *mut mpc_parser_t;
    pub fn mpc_regex_lit() -> *mut mpc_parser_t;
    pub fn mpc_ident() -> *mut mpc_parser_t;
    pub fn mpc_startwith(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_endwith(a: *mut mpc_parser_t, da: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_whole(a: *mut mpc_parser_t, da: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_stripl(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_stripr(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_strip(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_tok(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpc_sym(s: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_total(a: *mut mpc_parser_t, da: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_between(a: *mut mpc_parser_t, ad: mpc_dtor_t,
                       o: *const ::libc::c_char, c: *const ::libc::c_char)
     -> *mut mpc_parser_t;
    pub fn mpc_parens(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_braces(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_brackets(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_squares(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_tok_between(a: *mut mpc_parser_t, ad: mpc_dtor_t,
                           o: *const ::libc::c_char, c: *const ::libc::c_char)
     -> *mut mpc_parser_t;
    pub fn mpc_tok_parens(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_tok_braces(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_tok_brackets(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpc_tok_squares(a: *mut mpc_parser_t, ad: mpc_dtor_t)
     -> *mut mpc_parser_t;
    pub fn mpcf_dtor_null(x: *mut mpc_val_t) -> ();
    pub fn mpcf_ctor_null() -> *mut mpc_val_t;
    pub fn mpcf_ctor_str() -> *mut mpc_val_t;
    pub fn mpcf_free(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_int(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_hex(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_oct(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_float(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_escape(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_escape_regex(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_escape_string_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_escape_char_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_unescape(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_unescape_regex(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_unescape_string_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_unescape_char_raw(x: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_null(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_fst(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_snd(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_trd(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_fst_free(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_snd_free(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_trd_free(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_strfold(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_maths(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpc_re(re: *const ::libc::c_char) -> *mut mpc_parser_t;
    pub fn mpc_ast_new(tag: *const ::libc::c_char,
                       contents: *const ::libc::c_char) -> *mut mpc_ast_t;
    pub fn mpc_ast_build(n: ::libc::c_int, tag: *const ::libc::c_char, ...)
     -> *mut mpc_ast_t;
    pub fn mpc_ast_add_root(a: *mut mpc_ast_t) -> *mut mpc_ast_t;
    pub fn mpc_ast_add_child(r: *mut mpc_ast_t, a: *mut mpc_ast_t)
     -> *mut mpc_ast_t;
    pub fn mpc_ast_add_tag(a: *mut mpc_ast_t, t: *const ::libc::c_char)
     -> *mut mpc_ast_t;
    pub fn mpc_ast_tag(a: *mut mpc_ast_t, t: *const ::libc::c_char)
     -> *mut mpc_ast_t;
    pub fn mpc_ast_state(a: *mut mpc_ast_t, s: mpc_state_t) -> *mut mpc_ast_t;
    pub fn mpc_ast_delete(a: *mut mpc_ast_t) -> ();
    pub fn mpc_ast_print(a: *mut mpc_ast_t) -> ();
    pub fn mpc_ast_print_to(a: *mut mpc_ast_t, fp: *mut FILE) -> ();
    pub fn mpc_ast_eq(a: *mut mpc_ast_t, b: *mut mpc_ast_t) -> ::libc::c_int;
    pub fn mpcf_fold_ast(n: ::libc::c_int, _as: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpcf_str_ast(c: *mut mpc_val_t) -> *mut mpc_val_t;
    pub fn mpcf_state_ast(n: ::libc::c_int, xs: *mut *mut mpc_val_t)
     -> *mut mpc_val_t;
    pub fn mpca_tag(a: *mut mpc_parser_t, t: *const ::libc::c_char)
     -> *mut mpc_parser_t;
    pub fn mpca_add_tag(a: *mut mpc_parser_t, t: *const ::libc::c_char)
     -> *mut mpc_parser_t;
    pub fn mpca_root(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_state(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_total(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_not(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_maybe(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_many(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_many1(a: *mut mpc_parser_t) -> *mut mpc_parser_t;
    pub fn mpca_count(n: ::libc::c_int, a: *mut mpc_parser_t)
     -> *mut mpc_parser_t;
    pub fn mpca_or(n: ::libc::c_int, ...) -> *mut mpc_parser_t;
    pub fn mpca_and(n: ::libc::c_int, ...) -> *mut mpc_parser_t;
    pub fn mpca_grammar(flags: ::libc::c_int,
                        grammar: *const ::libc::c_char, ...)
     -> *mut mpc_parser_t;
    pub fn mpca_lang(flags: ::libc::c_int,
                     language: *const ::libc::c_char, ...) -> *mut mpc_err_t;
    pub fn mpca_lang_file(flags: ::libc::c_int, f: *mut FILE, ...)
     -> *mut mpc_err_t;
    pub fn mpca_lang_pipe(flags: ::libc::c_int, f: *mut FILE, ...)
     -> *mut mpc_err_t;
    pub fn mpca_lang_contents(flags: ::libc::c_int,
                              filename: *const ::libc::c_char, ...)
     -> *mut mpc_err_t;
    pub fn mpc_print(p: *mut mpc_parser_t) -> ();
    pub fn mpc_test_pass(p: *mut mpc_parser_t, s: *const ::libc::c_char,
                         d: *const ::libc::c_void,
                         tester:
                             ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *const ::libc::c_void,
                                                        arg2:
                                                            *const ::libc::c_void)
                                                       -> ::libc::c_int>,
                         destructor: mpc_dtor_t,
                         printer:
                             ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *const ::libc::c_void)
                                                       -> ()>)
     -> ::libc::c_int;
    pub fn mpc_test_fail(p: *mut mpc_parser_t, s: *const ::libc::c_char,
                         d: *const ::libc::c_void,
                         tester:
                             ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *const ::libc::c_void,
                                                        arg2:
                                                            *const ::libc::c_void)
                                                       -> ::libc::c_int>,
                         destructor: mpc_dtor_t,
                         printer:
                             ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *const ::libc::c_void)
                                                       -> ()>)
     -> ::libc::c_int;
}
