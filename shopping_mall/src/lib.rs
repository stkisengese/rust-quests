use std::collections::HashMap;

pub mod mall;
pub use mall::{Mall, Store, Employee, Guard};

/// Returns the Store with the most square_meters
pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| &floor.stores)
        .max_by_key(|(_, store)| store.square_meters)
        .expect("Mall should have at least one store")
}

/// Returns a vector containing the Employee(s) with the highest salary
pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let all_employees: Vec<(&String, &Employee)> = mall.floors
        .values()
        .flat_map(|floor| &floor.stores)
        .flat_map(|(_, store)| &store.employees)
        .collect();
    
    if all_employees.is_empty() {
        return Vec::new();
    }
    
    let max_salary = all_employees
        .iter()
        .map(|(_, employee)| employee.salary)
        .fold(f64::NEG_INFINITY, f64::max);
    
    all_employees
        .into_iter()
        .filter(|(_, employee)| employee.salary == max_salary)
        .collect()
}

/// Returns the number of employees and guards as a usize
pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employee_count = mall.floors
        .values()
        .flat_map(|floor| &floor.stores)
        .map(|(_, store)| store.employees.len())
        .sum::<usize>();
    
    let guard_count = mall.guards.len();
    
    employee_count + guard_count
}

/// Checks if there is at least 1 guard for every 200 square meters of total floor size.
/// If not, adds guards from the provided map to meet the requirement.
pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    // Calculate total floor space
    let total_square_meters: u64 = mall.floors
        .values()
        .flat_map(|floor| &floor.stores)
        .map(|(_, store)| store.square_meters)
        .sum();
    
    // Calculate required number of guards (1 guard per 200 square meters)
    let required_guards = (total_square_meters as f64 / 200.0).ceil() as usize;
    let current_guards = mall.guards.len();
    
    // If we need more guards, add them from the provided map
    if current_guards < required_guards {
        let guards_to_add = required_guards - current_guards;
        let mut guards_iter = guards.into_iter();
        
        for _ in 0..guards_to_add {
            if let Some((name, guard)) = guards_iter.next() {
                mall.guards.insert(name, guard);
            } else {
                break; // No more guards available to add
            }
        }
    }
}

/// For each employee, raises salary by 10% if they work 10+ hours, otherwise decreases by 10%
/// Guards are not considered employees of the mall
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours_worked = employee.working_hours.1 - employee.working_hours.0;
                
                if hours_worked >= 10 {
                    // Raise by 10%
                    employee.salary *= 1.1;
                } else {
                    // Cut by 10%
                    employee.salary *= 0.9;
                }
            }
        }
    }
}