fn main() {
    println!("Hello, world!");
}
// $5 + $5 = $10
// $5 + £5 = $15 if 2:1 rate
#[cfg(test)]
#[test]
fn test_addition() {
    let five_dollars = Dollar { amount: 5 };
    let ten_dollars = five_dollars.add(5);

    assert_eq!(10, ten_dollars.amount);
}