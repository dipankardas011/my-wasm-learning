pub fn reverse(input: &String) -> String {
    return input.chars().rev().collect();
}

pub fn inspect(inp: &String, digit: bool) -> (i32, String) {
    if !digit {
        (inp.len() as i32, String::from("char"))
    }else {
        (inspect_number(inp), String::from("digit"))
    }
}

fn inspect_number(inp: &String) -> i32 {
    let mut count = 0;
    for c in inp.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
    return count;
}
