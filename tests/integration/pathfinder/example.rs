use app::domain::pathsystem::is_dir_semantic_check;

#[test]
fn example3() {
    let result = is_dir_semantic_check("PLOP/PLOP");
    assert_eq!(result, Ok(true));
}
