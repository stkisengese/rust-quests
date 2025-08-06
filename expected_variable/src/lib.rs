
/// Calculates the edit distance (Levenshtein distance) between two strings
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

/// Checks if a string is in snake case
fn is_snake_case(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase() || c == '_')
}

/// Checks if a string is in camel case
fn is_camel_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    // Must start with lowercase letter
    if !s.chars().next().unwrap().is_lowercase() {
        return false;
    }
    
    // Should not contain underscores or spaces
    if s.contains('_') || s.contains(' ') {
        return false;
    }
    
    // Must contain at least one uppercase letter (to distinguish from all lowercase)
    s.chars().any(|c| c.is_uppercase())
}

/// Converts camelCase to snake_case manually
fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lower = false;
    
    for c in s.chars() {
        if c.is_uppercase() && prev_was_lower {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
        prev_was_lower = c.is_lowercase();
    }
    
    result
}

/// Compares a string to an expected string and returns similarity percentage if > 50%
pub fn expected_variable(a: &str, b: &str) -> Option<String> {
    // Check for spaces - if either contains spaces, return None
    if a.contains(' ') || b.contains(' ') {
        return None;
    }
    
    // Check if the compared string (a) is in camel case or snake case
    if !is_camel_case(a) && !is_snake_case(a) {
        return None;
    }
    
    // Convert both to snake_case for comparison
    let snake_a = if is_camel_case(a) {
        camel_to_snake(a)
    } else {
        a.to_lowercase()
    };
    
    let snake_b = if is_camel_case(b) {
        camel_to_snake(b)
    } else {
        b.to_lowercase()
    };
    
    let distance = edit_distance(&snake_a, &snake_b);
    let similarity = 1.0 - (distance as f64 / b.len() as f64);
    let res = (similarity * 100.0).round();
    
    if res > 50.0 {
        Some(format!("{}%", res as u32))
    } else {
        None
    }
}