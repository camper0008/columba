pub fn possible_write_error(res: Result<usize, std::io::Error>) {
    if res.is_err() {
        println!("{}", res.err().unwrap());
    };
}
