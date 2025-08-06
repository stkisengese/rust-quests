pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> Store {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .max_by_key(|store| store.square_meters)
        .cloned()
        .expect("Mall should have at least one store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut highest_salary = 0.0;
    let mut highest_paid: Vec<(String, Employee)> = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    highest_paid.clear();
                }
                if employee.salary == highest_salary {
                    highest_paid.push((name.clone(), *employee));
                }
            }
        }
    }
    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employee_count: usize = mall.floors   
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum();

    let guard_count: usize = mall.guards.len();
    employee_count + guard_count
}
