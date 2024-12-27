#![allow(dead_code)]

use core::arch::asm;

#[inline]
pub unsafe fn syscall0(sc_num: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0x80",
        in("x16") sc_num,
        lateout("x0") ret,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall1(sc_num: usize, arg1: usize) -> usize {
    let ret: usize;

    asm!(
        "svc 0x80",
        in("x16") sc_num,
        inlateout("x0") arg1 => ret,
        options(nostack, preserves_flags)
    );

    ret
}

#[inline]
pub unsafe fn syscall2(sc_num: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;

    asm!(
        "svc 0x80",
        in("x16") sc_num,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        options(nostack, preserves_flags)
    );

    ret
}

#[inline]
pub unsafe fn syscall3(sc_num: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;

    asm!(
        "svc 0x80",
        in("x16") sc_num,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        in("x2") arg3,
        options(nostack, preserves_flags)
    );

    ret
}

#[inline]
pub unsafe fn syscall4(sc_num: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0x80",
        in("x16") sc_num,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall5(
    sc_num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0x80",
        in("x16") sc_num,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        in("x4") arg5,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall6(
    sc_num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0x80",
        in("x16") sc_num,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        in("x4") arg5,
        in("x5") arg6,
        options(nostack, preserves_flags)
    );
    ret
}
