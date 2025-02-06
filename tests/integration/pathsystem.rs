use tuono_app::domain::pathsystem::is_dir;

#[test]
fn check_test() {
    let result = is_dir("PLOP/PLOP");
    assert_eq!(result, Ok(true));
}