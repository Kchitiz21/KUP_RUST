#[cfg(test)]
#[test]
fn substring() {
    use crate::task_1::generating_subset::_generating_substring;
   assert_eq!(generate_substring("pa".to_string()), ["p", "pa", "a"]);
}

#[test]
fn pattern() {
    use crate::task_1::search_pattern::_find_pattern;
    assert_eq!(
        _find_pattern("Pankaj Chaudhry".to_string(), "Cha".to_string()),
        "7".to_string()
    );
    assert_eq!(
        _find_pattern("Hyundai".to_string(), "Verna".to_string()),
        "no match".to_string()
    );
}
#[test]
fn check_string() {
    use crate::task_2::string_manipulation::_desired_output;

    assert_eq!(_desired_output("jjdhid", "ikjhjk", "rtysgi"), "itdsgk");
}
