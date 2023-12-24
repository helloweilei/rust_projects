pub mod smart_pointer;
pub mod seasons {
  pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter
  }

  pub fn is_holiday(season: &Season) -> bool {
    match season {
        Season::Summer => true,
        _ => false
    }
  }
}

pub use seasons::is_holiday; // re-exporting

pub fn intersection(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
  let mut result = vec![];
  for num in vec1.iter() {
    if vec2.contains(num) {
      result.insert(result.len(), *num);
    }
  }

  result
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}