use std::{usize};
use text_io::read;
mod bubble;
mod insertion;
fn main(){
    println!("Implementing differnt sorting algorithms in rust");
    let n : i32 = read!();
    let mut arr : Vec<i32> = vec![0; n as usize];
    println!("{}", arr.len());
    for i in 0..arr.len(){
        arr[i] = read!();
    }
    println!("Original Array -> {:?}", arr);
    bubble::sort(arr.clone());
    insertion::sort(arr.clone());
}