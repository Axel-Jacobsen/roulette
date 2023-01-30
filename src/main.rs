/* Many lines of un-idiomatic Rust
 */

mod commands;

fn vecu8_to_string(vs: Vec<u8>) -> String {
    String::from_utf8(vs).unwrap()
}

fn main() {
    let vs = ["FIXME", "TODO"].map(String::from).to_vec();

    let res = commands::git_grep(vs).unwrap();

    let stdout_str = vecu8_to_string(res.stdout);

    let r = commands::handle_git_grep(&stdout_str);

    for i in r {
        println!("{:?}", i);
    }
}

#[cfg(test)]
mod main_tests {
    use crate::synth_or;

    #[test]
    fn basic_symbol_freq() {
        let vs = ["FIXME", "TODO", "test", ":)"].map(String::from).to_vec();
        let res = synth_or(vs);
        assert_eq!(res, "(FIXME|TODO|test|':)')");
    }
}
