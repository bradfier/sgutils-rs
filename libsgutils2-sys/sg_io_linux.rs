/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_least8_t = ::std::os::raw::c_char;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_char;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type size_t = usize;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_char;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
impl ::std::default::Default for __fsid_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type FILE = _IO_FILE;
pub type __FILE = _IO_FILE;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: Union_Unnamed1,
}
impl ::std::default::Default for __mbstate_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed1 {
    pub unsafe fn __wch(&mut self) -> *mut ::std::os::raw::c_uint {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __wchb(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
impl ::std::default::Default for _G_fpos_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
impl ::std::default::Default for _G_fpos64_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type va_list = __gnuc_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub enum _IO_jump_t { }
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: ::std::os::raw::c_int,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for _IO_marker {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum __codecvt_result {
    __codecvt_ok = 0,
    __codecvt_partial = 1,
    __codecvt_error = 2,
    __codecvt_noconv = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_char,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::std::os::raw::c_void,
    pub __pad2: *mut ::std::os::raw::c_void,
    pub __pad3: *mut ::std::os::raw::c_void,
    pub __pad4: *mut ::std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
impl ::std::default::Default for _IO_FILE {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum _IO_FILE_plus { }
pub type __io_read_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void,
                                               __buf:
                                                   *mut ::std::os::raw::c_char,
                                               __nbytes: size_t)
                              -> __ssize_t>;
pub type __io_write_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void,
                                               __buf:
                                                   *const ::std::os::raw::c_char,
                                               __n: size_t) -> __ssize_t>;
pub type __io_seek_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void,
                                               __pos: *mut __off64_t,
                                               __w: ::std::os::raw::c_int)
                              -> ::std::os::raw::c_int>;
pub type __io_close_fn =
    ::std::option::Option<unsafe extern "C" fn(__cookie:
                                                   *mut ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sg_scsi_sense_hdr {
    pub response_code: ::std::os::raw::c_uchar,
    pub sense_key: ::std::os::raw::c_uchar,
    pub asc: ::std::os::raw::c_uchar,
    pub ascq: ::std::os::raw::c_uchar,
    pub byte4: ::std::os::raw::c_uchar,
    pub byte5: ::std::os::raw::c_uchar,
    pub byte6: ::std::os::raw::c_uchar,
    pub additional_length: ::std::os::raw::c_uchar,
}
impl ::std::default::Default for sg_scsi_sense_hdr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sg_iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: size_t,
}
impl ::std::default::Default for sg_iovec {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sg_iovec_t = sg_iovec;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sg_io_hdr {
    pub interface_id: ::std::os::raw::c_int,
    pub dxfer_direction: ::std::os::raw::c_int,
    pub cmd_len: ::std::os::raw::c_uchar,
    pub mx_sb_len: ::std::os::raw::c_uchar,
    pub iovec_count: ::std::os::raw::c_ushort,
    pub dxfer_len: ::std::os::raw::c_uint,
    pub dxferp: *mut ::std::os::raw::c_void,
    pub cmdp: *mut ::std::os::raw::c_uchar,
    pub sbp: *mut ::std::os::raw::c_uchar,
    pub timeout: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub pack_id: ::std::os::raw::c_int,
    pub usr_ptr: *mut ::std::os::raw::c_void,
    pub status: ::std::os::raw::c_uchar,
    pub masked_status: ::std::os::raw::c_uchar,
    pub msg_status: ::std::os::raw::c_uchar,
    pub sb_len_wr: ::std::os::raw::c_uchar,
    pub host_status: ::std::os::raw::c_ushort,
    pub driver_status: ::std::os::raw::c_ushort,
    pub resid: ::std::os::raw::c_int,
    pub duration: ::std::os::raw::c_uint,
    pub info: ::std::os::raw::c_uint,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for sg_io_hdr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sg_io_hdr_t = sg_io_hdr;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sg_scsi_id {
    pub host_no: ::std::os::raw::c_int,
    pub channel: ::std::os::raw::c_int,
    pub scsi_id: ::std::os::raw::c_int,
    pub lun: ::std::os::raw::c_int,
    pub scsi_type: ::std::os::raw::c_int,
    pub h_cmd_per_lun: ::std::os::raw::c_short,
    pub d_queue_depth: ::std::os::raw::c_short,
    pub unused: [::std::os::raw::c_int; 2usize],
}
impl ::std::default::Default for sg_scsi_id {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sg_req_info {
    pub req_state: ::std::os::raw::c_char,
    pub orphan: ::std::os::raw::c_char,
    pub sg_io_owned: ::std::os::raw::c_char,
    pub problem: ::std::os::raw::c_char,
    pub pack_id: ::std::os::raw::c_int,
    pub usr_ptr: *mut ::std::os::raw::c_void,
    pub duration: ::std::os::raw::c_uint,
    pub unused: ::std::os::raw::c_int,
}
impl ::std::default::Default for sg_req_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sg_req_info_t = sg_req_info;
pub type Sg_io_hdr = sg_io_hdr;
pub enum sg_io_vec { }
pub type Sg_io_vec = sg_io_vec;
pub type Sg_scsi_id = sg_scsi_id;
pub type Sg_req_info = sg_req_info;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct sg_header {
    pub pack_len: ::std::os::raw::c_int,
    pub reply_len: ::std::os::raw::c_int,
    pub pack_id: ::std::os::raw::c_int,
    pub result: ::std::os::raw::c_int,
    pub _bindgen_bitfield_1_: ::std::os::raw::c_uint,
    pub sense_buffer: [::std::os::raw::c_uchar; 16usize],
}
impl ::std::default::Default for sg_header {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ccs_modesel_head {
    pub _r1: ::std::os::raw::c_uchar,
    pub medium: ::std::os::raw::c_uchar,
    pub _r2: ::std::os::raw::c_uchar,
    pub block_desc_length: ::std::os::raw::c_uchar,
    pub density: ::std::os::raw::c_uchar,
    pub number_blocks_hi: ::std::os::raw::c_uchar,
    pub number_blocks_med: ::std::os::raw::c_uchar,
    pub number_blocks_lo: ::std::os::raw::c_uchar,
    pub _r3: ::std::os::raw::c_uchar,
    pub block_length_hi: ::std::os::raw::c_uchar,
    pub block_length_med: ::std::os::raw::c_uchar,
    pub block_length_lo: ::std::os::raw::c_uchar,
}
impl ::std::default::Default for ccs_modesel_head {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
impl ::std::default::Default for __va_list_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut _IO_2_1_stdin_: _IO_FILE_plus;
    pub static mut _IO_2_1_stdout_: _IO_FILE_plus;
    pub static mut _IO_2_1_stderr_: _IO_FILE_plus;
    pub static mut stdin: *mut _IO_FILE;
    pub static mut stdout: *mut _IO_FILE;
    pub static mut stderr: *mut _IO_FILE;
    pub static mut sys_nerr: ::std::os::raw::c_int;
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
    pub static mut sg_warnings_strm: *mut FILE;
}
extern "C" {
    pub fn __underflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn __uflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn __overflow(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn _IO_getc(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn _IO_putc(__c: ::std::os::raw::c_int, __fp: *mut _IO_FILE)
     -> ::std::os::raw::c_int;
    pub fn _IO_feof(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn _IO_ferror(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn _IO_peekc_locked(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn _IO_flockfile(arg1: *mut _IO_FILE);
    pub fn _IO_funlockfile(arg1: *mut _IO_FILE);
    pub fn _IO_ftrylockfile(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
    pub fn _IO_vfscanf(arg1: *mut _IO_FILE,
                       arg2: *const ::std::os::raw::c_char,
                       arg3: __gnuc_va_list, arg4: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn _IO_vfprintf(arg1: *mut _IO_FILE,
                        arg2: *const ::std::os::raw::c_char,
                        arg3: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn _IO_padn(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int,
                    arg3: __ssize_t) -> __ssize_t;
    pub fn _IO_sgetn(arg1: *mut _IO_FILE, arg2: *mut ::std::os::raw::c_void,
                     arg3: size_t) -> size_t;
    pub fn _IO_seekoff(arg1: *mut _IO_FILE, arg2: __off64_t,
                       arg3: ::std::os::raw::c_int,
                       arg4: ::std::os::raw::c_int) -> __off64_t;
    pub fn _IO_seekpos(arg1: *mut _IO_FILE, arg2: __off64_t,
                       arg3: ::std::os::raw::c_int) -> __off64_t;
    pub fn _IO_free_backup_area(arg1: *mut _IO_FILE);
    pub fn remove(__filename: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rename(__old: *const ::std::os::raw::c_char,
                  __new: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn renameat(__oldfd: ::std::os::raw::c_int,
                    __old: *const ::std::os::raw::c_char,
                    __newfd: ::std::os::raw::c_int,
                    __new: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn tempnam(__dir: *const ::std::os::raw::c_char,
                   __pfx: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fopen(__filename: *const ::std::os::raw::c_char,
                 __modes: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn freopen(__filename: *const ::std::os::raw::c_char,
                   __modes: *const ::std::os::raw::c_char,
                   __stream: *mut FILE) -> *mut FILE;
    pub fn fdopen(__fd: ::std::os::raw::c_int,
                  __modes: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn fmemopen(__s: *mut ::std::os::raw::c_void, __len: size_t,
                    __modes: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn open_memstream(__bufloc: *mut *mut ::std::os::raw::c_char,
                          __sizeloc: *mut size_t) -> *mut FILE;
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char,
                   __modes: ::std::os::raw::c_int, __n: size_t)
     -> ::std::os::raw::c_int;
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char,
                     __size: size_t);
    pub fn setlinebuf(__stream: *mut FILE);
    pub fn fprintf(__stream: *mut FILE,
                   __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn printf(__format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn sprintf(__s: *mut ::std::os::raw::c_char,
                   __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn vfprintf(__s: *mut FILE, __format: *const ::std::os::raw::c_char,
                    __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn vprintf(__format: *const ::std::os::raw::c_char,
                   __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn vsprintf(__s: *mut ::std::os::raw::c_char,
                    __format: *const ::std::os::raw::c_char,
                    __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn snprintf(__s: *mut ::std::os::raw::c_char, __maxlen: size_t,
                    __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn vsnprintf(__s: *mut ::std::os::raw::c_char, __maxlen: size_t,
                     __format: *const ::std::os::raw::c_char,
                     __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn vdprintf(__fd: ::std::os::raw::c_int,
                    __fmt: *const ::std::os::raw::c_char,
                    __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn dprintf(__fd: ::std::os::raw::c_int,
                   __fmt: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn fscanf(__stream: *mut FILE,
                  __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn sscanf(__s: *const ::std::os::raw::c_char,
                  __format: *const ::std::os::raw::c_char, ...)
     -> ::std::os::raw::c_int;
    pub fn vfscanf(__s: *mut FILE, __format: *const ::std::os::raw::c_char,
                   __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn vscanf(__format: *const ::std::os::raw::c_char,
                  __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn vsscanf(__s: *const ::std::os::raw::c_char,
                   __format: *const ::std::os::raw::c_char,
                   __arg: __gnuc_va_list) -> ::std::os::raw::c_int;
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar() -> ::std::os::raw::c_int;
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn fgets(__s: *mut ::std::os::raw::c_char, __n: ::std::os::raw::c_int,
                 __stream: *mut FILE) -> *mut ::std::os::raw::c_char;
    pub fn __getdelim(__lineptr: *mut *mut ::std::os::raw::c_char,
                      __n: *mut size_t, __delimiter: ::std::os::raw::c_int,
                      __stream: *mut FILE) -> __ssize_t;
    pub fn getdelim(__lineptr: *mut *mut ::std::os::raw::c_char,
                    __n: *mut size_t, __delimiter: ::std::os::raw::c_int,
                    __stream: *mut FILE) -> __ssize_t;
    pub fn getline(__lineptr: *mut *mut ::std::os::raw::c_char,
                   __n: *mut size_t, __stream: *mut FILE) -> __ssize_t;
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE)
     -> ::std::os::raw::c_int;
    pub fn fread(__ptr: *mut ::std::os::raw::c_void, __size: size_t,
                 __n: size_t, __stream: *mut FILE) -> size_t;
    pub fn fwrite(__ptr: *const ::std::os::raw::c_void, __size: size_t,
                  __n: size_t, __s: *mut FILE) -> size_t;
    pub fn fread_unlocked(__ptr: *mut ::std::os::raw::c_void, __size: size_t,
                          __n: size_t, __stream: *mut FILE) -> size_t;
    pub fn fwrite_unlocked(__ptr: *const ::std::os::raw::c_void,
                           __size: size_t, __n: size_t, __stream: *mut FILE)
     -> size_t;
    pub fn fseek(__stream: *mut FILE, __off: ::std::os::raw::c_long,
                 __whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
    pub fn rewind(__stream: *mut FILE);
    pub fn fseeko(__stream: *mut FILE, __off: __off_t,
                  __whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ftello(__stream: *mut FILE) -> __off_t;
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t)
     -> ::std::os::raw::c_int;
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t)
     -> ::std::os::raw::c_int;
    pub fn clearerr(__stream: *mut FILE);
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn clearerr_unlocked(__stream: *mut FILE);
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn perror(__s: *const ::std::os::raw::c_char);
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn popen(__command: *const ::std::os::raw::c_char,
                 __modes: *const ::std::os::raw::c_char) -> *mut FILE;
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ctermid(__s: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn flockfile(__stream: *mut FILE);
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn funlockfile(__stream: *mut FILE);
    pub fn sg_lib_version() -> *const ::std::os::raw::c_char;
    pub fn sg_get_command_size(cdb_byte0: ::std::os::raw::c_uchar)
     -> ::std::os::raw::c_int;
    pub fn sg_get_command_name(cdbp: *const ::std::os::raw::c_uchar,
                               peri_type: ::std::os::raw::c_int,
                               buff_len: ::std::os::raw::c_int,
                               buff: *mut ::std::os::raw::c_char);
    pub fn sg_get_opcode_name(cdb_byte0: ::std::os::raw::c_uchar,
                              peri_type: ::std::os::raw::c_int,
                              buff_len: ::std::os::raw::c_int,
                              buff: *mut ::std::os::raw::c_char);
    pub fn sg_get_opcode_sa_name(cdb_byte0: ::std::os::raw::c_uchar,
                                 service_action: ::std::os::raw::c_int,
                                 peri_type: ::std::os::raw::c_int,
                                 buff_len: ::std::os::raw::c_int,
                                 buff: *mut ::std::os::raw::c_char);
    pub fn sg_get_scsi_status_str(scsi_status: ::std::os::raw::c_int,
                                  buff_len: ::std::os::raw::c_int,
                                  buff: *mut ::std::os::raw::c_char);
    pub fn sg_scsi_normalize_sense(sensep: *const ::std::os::raw::c_uchar,
                                   sense_len: ::std::os::raw::c_int,
                                   sshp: *mut sg_scsi_sense_hdr)
     -> ::std::os::raw::c_int;
    pub fn sg_scsi_sense_desc_find(sensep: *const ::std::os::raw::c_uchar,
                                   sense_len: ::std::os::raw::c_int,
                                   desc_type: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_uchar;
    pub fn sg_get_sense_key(sensep: *const ::std::os::raw::c_uchar,
                            sense_len: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_get_sense_key_str(sense_key: ::std::os::raw::c_int,
                                buff_len: ::std::os::raw::c_int,
                                buff: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn sg_get_asc_ascq_str(asc: ::std::os::raw::c_int,
                               ascq: ::std::os::raw::c_int,
                               buff_len: ::std::os::raw::c_int,
                               buff: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn sg_get_sense_info_fld(sensep: *const ::std::os::raw::c_uchar,
                                 sb_len: ::std::os::raw::c_int,
                                 info_outp: *mut uint64_t)
     -> ::std::os::raw::c_int;
    pub fn sg_get_sense_filemark_eom_ili(sensep:
                                             *const ::std::os::raw::c_uchar,
                                         sb_len: ::std::os::raw::c_int,
                                         filemark_p:
                                             *mut ::std::os::raw::c_int,
                                         eom_p: *mut ::std::os::raw::c_int,
                                         ili_p: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_get_sense_progress_fld(sensep: *const ::std::os::raw::c_uchar,
                                     sb_len: ::std::os::raw::c_int,
                                     progress_outp:
                                         *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_get_sense_str(leadin: *const ::std::os::raw::c_char,
                            sense_buffer: *const ::std::os::raw::c_uchar,
                            sb_len: ::std::os::raw::c_int,
                            raw_sinfo: ::std::os::raw::c_int,
                            buff_len: ::std::os::raw::c_int,
                            buff: *mut ::std::os::raw::c_char);
    pub fn sg_get_pdt_str(pdt: ::std::os::raw::c_int,
                          buff_len: ::std::os::raw::c_int,
                          buff: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn sg_lib_pdt_decay(pdt: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_get_trans_proto_str(tpi: ::std::os::raw::c_int,
                                  buff_len: ::std::os::raw::c_int,
                                  buff: *mut ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn sg_set_warnings_strm(warnings_strm: *mut FILE);
    pub fn sg_print_command(command: *const ::std::os::raw::c_uchar);
    pub fn sg_print_sense(leadin: *const ::std::os::raw::c_char,
                          sense_buffer: *const ::std::os::raw::c_uchar,
                          sb_len: ::std::os::raw::c_int,
                          raw_info: ::std::os::raw::c_int);
    pub fn sg_print_scsi_status(scsi_status: ::std::os::raw::c_int);
    pub fn sg_err_category_sense(sense_buffer: *const ::std::os::raw::c_uchar,
                                 sb_len: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_get_category_sense_str(sense_cat: ::std::os::raw::c_int,
                                     buff_len: ::std::os::raw::c_int,
                                     buff: *mut ::std::os::raw::c_char,
                                     verbose: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
    pub fn sg_vpd_dev_id_iter(initial_desig_desc:
                                  *const ::std::os::raw::c_uchar,
                              page_len: ::std::os::raw::c_int,
                              off: *mut ::std::os::raw::c_int,
                              m_assoc: ::std::os::raw::c_int,
                              m_desig_type: ::std::os::raw::c_int,
                              m_code_set: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn safe_strerror(errnum: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_char;
    pub fn dStrHex(str: *const ::std::os::raw::c_char,
                   len: ::std::os::raw::c_int,
                   no_ascii: ::std::os::raw::c_int);
    pub fn dStrHexErr(str: *const ::std::os::raw::c_char,
                      len: ::std::os::raw::c_int,
                      no_ascii: ::std::os::raw::c_int);
    pub fn dStrHexStr(str: *const ::std::os::raw::c_char,
                      len: ::std::os::raw::c_int,
                      leadin: *const ::std::os::raw::c_char,
                      format: ::std::os::raw::c_int,
                      b_len: ::std::os::raw::c_int,
                      b: *mut ::std::os::raw::c_char);
    pub fn sg_is_big_endian() -> ::std::os::raw::c_int;
    pub fn sg_ata_get_chars(word_arr: *const ::std::os::raw::c_ushort,
                            start_word: ::std::os::raw::c_int,
                            num_words: ::std::os::raw::c_int,
                            is_big_endian: ::std::os::raw::c_int,
                            ochars: *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn dWordHex(words: *const ::std::os::raw::c_ushort,
                    num: ::std::os::raw::c_int,
                    no_ascii: ::std::os::raw::c_int,
                    swapb: ::std::os::raw::c_int);
    pub fn sg_get_num(buf: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn sg_get_num_nomult(buf: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn sg_get_llnum(buf: *const ::std::os::raw::c_char) -> int64_t;
    pub fn sg_set_text_mode(fd: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_set_binary_mode(fd: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_print_masked_status(masked_status: ::std::os::raw::c_int);
    pub fn sg_print_host_status(host_status: ::std::os::raw::c_int);
    pub fn sg_print_driver_status(driver_status: ::std::os::raw::c_int);
    pub fn sg_chk_n_print(leadin: *const ::std::os::raw::c_char,
                          masked_status: ::std::os::raw::c_int,
                          host_status: ::std::os::raw::c_int,
                          driver_status: ::std::os::raw::c_int,
                          sense_buffer: *const ::std::os::raw::c_uchar,
                          sb_len: ::std::os::raw::c_int,
                          raw_sinfo: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_chk_n_print3(leadin: *const ::std::os::raw::c_char,
                           hp: *mut sg_io_hdr,
                           raw_sinfo: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_normalize_sense(hp: *const sg_io_hdr,
                              sshp: *mut sg_scsi_sense_hdr)
     -> ::std::os::raw::c_int;
    pub fn sg_err_category(masked_status: ::std::os::raw::c_int,
                           host_status: ::std::os::raw::c_int,
                           driver_status: ::std::os::raw::c_int,
                           sense_buffer: *const ::std::os::raw::c_uchar,
                           sb_len: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_err_category_new(scsi_status: ::std::os::raw::c_int,
                               host_status: ::std::os::raw::c_int,
                               driver_status: ::std::os::raw::c_int,
                               sense_buffer: *const ::std::os::raw::c_uchar,
                               sb_len: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn sg_err_category3(hp: *mut sg_io_hdr) -> ::std::os::raw::c_int;
}