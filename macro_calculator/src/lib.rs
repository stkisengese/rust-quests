use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Extract kcal from the second element of calories tuple
        let kcal_str = &food.calories.1;
        let kcal = kcal_str
            .trim_end_matches("kcal")
            .parse::<f64>()
            .expect("Invalid kcal format");

        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Round to 2 decimal places
    fn round_to_2_decimal(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    json::object! {
        "cals" => round_to_2_decimal(total_cals),
        "carbs" => round_to_2_decimal(total_carbs),
        "proteins" => round_to_2_decimal(total_proteins),
        "fats" => round_to_2_decimal(total_fats)
    }
}