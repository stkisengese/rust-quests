use convert_case::{Case, Casing};

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case
    let is_camel = compared == compared.to_case(Case::Camel);
    let is_snake = compared == compared.to_case(Case::Snake);
    if !is_camel && !is_snake {
        return None;
    }

    // Convert both strings to lowercase for case-insensitive comparison
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();

    // Calculate edit distance
    let edit_dist = edit_distance(&compared_lower, &expected_lower);
    let expected_len = expected_lower.len() as f64;
    let similarity = (1.0 - (edit_dist as f64 / expected_len)) * 100.0;

    // Round to nearest integer
    let similarity_rounded = similarity.round() as u32;

    // Check if similarity is more than 50%
    if similarity_rounded > 50 {
        Some(format!("{}%", similarity_rounded))
    } else {
        None
    }
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let len1 = source.chars().count();
    let len2 = target.chars().count();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else {
                let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                dp[i][j] = (dp[i - 1][j - 1] + cost)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1);
            }
        }
    }
    dp[len1][len2]
}