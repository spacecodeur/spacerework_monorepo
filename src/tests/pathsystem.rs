use crate::domain::pathsystem::is_dir;

#[test]
fn check_if_dir() {
    let result = is_dir("toto/titi/tata");
    assert_eq!(result, Ok(true));
    let result = is_dir("cours");
    assert_eq!(result, Ok(true));
    let result = is_dir("");
    assert_eq!(result, Ok(true));
}

#[test]
fn check_if_file() {
    let result = is_dir("toto/titi/tata/cours.md");
    assert_eq!(result,Ok(false));

    let result = is_dir("lesson.md");
    assert_eq!(result,Ok(false));

}

#[test]
fn check_if_malformed() {
    let result = is_dir("toto/titi/cours.md/tata");
    assert_eq!(result, Err("malformed path"));
    let result = is_dir("toto/titi/cours.coucou/tata");
    assert_eq!(result, Err("malformed piece of path"));
    let result = is_dir("toto/titi/cours.coucou");
    assert_eq!(result, Err("malformed piece of path"));
    let result = is_dir("toto/titi/cou??rs/tata");
    assert_eq!(result, Err("malformed path"));
}