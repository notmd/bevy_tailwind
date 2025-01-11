#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let output = std::process::Command::new("cargo")
            .arg("expand")
            .arg("-p")
            .arg("test_pass")
            .arg("--color")
            .arg("never")
            .output()
            .expect("Failed to execute cargo expand");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("/*ERROR*/"),
            "Generated code contains invalid syntax"
        );
        insta::assert_snapshot!(stdout);
    }
}
