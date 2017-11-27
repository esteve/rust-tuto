// import the regex crate into our project.
extern crate regex;

// declare the date module (src/date.rs)
mod date;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
