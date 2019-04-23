use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    //unimplemented!("classify {}", num);
    let mut factors = HashSet::new();
    let itself: u64 = num;
    let mut sum: u64 = 0;

    for i in 1..(itself-1) {
        if num % i == 0 {
            factors.insert(i);
        }
    }

    for factor in &factors {
        sum = sum + factor;
    }

    if sum == num {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}
