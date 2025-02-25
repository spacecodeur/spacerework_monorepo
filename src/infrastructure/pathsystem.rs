pub fn get_segment_data(path: &str, trainer_id: i32) -> Result<Vec<&'static str>, &'static str> {
    Ok(vec!["coursphp.md", "tata"])
}

// pub fn is_dir_db_check => retourne id dir ou lesson

// et ensuite on check les results du dessus : en fonction de, on recherche dans la db les info du dir ou de la lesson
