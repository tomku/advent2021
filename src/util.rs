pub fn puzzle_input(day: &str) -> String {
    let filename = format!("files/day{}.txt", day);
    let path = std::path::Path::new(&filename);
    std::fs::read_to_string(&path).map_err(|e| {
        panic!("Error opening {}: {}", path.display(), e)
    }).unwrap()
}
