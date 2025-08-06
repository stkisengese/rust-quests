pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&str, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| &floor.stores)
        .max_by_key(|(_, store)| store.square_meters)
        .map(|(name, store)| (name.as_str(), store))
        .expect("Mall should have at least one store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut highest_salary = f64::NEG_INFINITY;
    let mut highest_paid: Vec<(&str, Employee)> = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    highest_paid.clear();
                }
                if employee.salary == highest_salary {
                    highest_paid.push((name.as_str(), *employee));
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

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    // Calculate total floor size
    let total_size: u64 = mall.floors.values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    // Calculate required number of guards (rounding up)
    let required_guards = ((total_size + 199) / 200) as usize;
    let current_guards = mall.guards.len();

    // Add guards if needed
    if current_guards < required_guards {
        let needed = required_guards - current_guards;
        for (name, guard) in guards.into_iter().take(needed) {
            mall.guards.insert(name, guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors
        .values_mut()
        .flat_map(|floor| floor.stores.values_mut())
        .flat_map(|store| store.employees.values_mut())
        .for_each(|employee| {
            let working_hours = employee.working_hours.1 - employee.working_hours.0;
            let adjustment = employee.salary * 0.1;

            if working_hours < 10 {
                employee.salary -= adjustment;
            } else if working_hours >= 10 {
                employee.salary += adjustment;
            }
        })
}