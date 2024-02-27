use std::ffi::c_int;
use ppmitzador::*;

#[repr(C)]
#[derive(Debug)]
struct PVec {
    ptr: *const u8,
    len: u32,
}


extern "C" {
    fn sierpinsky_extern(n: c_int) -> PVec;
}

pub(crate) fn idx_to_coords(i: usize, w: usize) -> (usize, usize) {
   ( 
    i % w,
    i / w, 
    )
}
fn main() {
    let n= 5;
    let r = unsafe { sierpinsky_extern(n) };
    //dbg!(&r);
    println!("points to {}", unsafe {*r.ptr });
    for i in 00..(r.len) {
        let k = unsafe { *(r.ptr.add(i as usize)) };
        println!("val {i} = 0x{:X} = {}", k, k as char);
    }
    const BG: Pixel = Pixel::new(0, 0, 0);
    const EMPTY: Pixel = BG;
    const LEFT_STICK: Pixel = Pixel::WHITE;
    const RIGHT_STICK: Pixel = Pixel::WHITE;
    const GARBAGE: Pixel = Pixel::PURPLE;
    let width = 2usize.pow(n as u32);
    let height = 2usize.pow(n as u32 -1);
    let mut img = ImagePPM::new(width, height, BG);

    for i in 0..(r.len) {
        let (x, y) = idx_to_coords(i as usize, width);
        let k = unsafe { *(r.ptr.add(i as usize)) };
        let col = match k {
            1 => EMPTY,
            2 => LEFT_STICK,
            3 => RIGHT_STICK,
            _ => GARBAGE,
        };
        *img.get_mut(x, height - y - 1).unwrap() = col;
        
    }
    img.save_to_file("triangle5.ppm").unwrap();
    //for i in 0..(r.len) {
    //    let k = unsafe { *(r.ptr.add(i as usize)) };
    //    match k {
    //        1 => print!(" "),
    //        2 => print!("/"),
    //        3 => print!("\\"),
    //        _ => print!("·"),
    //    }
    //    let s = r.len / 2u32.pow(n as u32 -1);
    //    if (i+1) % s == 0 {println!("n");}

    //}
}
