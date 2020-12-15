use std::env;
use std::fmt::{self,Write};
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Seat {
  Floor,
  Unoccupied,
  Occupied,
}

impl fmt::Display for Seat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_char(match self {
      Seat::Floor => '.',
      Seat::Unoccupied => 'L',
      Seat::Occupied => '#',
    })
  }
}

#[derive(Eq, PartialEq)]
struct Seating {
  seats: Vec<Vec<Seat>>,
  width: usize,
  height: usize,
}

impl fmt::Display for Seating {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for row in self.seats.iter() {
      for seat in row.iter() {
        write!(f, "{}", seat)?
      }
      write!(f, "\n")?
    }
    Ok(())
  }
}

impl Seating {
  fn new(lines: &mut dyn Iterator<Item = String>) -> Seating {
    let mut width: usize = 0;
    let seats: Vec<Vec<Seat>> = lines.map(|line| {
      let row: Vec<Seat> = line.chars().map(|x| match x {
        '.' => Seat::Floor,
        'L' => Seat::Unoccupied,
        '#' => Seat::Occupied,
        _ => panic!("Invalid seating character")
      }).collect();
      if width == 0 {
        width = row.len();
      } else if width != row.len() {
        panic!("Inconsistent row width");
      }
      row
    }).collect();
    let height = seats.len();
    Seating{
      seats,
      width,
      height,
    }
  }

  fn seated_neighbours(&self, x: usize, y: usize) -> usize {
    let minx = if x == 0 { x } else { x - 1 };
    let miny = if y == 0 { y } else { y - 1 };
    let maxx = if x + 1 < self.width { x + 1 } else { x };
    let maxy = if y + 1 < self.height { y + 1 } else { y };
    let mut count = 0;
    let mut checked = 0;
    for yn in miny..=maxy {
      for xn in minx..=maxx {
        if xn != x || yn != y {
          count += match self.seats[yn][xn] {
            Seat::Floor | Seat::Unoccupied => 0,
            Seat::Occupied => 1,
          };
          checked += 1;
        }
      }
    }
    let is_x_edge = x == 0 || x == self.width - 1;
    let is_y_edge = y == 0 || y == self.height - 1;
    match (is_x_edge, is_y_edge) {
      (true, true) => assert_eq!(checked, 3),
      (true, _) | (_, true) => assert_eq!(checked, 5),
      _ => assert_eq!(checked, 8),
    };
    count
  }

  fn next(&self) -> Seating {
    let rows = self.seats.iter().enumerate().map(|(y, row)| {
      row.iter().enumerate().map(|(x, seat)| {
        let adjacent = self.seated_neighbours(x, y);
        if *seat == Seat::Unoccupied && adjacent == 0 {
          Seat::Occupied
        } else if *seat == Seat::Occupied && adjacent >= 4 {
          Seat::Unoccupied
        } else {
          *seat
        }
      }).collect()
    });
    Seating{
      seats: rows.collect(),
      width: self.width,
      height: self.height,
    }
  }

  fn num_occupied(&self) -> usize {
    self.seats.iter().flatten().filter(|x| **x == Seat::Occupied).count()
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let mut lines = reader.lines().into_iter().map(|x| x.unwrap());
  let verbose = env::args().nth(1).filter(|arg| arg == "-v").is_some();

  let mut prev = Seating::new(&mut lines);
  if verbose {
    println!("{}", &prev);
  }
  loop {
    let next = prev.next();
    if verbose {
      println!("{}", &next);
    }
    if next == prev {
      println!("{} seats are occupied after stabilizing", next.num_occupied());
      break;
    }
    prev = next;
  }
}
