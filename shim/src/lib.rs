#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use shimlib::defs::*;
use windows::{core::*, Win32::System::Diagnostics::Debug::*};

#[repr(C)]
pub struct tagHOOKAPI {
    DllName: PCSTR,
    FunctionName: PCSTR,
    HookFunction: PVOID,
    NextFunction: PVOID, // Populated by the shim engine
    Reserved1: PVOID,
    Reserved2: PVOID,
}

#[no_mangle]
pub fn GetHookAPIs(pszArgs: PCSTR, pwszShim: PCWSTR, pdwHookCount: &DWORD) -> Option<&tagHOOKAPI> {
    unsafe {
        OutputDebugStringW(w!("[PATH] in NotifyShims"));
    }
    None
}

// GetHookAPIs(char *,ushort *,ulong *)
// NotifyShims(char *, unsigned __int16 *, unsigned __int32 *)

#[no_mangle]
pub fn NotifyShims(notification: DWORD, data: PVOID) {
    unsafe {
        OutputDebugStringW(w!("[PATH] in NotifyShims"));
    }
}
