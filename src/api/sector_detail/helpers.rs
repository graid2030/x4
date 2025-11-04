use std::collections::HashMap;

/// Convert number to Roman numerals
pub fn to_roman(n: u32) -> String {
    let values = [(10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];
    let mut result = String::new();
    let mut num = n;
    for (val, symbol) in values {
        while num >= val {
            result.push_str(symbol);
            num -= val;
        }
    }
    result
}

/// Capitalize each word
pub fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build factory name from source entry
/// entry format: "arg_hullparts" → "ARG Hull Parts Factory I"
pub fn build_factory_name(entry: &str, nameindex: u32, _component_names: &HashMap<String, String>) -> String {
    let parts: Vec<&str> = entry.split('_').collect();
    if parts.len() >= 2 {
        let faction = parts[0].to_uppercase();
        let ware = parts[1..].join(" ");
        let roman = to_roman(nameindex);
        format!("{} {} Factory {}", faction, capitalize_words(&ware), roman)
    } else {
        entry.to_string()
    }
}
