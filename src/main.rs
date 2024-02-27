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
const BG: Pixel = Pixel::new(54, 58, 79);
const EMPTY: Pixel = BG;
const LEFT_STICK: Pixel = Pixel::new(237, 135, 150); //Pixel::new(202, 211, 245);
const RIGHT_STICK: Pixel = LEFT_STICK;
const GARBAGE: Pixel = Pixel::PURPLE;

fn main() {
    let n = 8;
    let r = unsafe { sierpinsky_extern(n) };
    println!("points to {}", unsafe {*r.ptr });

    let width = 2usize.pow(n as u32);
    let height = 2usize.pow(n as u32 -1);

    let mut new_arr = vec![0; r.len as usize]; // IF WE DON'T COPY IT FIRST, THE IMAGE DATA
                                               // OVERRIDES THE FUCKING DATA,
                                               // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    for i in 0..(r.len) {
        let k = unsafe { *(r.ptr.add(i as usize)) };
        new_arr[i as usize] = k;
    }

    let mut img = Box::new(ImagePPM::new(width, height, BG));

    for i in 0..(r.len) {
        let (x, y) = idx_to_coords(i as usize, width);
        let k = new_arr[i as usize];
        let col = match k {
            1 => EMPTY,
            2 => LEFT_STICK,
            3 => RIGHT_STICK,
            _ => GARBAGE,
        };
        //println!("val {i} = 0x{:X} => {col:?}", k);
        *img.get_mut(x, height - y - 1).unwrap() = col;
    }

    img.save_to_file("triangle6-2.ppm").unwrap();
}
