use std::ffi::c_int;
use ppmitzador::*;

#[repr(C)]
#[derive(Debug)]
struct PVec {
    ptr: *const u8,
    len: u32,
}


extern "C" { fn sierpinsky_extern(n: c_int) -> PVec; }

pub(crate) fn idx_to_coords(i: usize, w: usize) -> (usize, usize) { (i % w, i / w) }
const BG: Pixel = Pixel::new(36, 39, 58);
const EMPTY: Pixel = BG;
const LEFT_STICK: Pixel = Pixel::new(166, 218, 149);
const RIGHT_STICK: Pixel = LEFT_STICK; //Pixel::BLACK; // Neat shadows if set to a dark color!
const GARBAGE: Pixel = Pixel::PURPLE;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(1).expect("You forgot to provide n!")
        .parse().expect("n (first argument) must be a positive number");
    let r = unsafe { sierpinsky_extern(n as i32) };
    let width = 2usize.pow(n as u32);
    let height = 2usize.pow(n as u32);

    let mut img = ImagePPM::new(width, height, BG);

    for i in 0..(r.len as usize) {
        let (x, y) = idx_to_coords(i, width);
        let k = unsafe { *(r.ptr.add(i as usize)) };
        let col = match k {
            1 => EMPTY,
            2 => LEFT_STICK,
            3 => RIGHT_STICK,
            _ => GARBAGE,
        };

        // Duplicate each row
        *img.get_mut(x, height - y*2 - 2).unwrap() = col;
        *img.get_mut(x, height - y*2 - 1).unwrap() = col;
    }


    img.save_to_file(format!("triangle-{}.ppm", n)).unwrap();
}
