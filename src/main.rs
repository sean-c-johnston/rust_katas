mod cli;

fn main() {
    cli::run_cli(std::io::stdout());
}

// $5 + $5 = $10
// $5 + £5 = $15 if 2:1 rate

struct Dollar {
    amount: i32,
}

impl Dollar {
    pub(crate) fn add(&self, p0: i32) -> Dollar {
        Dollar { amount: self.amount + p0 }
    }
}

#[cfg(test)]
#[test]
fn test_addition() {
    let five_dollars = Dollar { amount: 5 };
    let ten_dollars = five_dollars.add(5);

    assert_eq!(10, ten_dollars.amount);
}