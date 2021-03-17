// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
// impl AppendBar for Vec<String> {
//     fn append_bar(self) -> Vec<String> {
//         let mut withBar :Vec<String> = Vec::new();
//         for string in self.iter() {
//             withBar.push(string.to_owned() + "Bar");
//         }
//         withBar
//     }
// }

// mutates
// impl AppendBar for Vec<String> {
//     fn append_bar(mut self) -> Self {
//         self.push(String::from("Bar"));
//         self
//     }
// }

// copies
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut withBar = self.clone();
        withBar.push(String::from("Bar"));
        withBar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
