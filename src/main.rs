fn main() {
  let my_vec = vec![4, 5, 7, 17, 32, 40];
  println!("Result is {}", binary_search(my_vec, 4));
}

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

fn linear_search(v: Vec<i32>, key: i32) -> i32 {
  for (i, &val) in v.iter().enumerate() {
    if val == key {
      return i as i32;
    }
  }
  return -1;
}