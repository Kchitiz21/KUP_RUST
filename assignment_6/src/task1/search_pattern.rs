/// The pattern function is used to search pattern .
///
/// #Arguments
///
/// string1 - Given input string  to Apply to find pattern
/// search_pattern - string to checked with string1
///
/// #Return
///
/// Returns the String to given value with index and match or not
 fn pattern(string1: String, search_pattern: String) -> String {
    let string_vec: Vec<char> = string1.chars().collect();
    let search_pattern_vec: Vec<char> = search_pattern.chars().collect();
    let mut iteration_count = 0;
    let mut check;
    let mut temp;
    for index in 0..(string_vec.len() - search_pattern_vec.len() + 1) {
        temp = index;
        check = index;
        for element_match_index in &search_pattern_vec {
            if element_match_index == &string_vec[temp] {
                iteration_count += 1;
            }
            if iteration_count == search_pattern.len() {
                return check.to_string();
            }
            temp += 1;
        }
        iteration_count = 0;
    }
    "no match".to_string()
}