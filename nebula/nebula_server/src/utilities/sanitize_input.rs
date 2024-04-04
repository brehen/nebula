use std::collections::HashMap;

pub fn sanitize_input(func_name: &str, input: &str, limits: &HashMap<&str, u64>) -> String {
    if let Some(max_limit) = limits.get(func_name) {
        let input_value = input.parse::<u64>().unwrap_or(0);
        if input_value < *max_limit {
            input_value.to_string()
        } else {
            max_limit.to_string()
        }
    } else {
        input.to_string()
    }
}

pub fn get_limits() -> HashMap<&'static str, u64> {
    #[cfg(debug_assertions)]
    {
        HashMap::new()
    }

    #[cfg(not(debug_assertions))]
    {
        HashMap::from([
            ("exponential", 500000),
            ("factorial", 500000),
            ("fibonacci", 500000),
            ("fibonacci-recursive", 40),
            ("prime-number", 500000),
        ])
    }
}
