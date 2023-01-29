#[no_mangle]
pub fn execute() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use crate::execute;

    #[test]
    fn it_works() {
        execute();
        assert!(true);
    }
}
