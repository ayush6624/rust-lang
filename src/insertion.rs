pub fn sort(mut arr: Vec<i32>){
    let n = arr.len();
    for i in 1..n {
        for j in (1..i + 1).rev() {
          if arr[j - 1] <= arr[j] { break; }
          arr.swap(j - 1, j)
        }
      }
    println!("Insetion Sort -> {:?}", arr);
}