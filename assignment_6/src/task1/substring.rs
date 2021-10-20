/// The substring function is used to find all substring of input string
///
/// #Arguments
///
/// input_string - Taking String as input and generating substring of input string
///
/// #Return
///
/// Returns a vector<Sting> having all substring of input string
///
 fn substring(input: String) -> Vec<String> {
    if input.is_empty() {
        return vec!["".to_string()];
    }
    let mut substring1: Vec<String> = Vec::new();
    let mut substring: &str;
    for index_1 in 0..input.len() {
        for index_2 in index_1..input.len() {
            substring = &input[index_1..(index_2 + 1)];
            substring1.push(substring.to_string());
        }
    }
    substring1
}