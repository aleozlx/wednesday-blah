#![feature(core_intrinsics)]
#![feature(test)]
 #![feature(extern_crate_item_prelude)]

extern crate test;
#[cfg(test)]
mod tests {

// #[inline(never)]
// #[no_mangle]
extern crate rayon;
// #[test]
// #[test]
// use rand::Rng; use rand::distributions::{Range, IndependentSample};
// #[test]
fn zz(n: usize) -> f32 {
    let mut s = 0.0f32;
    let a : Vec<f32> = (0..n).map(|x| {x as f32}).collect();//vec![7.0f32;n];
    let b : Vec<f32> = (0..n).map(|x| {x as f32}).collect();
    for i in 0..n {
        s = a[i].mul_add(b[i], s); //unsafe {std::intrinsics::fmaf32(a[i], b[i], s)};
    }
    return s;
}

// #[test]
fn yy(n: usize) -> f32 {
    let mut s = 0.0f32;
    let a: Vec<f32> = (0..n).map(|x| {x as f32}).collect();
    let b : Vec<f32> = (0..n).map(|x| {x as f32}).collect();
    for i in 0..n {
        s += a[i]*b[i];
    }
    return s;
}
// #[test]
fn ww(n: usize) -> f32 {
    let mut s = 0.0f64;
let a: Vec<f64> = (0..n).map(|x| {x as f64}).collect();
    let b : Vec<f64> = (0..n).map(|x| {x as f64}).collect();
    for i in 0..n {
        s += a[i]*b[i];
    }
    return s as f32;
}

use rayon::prelude::*;
fn xx(n: usize) -> f32 {
let a: Vec<f32> = (0..n).map(|x| {x as f32}).collect();
    let b : Vec<f32> = (0..n).map(|x| {x as f32}).collect();
    (0..n).into_par_iter().map(|i| a[i]*b[i]).sum()
    // for i in 0..n {
    //     s += a[i]*b[i];
    // }
    // return s as f32;
}




use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
fn dskfja()->std::io::Result<()>{
    let mut file = OpenOptions::new().create(true).append(true).open("foo.txt")?;

    file.write(b"some bytes")?;
    Ok(())
}

#[bench]
fn test_fma32 (b: &mut test::Bencher) {
    // let x = std::env::args().into_iter();
    // unsafe {zz(std::env::args()[1])}
    b.iter(|| {
        
        // if let Ok(_)= dskfja() {
            
        // }
        zz(1e5 as usize)});
    // ;
}

#[bench]
fn test_f32 (b: &mut test::Bencher) {
    b.iter(|| yy(1e5 as usize));
}

#[bench]
fn test_f64 (b: &mut test::Bencher) {
    b.iter(|| ww(1e5 as usize));
}

#[bench]
fn test_par32 (b: &mut test::Bencher) {
    b.iter(|| xx(1e5 as usize));
}

}


fn main() {
    // std::process::exit(unsafe {zz()} as i32);
}
