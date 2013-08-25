/* 
automatically generated by rust-bindgen, with some manual changes needed:
 *  * Replace Struct_stat with libc::stat
 *  * use a bindgen'd statvfs and statfs
 *  * add Struct_iovec
 *  * add stub Struct_flock
 *  * add derived Zero impl for Struct_fuse_file_info
 *  * manually change the type of the function pointers, since rust-bindgen hasn't been updated for that yet
*/

use std::libc::*;
type Struct_stat = stat;
mod statfs;
type Struct_statfs = statfs::Struct_statfs;
type Struct_flock = c_void; // This is not implemented and only used for a pointer anyway...

pub type __fsblkcnt64_t = c_ulong;
pub type __fsfilcnt64_t = c_ulong;
pub struct Struct_statvfs {
    f_bsize: c_ulong,
    f_frsize: c_ulong,
    f_blocks: __fsblkcnt64_t,
    f_bfree: __fsblkcnt64_t,
    f_bavail: __fsblkcnt64_t,
    f_files: __fsfilcnt64_t,
    f_ffree: __fsfilcnt64_t,
    f_favail: __fsfilcnt64_t,
    f_fsid: c_ulong,
    f_flag: c_ulong,
    f_namemax: c_ulong,
    __f_spare: [c_int, ..6u],
}
pub struct Struct_iovec {
    iov_base: *c_void,
    iov_len: size_t
}

pub struct Struct_fuse_opt {
    templ: *c_schar,
    offset: c_ulong,
    value: c_int,
}
pub struct Struct_fuse_args {
    argc: c_int,
    argv: *mut *mut c_schar,
    allocated: c_int,
}
pub type fuse_opt_proc_t = *u8;

#[deriving(Zero)]
pub struct Struct_fuse_file_info {
    flags: c_int,
    fh_old: c_ulong,
    writepage: c_int,
    direct_io: c_uint,
    keep_cache: c_uint,
    flush: c_uint,
    nonseekable: c_uint,
    flock_release: c_uint,
    padding: c_uint,
    fh: uint64_t,
    lock_owner: uint64_t,
}
pub struct Struct_fuse_conn_info {
    proto_major: c_uint,
    proto_minor: c_uint,
    async_read: c_uint,
    max_write: c_uint,
    max_readahead: c_uint,
    capable: c_uint,
    want: c_uint,
    max_background: c_uint,
    congestion_threshold: c_uint,
    reserved: [c_uint, ..23u],
}
pub type Struct_fuse_session = c_void;
pub type Struct_fuse_chan = c_void;
pub type Struct_fuse_pollhandle = c_void;
pub type Enum_fuse_buf_flags = c_uint;
pub static FUSE_BUF_IS_FD: c_uint = 2;
pub static FUSE_BUF_FD_SEEK: c_uint = 4;
pub static FUSE_BUF_FD_RETRY: c_uint = 8;
pub type Enum_fuse_buf_copy_flags = c_uint;
pub static FUSE_BUF_NO_SPLICE: c_uint = 2;
pub static FUSE_BUF_FORCE_SPLICE: c_uint = 4;
pub static FUSE_BUF_SPLICE_MOVE: c_uint = 8;
pub static FUSE_BUF_SPLICE_NONBLOCK: c_uint = 16;
pub struct Struct_fuse_buf {
    size: size_t,
    flags: Enum_fuse_buf_flags,
    mem: *mut c_void,
    fd: c_int,
    pos: off_t,
}
pub struct Struct_fuse_bufvec {
    count: size_t,
    idx: size_t,
    off: size_t,
    buf: [Struct_fuse_buf, ..1u],
}
pub type fuse_ino_t = c_ulong;
pub type Struct_fuse_req = c_void;
pub type fuse_req_t = *mut Struct_fuse_req;
pub struct Struct_fuse_entry_param {
    ino: fuse_ino_t,
    generation: c_ulong,
    attr: Struct_stat,
    attr_timeout: c_double,
    entry_timeout: c_double,
}
pub struct Struct_fuse_ctx {
    uid: uid_t,
    gid: gid_t,
    pid: pid_t,
    umask: mode_t,
}
pub struct Struct_fuse_forget_data {
    ino: uint64_t,
    nlookup: uint64_t,
}
pub struct Struct_fuse_lowlevel_ops {
    init: Option<extern "C" fn(userdata:*mut c_void, 
                               _conn:*Struct_fuse_conn_info)>,
    destroy: Option<extern "C" fn(userdata:*mut c_void)>,
    lookup: Option<extern "C" fn(req:fuse_req_t,  parent:fuse_ino_t, 
                                 name:*c_schar)>,
    forget: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                 nlookup:c_ulong)>,
    getattr: Option<extern "C" fn(req:fuse_req_t, ino: fuse_ino_t,
                                  _fi:*Struct_fuse_file_info)>,
    setattr: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, attr:*stat,
                                  to_set: int, fi: *Struct_fuse_file_info)>,
    readlink: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t)>,
    mknod: Option<extern "C" fn(req:fuse_req_t, parent: fuse_ino_t, 
                                name:*c_schar, mode: mode_t, rdev: dev_t)>,
    mkdir: Option<extern "C" fn(req: fuse_req_t, parent: fuse_ino_t, 
                                name:*c_schar, mode:mode_t)>,
    unlink: Option<extern "C" fn(req: fuse_req_t, parent: fuse_ino_t, 
                                 name:*c_schar)>,
    rmdir: Option<extern "C" fn(req: fuse_req_t, parent: fuse_ino_t, 
                                name:*c_schar)>,
    symlink: Option<extern "C" fn(req: fuse_req_t, link: *c_schar, 
                                  parent: fuse_ino_t, name: *c_schar)>,
    rename: Option<extern "C" fn(req: fuse_req_t, parent: fuse_ino_t, 
                                 name: *c_schar, newparent: fuse_ino_t,
                                 newname: *c_schar)>,
    link: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                               newparent: fuse_ino_t, newname: *c_schar)>,
    open: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t,
                               fi: *Struct_fuse_file_info)>,
    read: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, size: size_t,
                               off: off_t, fi: *Struct_fuse_file_info)>,
    write: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, buf: *u8,
                                size: size_t, off: off_t, 
                                fi: *Struct_fuse_file_info)>,
    flush: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t,
                                fi: *Struct_fuse_file_info)>,
    release: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t,
                                  fi: *Struct_fuse_file_info)>,
    fsync: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                datasync: c_int, fi: *Struct_fuse_file_info)>,
    opendir: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t,
                                  _fi: *Struct_fuse_file_info)>,
    readdir: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                  size: size_t, off: off_t, 
                                  fi: *Struct_fuse_file_info)>,
    releasedir: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t,
                                     fi: *Struct_fuse_file_info)>,
    fsyncdir: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                   datasync: c_int,
                                   fi: *Struct_fuse_file_info)>,
    statfs: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t)>,
    setxattr: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                   name: *c_schar, value: *u8, size: size_t,
                                   flags: c_int)>,
    getxattr: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                   name: *c_schar, size: size_t)>,
    listxattr: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                    size: size_t)>,
    removexattr: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t, 
                                      name: *c_schar)>,
    access: Option<extern "C" fn(req: fuse_req_t, ino: fuse_ino_t,
                                 mask: c_int)>,
    create: Option<extern "C" fn(req: fuse_req_t, parent: fuse_ino_t, 
                                 name: *c_schar,
                                 mode: mode_t, fi: *Struct_fuse_file_info)>,

    // We don't use these yet...
    getlk: *u8,
    setlk: *u8,
    bmap: *u8,
    ioctl: *u8,
    poll: *u8,
    write_buf: *u8,
    retrieve_reply: *u8,
    forget_multi: *u8,
    flock: *u8,
    fallocate: *u8,
}
pub type fuse_interrupt_func_t = *u8;
pub struct Struct_fuse_session_ops {
    process: *u8,
    exit: *u8,
    exited: *u8,
    destroy: *u8,
}
pub struct Struct_fuse_chan_ops {
    receive: *u8,
    send: *u8,
    destroy: *u8,
}
#[link_args = "-lfuse"]
extern "C" {
    pub fn fuse_opt_parse(args: *mut Struct_fuse_args, data: *mut c_void,
                          opts: *Struct_fuse_opt, proc: fuse_opt_proc_t) ->
     c_int;
    pub fn fuse_opt_add_opt(opts: *mut *mut c_schar, opt: *c_schar) -> c_int;
    pub fn fuse_opt_add_opt_escaped(opts: *mut *mut c_schar, opt: *c_schar) ->
     c_int;
    pub fn fuse_opt_add_arg(args: *mut Struct_fuse_args, arg: *c_schar) ->
     c_int;
    pub fn fuse_opt_insert_arg(args: *mut Struct_fuse_args, pos: c_int,
                               arg: *c_schar) -> c_int;
    pub fn fuse_opt_free_args(args: *mut Struct_fuse_args);
    pub fn fuse_opt_match(opts: *Struct_fuse_opt, opt: *c_schar) -> c_int;
    pub fn fuse_mount(mountpoint: *c_schar, args: *mut Struct_fuse_args) ->
     *mut Struct_fuse_chan;
    pub fn fuse_unmount(mountpoint: *c_schar, ch: *mut Struct_fuse_chan);
    pub fn fuse_parse_cmdline(args: *mut Struct_fuse_args,
                              mountpoint: *mut *mut c_schar,
                              multithreaded: *mut c_int,
                              foreground: *mut c_int) -> c_int;
    pub fn fuse_daemonize(foreground: c_int) -> c_int;
    pub fn fuse_version() -> c_int;
    pub fn fuse_pollhandle_destroy(ph: *mut Struct_fuse_pollhandle);
    pub fn fuse_buf_size(bufv: *Struct_fuse_bufvec) -> size_t;
    pub fn fuse_buf_copy(dst: *mut Struct_fuse_bufvec,
                         src: *mut Struct_fuse_bufvec,
                         flags: Enum_fuse_buf_copy_flags) -> ssize_t;
    pub fn fuse_set_signal_handlers(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_remove_signal_handlers(se: *mut Struct_fuse_session);
    pub fn fuse_reply_err(req: fuse_req_t, err: c_int) -> c_int;
    pub fn fuse_reply_none(req: fuse_req_t);
    pub fn fuse_reply_entry(req: fuse_req_t, e: *Struct_fuse_entry_param) ->
     c_int;
    pub fn fuse_reply_create(req: fuse_req_t, e: *Struct_fuse_entry_param,
                             fi: *Struct_fuse_file_info) -> c_int;
    pub fn fuse_reply_attr(req: fuse_req_t, attr: *Struct_stat,
                           attr_timeout: c_double) -> c_int;
    pub fn fuse_reply_readlink(req: fuse_req_t, link: *c_schar) -> c_int;
    pub fn fuse_reply_open(req: fuse_req_t, fi: *Struct_fuse_file_info) ->
     c_int;
    pub fn fuse_reply_write(req: fuse_req_t, count: size_t) -> c_int;
    pub fn fuse_reply_buf(req: fuse_req_t, buf: *c_schar, size: size_t) ->
     c_int;
    pub fn fuse_reply_data(req: fuse_req_t, bufv: *mut Struct_fuse_bufvec,
                           flags: Enum_fuse_buf_copy_flags) -> c_int;
    pub fn fuse_reply_iov(req: fuse_req_t, iov: *Struct_iovec, count: c_int)
     -> c_int;
    pub fn fuse_reply_statfs(req: fuse_req_t, stbuf: *Struct_statvfs) ->
     c_int;
    pub fn fuse_reply_xattr(req: fuse_req_t, count: size_t) -> c_int;
    pub fn fuse_reply_lock(req: fuse_req_t, lock: *Struct_flock) -> c_int;
    pub fn fuse_reply_bmap(req: fuse_req_t, idx: uint64_t) -> c_int;
    pub fn fuse_add_direntry(req: fuse_req_t, buf: *mut c_schar,
                             bufsize: size_t, name: *c_schar,
                             stbuf: *Struct_stat, off: off_t) -> size_t;
    pub fn fuse_reply_ioctl_retry(req: fuse_req_t, in_iov: *Struct_iovec,
                                  in_count: size_t, out_iov: *Struct_iovec,
                                  out_count: size_t) -> c_int;
    pub fn fuse_reply_ioctl(req: fuse_req_t, result: c_int, buf: *c_void,
                            size: size_t) -> c_int;
    pub fn fuse_reply_ioctl_iov(req: fuse_req_t, result: c_int,
                                iov: *Struct_iovec, count: c_int) -> c_int;
    pub fn fuse_reply_poll(req: fuse_req_t, revents: c_uint) -> c_int;
    pub fn fuse_lowlevel_notify_poll(ph: *mut Struct_fuse_pollhandle) ->
     c_int;
    pub fn fuse_lowlevel_notify_inval_inode(ch: *mut Struct_fuse_chan,
                                            ino: fuse_ino_t, off: off_t,
                                            len: off_t) -> c_int;
    pub fn fuse_lowlevel_notify_inval_entry(ch: *mut Struct_fuse_chan,
                                            parent: fuse_ino_t,
                                            name: *c_schar, namelen: size_t)
     -> c_int;
    pub fn fuse_lowlevel_notify_delete(ch: *mut Struct_fuse_chan,
                                       parent: fuse_ino_t, child: fuse_ino_t,
                                       name: *c_schar, namelen: size_t) ->
     c_int;
    pub fn fuse_lowlevel_notify_store(ch: *mut Struct_fuse_chan,
                                      ino: fuse_ino_t, offset: off_t,
                                      bufv: *mut Struct_fuse_bufvec,
                                      flags: Enum_fuse_buf_copy_flags) ->
     c_int;
    pub fn fuse_lowlevel_notify_retrieve(ch: *mut Struct_fuse_chan,
                                         ino: fuse_ino_t, size: size_t,
                                         offset: off_t, cookie: *mut c_void)
     -> c_int;
    pub fn fuse_req_userdata(req: fuse_req_t) -> *mut c_void;
    pub fn fuse_req_ctx(req: fuse_req_t) -> *Struct_fuse_ctx;
    pub fn fuse_req_getgroups(req: fuse_req_t, size: c_int, list: *mut gid_t)
     -> c_int;
    pub fn fuse_req_interrupt_func(req: fuse_req_t,
                                   func: fuse_interrupt_func_t,
                                   data: *mut c_void);
    pub fn fuse_req_interrupted(req: fuse_req_t) -> c_int;
    pub fn fuse_lowlevel_is_lib_option(opt: *c_schar) -> c_int;
    pub fn fuse_lowlevel_new(args: *mut Struct_fuse_args,
                             op: *Struct_fuse_lowlevel_ops, op_size: size_t,
                             userdata: *mut c_void) ->
     *mut Struct_fuse_session;
    pub fn fuse_session_new(op: *mut Struct_fuse_session_ops,
                            data: *mut c_void) -> *mut Struct_fuse_session;
    pub fn fuse_session_add_chan(se: *mut Struct_fuse_session,
                                 ch: *mut Struct_fuse_chan);
    pub fn fuse_session_remove_chan(ch: *mut Struct_fuse_chan);
    pub fn fuse_session_next_chan(se: *mut Struct_fuse_session,
                                  ch: *mut Struct_fuse_chan) ->
     *mut Struct_fuse_chan;
    pub fn fuse_session_process(se: *mut Struct_fuse_session, buf: *c_schar,
                                len: size_t, ch: *mut Struct_fuse_chan);
    pub fn fuse_session_process_buf(se: *mut Struct_fuse_session,
                                    buf: *Struct_fuse_buf,
                                    ch: *mut Struct_fuse_chan);
    pub fn fuse_session_receive_buf(se: *mut Struct_fuse_session,
                                    buf: *mut Struct_fuse_buf,
                                    chp: *mut *mut Struct_fuse_chan) -> c_int;
    pub fn fuse_session_destroy(se: *mut Struct_fuse_session);
    pub fn fuse_session_exit(se: *mut Struct_fuse_session);
    pub fn fuse_session_reset(se: *mut Struct_fuse_session);
    pub fn fuse_session_exited(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_session_data(se: *mut Struct_fuse_session) -> *mut c_void;
    pub fn fuse_session_loop(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_session_loop_mt(se: *mut Struct_fuse_session) -> c_int;
    pub fn fuse_chan_new(op: *mut Struct_fuse_chan_ops, fd: c_int,
                         bufsize: size_t, data: *mut c_void) ->
     *mut Struct_fuse_chan;
    pub fn fuse_chan_fd(ch: *mut Struct_fuse_chan) -> c_int;
    pub fn fuse_chan_bufsize(ch: *mut Struct_fuse_chan) -> size_t;
    pub fn fuse_chan_data(ch: *mut Struct_fuse_chan) -> *mut c_void;
    pub fn fuse_chan_session(ch: *mut Struct_fuse_chan) ->
     *mut Struct_fuse_session;
    pub fn fuse_chan_recv(ch: *mut *mut Struct_fuse_chan, buf: *mut c_schar,
                          size: size_t) -> c_int;
    pub fn fuse_chan_send(ch: *mut Struct_fuse_chan, iov: *Struct_iovec,
                          count: size_t) -> c_int;
    pub fn fuse_chan_destroy(ch: *mut Struct_fuse_chan);
}
