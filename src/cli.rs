pub(crate) fn run_cli(mut writer: impl std::io::Write) {
    writeln!(writer, "{}", "error: no arguments").expect("TODO: panic message");
}

#[cfg(test)]
#[test]
fn with_no_args() {
    let mut result = Vec::new();
    run_cli(&mut result);

    assert_eq!(result, b"error: no arguments\n")
}