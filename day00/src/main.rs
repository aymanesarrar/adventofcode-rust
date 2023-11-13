use std::fs;


fn main() {
    let contents = fs::read_to_string("src/file.txt").expect("Should have been able to read the file");
    let  arr = contents.split("\n");
    let mut count = 0;
    let mut vec: Vec<i32> = Vec::new();
    for line in arr {
        if line.is_empty() {
            vec.push(count);
            count = 0;
        } else {
            let nb: i32 = line.parse().unwrap();
            count += nb;
        }
    }
    vec.sort();
    
    println!("{:?}", vec.last().copied())
}
