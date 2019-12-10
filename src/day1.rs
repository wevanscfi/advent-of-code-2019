use std::fs::File;

/// Strucutre to track each modules mass and required fuel
struct Module {
    mass: i32,
}

impl Module {
    fn new(mass: i32) -> Module {
        Module { mass }
    }

    fn fuel_required_simple(&self) -> i32 {
        return self.mass / 3 - 2;
    }

    fn fuel_required(&self) -> i32 {
        return Module::calc_fuel(self.mass)
    }

    fn calc_fuel(mass: i32) -> i32 {
        let fuel = (mass / 3) - 2;
        if fuel < 0 {
            return 0
        } else {
            return fuel + Module::calc_fuel(fuel)
        }
    }
}

fn modules_from_csv(file_path: String) -> Vec<Module> {
    let file = File::open(file_path)
        .expect("Can not open file");
    let mut rdr = csv::Reader::from_reader(file);
    let mut modules = Vec::new();
    for result in rdr.records() {
        let record = result.expect("invalid record");
        let mass: i32 = record[0].parse().expect("Not a number!");
        let module = Module { mass };
        modules.push(module);
    }
    return modules;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_one() {
        let m = Module::new(12i32);
        assert_eq!(m.fuel_required_simple(),2i32);
    }
    
    #[test]
    fn test_two() {
        let m = Module::new(14i32);
        assert_eq!(m.fuel_required_simple(),2i32);
    }

    #[test]
    fn test_three() {
        let m = Module::new(1969i32);
        assert_eq!(m.fuel_required_simple(),654i32);
    }

    #[test]
    fn first_task() {
        let modules = modules_from_csv("inputs/day1.csv".to_string());
        assert_eq!(modules.len(),100);
        let total_mass: i32 = modules.iter().map(|m| m.fuel_required_simple() ).sum();
        assert_eq!(total_mass,3173518);
    }
    
    #[test]
    fn second_task() {
        let modules = modules_from_csv("inputs/day1.csv".to_string());
        assert_eq!(modules.len(),100);
        let total_mass: i32 = modules.iter().map(|m| m.fuel_required() ).sum();
        assert_eq!(total_mass,4757427);
    }


}
