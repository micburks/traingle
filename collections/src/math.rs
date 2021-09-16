use std::collections::HashMap;

pub fn run() {
    let mut values = vec![1, 2, 3, 4, 1, 2, 3, 2, 3, 1, 1, 1, 35];
    values.sort();

    let mut mode = 0;
    let mut median = 0;
    let mut sum = 0;
    let mut map = HashMap::new();
    let middle = values.len() / 2;

    for (ind, val) in values.iter().enumerate() {
        // median
        if ind == middle {
            median = *val;
        }
        
        // mean
        sum += val;

        // mode
        // Update HashMap
        let entry = map.entry(val).or_insert(0);
        *entry += 1;
        // Update most-commonly-used
        if *entry > mode {
            mode = *val;
        }
    }

    let mean: f64 = (sum as f64) / (values.len() as f64);

    println!("sum - {}", sum);
    println!("len - {}", values.len());
    println!("mean - {}", mean); 
    println!("median - {}", median);
    println!("mode - {}", mode);
}
