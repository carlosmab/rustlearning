pub fn sort(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() <= 1 {
        return numbers;
    }

    let pivot = numbers[0];
    let (smaller, larger): (Vec<i32>, Vec<i32>) = numbers[1..]
        .iter()
        .copied()
        .partition(|&n| n < pivot);
    
    let mut result = sort(smaller);
    result.push(pivot);
    result.extend(sort(larger));
    result
}