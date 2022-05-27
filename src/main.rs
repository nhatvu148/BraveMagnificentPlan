use std::time::Instant;

fn main() {
  // let my_vec = vec![4, 5, 7, 17, 32, 40];
  // println!("Result is {}", binary_search(my_vec, 4));
  let now = Instant::now();
  println!("Result is {}", square_root(100000000));
  println!("{}", now.elapsed().as_micros());
}

#[allow(unused_assignments)]
fn square_root(n: u128) -> u128 { // O(log(N))
  let mut low = 0;
  let mut high = n;
  let mut mid = 0;

  while high - low > 1 {
    mid = low + (high - low) / 2;
    if mid * mid > n {
      high = mid;
    } else {
      low = mid;
    }
  }
  return low;
}

#[allow(dead_code)]
fn binary_search(v: Vec<i32>, key: i32) -> i32 {
  let mut start = 0;
  let mut end = v.len() - 1;

  while start <= end {
    let mid = (start + end) / 2;
    if v[mid] == key {
      return mid as i32;
    } else if v[mid] > key {
      end = mid - 1;
    } else {
      start = mid + 1;
    }
  }
  
  return -1;
}

#[allow(dead_code)]
fn linear_search(v: Vec<i32>, key: i32) -> i32 {
  for (i, &val) in v.iter().enumerate() {
    if val == key {
      return i as i32;
    }
  }
  return -1;
}