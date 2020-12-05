use std::io::{BufRead,BufReader};
use std::fs::File;

struct Seat {
  row: usize,
  column: usize,
}

impl Seat {
  fn id(&self) -> usize {
    self.row * 8 + self.column
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let mut seats = Vec::new();
  for line in lines {
    let mut seat = Seat{row: 0, column: 0};
    for c in line.chars() {
      match c {
        'F' => { seat.row = seat.row << 1; },
        'B' => { seat.row = (seat.row << 1) + 1; },
        'L' => { seat.column = seat.column << 1; },
        'R' => { seat.column = (seat.column << 1) + 1; },
        _ => panic!("Unexpected input character")
      }
    }
    seats.push(seat);
  }

  let mut ids: Vec<usize> = seats.iter().map(|seat| seat.id()).collect();
  ids.sort();
  let mut prev = ids[0] - 1;
  for id in &ids {
    if *id != prev + 1 {
      println!("The missing seat id is {}", id - 1);
      break;
    }
    prev = *id;
  }
}
