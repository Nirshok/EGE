use std::fs;
use std::error::Error;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {
            file_path,
        })
    }
    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = to_i32(&contents);
    
    let (x, y) = pairs_and_sum(results);

    println!("{x}, {y}");
    
    Ok(())
}

pub fn to_i32(contents: &str) -> Vec<i32> {
    let strings: Vec<&str> = contents.split_whitespace().collect();
    
    let mut numbs: Vec<i32> = Vec::new();
    
    for numbstr in strings {
        if let Ok(num) = numbstr.parse::<i32>() {
            numbs.push(num);
        }
    }
    numbs        
}

pub fn pairs_and_sum(to_i32: Vec<i32>) -> (u32, i64) {
    let mut previous = to_i32[0];
    let mut count = 0;
    let mut sum: i64 = i32::MIN as i64;

    if to_i32.len() < 2 {
        println!("Not enought argunments!");
        return (0, 0);
    }

    for numb in &to_i32[1..] {
        if previous % 3 == 0 || numb % 3 == 0 {
            count += 1;
            let sum_i64 = (previous + numb) as i64;

            if sum_i64 > sum {
                sum = sum_i64;
            }
        }
        previous = *numb;
    }

    if count == 0 {
        sum = 0;
    }

    (count, sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_i32_works() {
        let contents = "1234 -6654 556 -3 -9";
        
        assert_eq!(vec![1234, -6654, 556, -3, -9], to_i32(contents));
    }

    #[test]
    fn pairs_and_sum_works() {
        let i32vec = vec![1234, -6654, 556, -3, -9];

        let (x, y) = pairs_and_sum(i32vec);
        println!("{x} {y}");
    }

    #[test]
    fn running() {
        let path = "numbers.txt";
        let results = to_i32(path);

        let (x, y) = pairs_and_sum(results);

        assert_eq!(x, 4);
        assert_eq!(y, 553);
    }
}