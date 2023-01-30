/* Many lines of un-idiomatic Rust
 */

mod commands;

fn main() {
    let vs = ["FIXME", "TODO"].map(String::from).to_vec();

    let r = commands::git_grep(vs).unwrap();

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
