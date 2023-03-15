use windows::{core::*, Win32::System::Diagnostics::Debug::*};

fn main() {
    unsafe {
        OutputDebugStringW(w!("[PATH] shimee main"));
    }
    println!("from shimee");
}
