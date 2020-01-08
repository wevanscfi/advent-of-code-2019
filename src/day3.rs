use regex::Regex;
use std::collections::{HashSet};
use std::fs;

#[derive(Clone,Eq,PartialEq,Debug,Hash,Copy)]
struct Vector(i32,i32);

impl Vector {
    fn cost(self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn add(self, operand: Vector) -> Vector {
        Vector(self.0 + operand.0, self.1 + operand.1)
    }
}

struct Wire {
    path: Vec<String>,
    points: Vec<Vector>,
}

impl Wire {
    fn new(path: Vec<String>) -> Wire {
        Wire { 
            path: path.clone(),
            points: vec![Vector(0,0)],
        }
    }

    fn find_intersection_points(w1: &Wire, w2: &Wire) -> Vec<Vector> {
        let first = w1.points.iter().cloned().collect::<HashSet<Vector>>();
        let second = w2.points.iter().cloned().collect::<HashSet<Vector>>();
        first.intersection(&second).cloned().collect::<Vec<Vector>>()
    }

    fn find_lowest_cost_intersection(w1: &Wire, w2: &Wire) -> i32 {
        let points = Wire::find_intersection_points(&w1,&w2);
        points.iter().map( |p| p.cost() ).filter( |c| *c != 0 ).min().unwrap()
    }

    fn find_lowest_length_intersection(w1: &Wire, w2: &Wire) -> i32 {
        let points = Wire::find_intersection_points(&w1,&w2);
        points.iter().map( |p| Wire::length_to(w1.points.clone(), p) + Wire::length_to(w2.points.clone(), p) )
            .filter( |c| *c != 0 ).min().unwrap()
    }

    fn length_to(path: Vec<Vector>, point: &Vector) -> i32 {
        path.iter().position(|p| p == point).unwrap() as i32
    }

    fn follow_path(&mut self, dir: Vector, size: i32) {
        let mut from = self.points.last().unwrap().clone();
        for _ in 0..size {
            let to = from.add(dir);
            self.points.push(to.clone());
            from = to;
        }
    }

    fn construct_points(&mut self) {
        for step in self.path.clone().iter() {
            lazy_static! { 
                static ref RE: Regex = Regex::new(r"^(?P<dir>[UDLR])(?P<size>[0-9]*)")
                    .unwrap();
            }
            let caps = RE.captures(step).unwrap();
            let dir = caps.name("dir").map_or("", |m| m.as_str());
            let size = caps.name("size").map_or("", |m| m.as_str())
                .parse::<i32>().unwrap();
            match dir {
                "U" => self.follow_path(Vector(0,1), size),
                "D" => self.follow_path(Vector(0,-1), size),
                "R" => self.follow_path(Vector(1,0), size),
                "L" => self.follow_path(Vector(-1,0), size),
                _ => panic!("Invalid Point"),
            };
        }
    }
}

fn parse_path(path: String) -> Vec<String> {
    path.trim().split(',').map(|s| s.trim().to_string())
        .collect::<Vec<String>>()

}

fn load_paths_from_file(input: String) -> Vec<Vec<String>> {
        let contents = fs::read_to_string(input)
            .expect("Something went wrong reading the file");
        let lines = contents.trim().lines();
        lines.map(|p| parse_path(p.to_string()))
            .collect::<Vec<Vec<String>>>()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_one() {
        let p1 = parse_path("R8,U5,L5,D3".to_string());
        let mut w1 = Wire::new(p1);
        w1.construct_points();
        
        let p2 = parse_path("U7,R6,D4,L4".to_string());
        let mut w2 = Wire::new(p2);
        w2.construct_points();
        
        assert_eq!(Wire::find_lowest_cost_intersection(&w1,&w2), 6);
    }

    #[test]
    fn load_from_file() {
        let paths = load_paths_from_file("inputs/day3.txt".to_string());
        assert_eq!(paths.len(), 2)
    }
    
    #[test]
    fn first_task() {
        let paths = load_paths_from_file("inputs/day3.txt".to_string());
        let mut w1 = Wire::new(paths[0].clone());
        let mut w2 = Wire::new(paths[1].clone());
        w1.construct_points();
        w2.construct_points();
        
        assert_eq!(Wire::find_lowest_cost_intersection(&w1,&w2), 489);
    }
    
    #[test]
    fn test_two() {
        let p1 = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string());
        let mut w1 = Wire::new(p1);
        w1.construct_points();
        
        let p2 = parse_path("U62,R66,U55,R34,D71,R55,D58,R83".to_string());
        let mut w2 = Wire::new(p2);
        w2.construct_points();
        
        assert_eq!(Wire::find_lowest_length_intersection(&w1,&w2), 610);
    }
    
    #[test]
    fn second_task() {
        let paths = load_paths_from_file("inputs/day3.txt".to_string());
        let mut w1 = Wire::new(paths[0].clone());
        let mut w2 = Wire::new(paths[1].clone());
        w1.construct_points();
        w2.construct_points();
        
        assert_eq!(Wire::find_lowest_length_intersection(&w1,&w2), 93654);
    }
}
