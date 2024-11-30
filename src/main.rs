fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_does_not_work() {
        assert_eq!(1i32, 2i32);
    }
}
