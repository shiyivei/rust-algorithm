#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]

pub mod complexity_analysis;

fn cal(n: u64) -> u64 {
    let mut sum = 0;

    let i = 1u64;

    for i in 0..n {
        sum = sum + i
    }

    sum
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_complexity_analysis() {
        let sum = cal(10);

        println!("sum is {}", sum);
    }
}
