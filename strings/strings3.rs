// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut new = String::from(input);

    for j in [0, 1] {
        new = new.chars().rev().collect();

        let mut aux = String::from("");
        for (i, c) in new.chars().enumerate() {
            aux = new[i..].to_string();
            if c != ' ' {
                break;
            }
        }
        new = aux.to_string();
    }
    new
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    let mut new = String::from("");
    let cinput: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < cinput.len() {
        if cinput[i] == 'c' && cinput[i+1] == 'a' && cinput[i+2] == 'r' && cinput[i+3] == 's' {
            new.push_str("balloons");
            i += 4;
        } else {
            new.push(cinput[i]);
            i += 1;
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
