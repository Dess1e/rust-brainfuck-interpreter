use std::fs;

pub fn read_file(filename: &String) -> String {
    match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => {
            panic!("Couldn't open file {}. Error: {}", filename, err.to_string())
        }
    }
}

pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
