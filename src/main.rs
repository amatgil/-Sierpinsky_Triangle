use std::ffi::c_int;

#[repr(C)]
#[derive(Debug)]
struct PVec {
    ptr: *const u8,
    len: u32,
}


extern "C" {
    fn sierpinsky_extern(n: c_int) -> PVec;
}

fn main() {
    let n= 4;
    let r = unsafe { sierpinsky_extern(n) };
    //dbg!(&r);
    println!("points to {}", unsafe {*r.ptr });
    for i in 00..(r.len) {
        let k = unsafe { *(r.ptr.add(i as usize)) };
        println!("val {i} = 0x{:X} = {}", k, k as char);
    }
    for i in 0..(r.len) {
        let k = unsafe { *(r.ptr.add(i as usize)) };
        match k {
            1 => print!(" "),
            2 => print!("/"),
            3 => print!("\\"),
            _ => print!("·"),
        }
        let s = r.len / 2u32.pow(n as u32 -1);
        if (i+1) % s == 0 {println!("n");}

    }
}
