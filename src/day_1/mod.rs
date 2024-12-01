use std::path::Path;

pub fn list_distance(list_0: &mut [i64], list_1: &mut [i64]) -> i64 {
  assert_eq!(list_0.len(), list_1.len());
  list_0.sort();
  list_1.sort();

  list_0.iter().zip(list_1.iter()).map(|(x_0, x_1)| (x_0 - x_1).abs()).sum()
}

pub fn list_similarity(list_0: &mut [i64], list_1: &mut [i64]) -> i64 {
  assert_eq!(list_0.len(), list_1.len());

  list_0.iter().fold(0, |acc, x_0| acc + list_1.iter().filter(|&x_1| x_0 == x_1).sum::<i64>())
}

pub fn read_input_file(path: &Path) -> (Vec<i64>, Vec<i64>) {
  let string = String::from_utf8(std::fs::read(path).unwrap()).unwrap();

  let mut lhs = Vec::new();
  let mut rhs = Vec::new();
  string.split_ascii_whitespace().enumerate().for_each(|(idx, val)| {
    let num_val = val.parse::<i64>().unwrap();
    if idx % 2 == 0 {
      lhs.push(num_val)
    } else {
      rhs.push(num_val)
    }
  });
  (lhs, rhs)
}

#[cfg(test)]
mod tests {

  use super::*;

  const LIST_0: [i64; 6] = [3, 4, 2, 1, 3, 3];
  const LIST_1: [i64; 6] = [4, 3, 5, 3, 9, 3];

  fn input_file_parse() -> (Vec<i64>, Vec<i64>) {
    let path = std::env::current_dir().unwrap();
    let path = path.join("src/day_1/input.txt");
    dbg!(&path);
    read_input_file(&path)
  }

  #[test]
  fn test_list_distance() {
    let distance = list_distance(&mut LIST_0.clone(), &mut LIST_1.clone());
    assert_eq!(distance, 11);
  }

  #[test]
  fn get_example_answer_part_1() {
    let (mut list_0, mut list_1) = input_file_parse();
    let distance = list_distance(&mut list_0, &mut list_1);
    dbg!(&distance);
  }

  #[test]
  fn test_list_similarity() {
    let similarity = list_similarity(&mut LIST_0.clone(), &mut LIST_1.clone());
    assert_eq!(similarity, 31);
  }

  #[test]
  fn get_example_answer_part_2() {
    let (mut list_0, mut list_1) = input_file_parse();
    let similarity = list_similarity(&mut list_0, &mut list_1);
    dbg!(&similarity);
  }
}
