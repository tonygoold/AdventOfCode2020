use std::collections::{HashMap,HashSet};
use std::io::{BufRead,BufReader};
use std::fs::File;
use regex::Regex;
use graphlib::{Graph,GraphErr,VertexId};

struct NamedGraph {
  graph: Graph<String>,
  name_map: HashMap<String, VertexId>,
}

impl NamedGraph {
  fn new() -> NamedGraph {
    NamedGraph {
      graph: Graph::new(),
      name_map: HashMap::new()
    }
  }

  fn get(&mut self, name: &str) -> VertexId {
    match self.name_map.get(name) {
      Some(id) => *id,
      None => {
        let id = self.graph.add_vertex(name.to_string());
        self.name_map.insert(name.to_string(), id);
        id
      }
    }
  }

  fn add_edge(&mut self, from: &VertexId, to: &VertexId, count: usize) -> Result<(), GraphErr> {
    self.graph.add_edge_with_weight(from, to, (count as f32) / 10.0)
  }

  fn count_inbound(&self, name: &str) -> Option<usize> {
    let root_id = self.name_map.get(name)?;
    let mut seen = HashSet::new();
    Some(self.count_inbound_impl(root_id, &mut seen))
  }

  fn count_inbound_impl(&self, id: &VertexId, seen: &mut HashSet<VertexId>) -> usize {
    let mut count = 0;
    for inbound in self.graph.in_neighbors(id) {
      if seen.insert(*inbound) {
        count += 1 + self.count_inbound_impl(inbound, seen);
      }
    }
    count
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());

  let line_re = Regex::new(r"^(\w+ \w+) bags contain (.*)\.$")
    .expect("Unable to compile line regex");
  let bag_re = Regex::new(r"^(\d+) (\w+ \w+) bags?")
    .expect("Unable to compile bag regex");

  let mut graph = NamedGraph::new();

  for line in lines {
    let caps = line_re.captures(&line).expect("Line does not match pattern");
    if &caps[2] == "no other bags" {
      continue;
    }

    let src_id = graph.get(&caps[1]);
    for bag in caps[2].split(", ") {
      let caps = bag_re.captures(&bag).expect("Bag does not match pattern");
      let count = usize::from_str_radix(&caps[1], 10).expect("Bag count is not a usize");
      let dst_id = graph.get(&caps[2]);
      graph.add_edge(&src_id, &dst_id, count).expect("Failed to add edge");
    }
  }

  match graph.count_inbound("shiny gold") {
    Some(count) => println!("Found {} reachable nodes", count),
    None => println!("Did not find shiny gold in the graph")
  }
}
