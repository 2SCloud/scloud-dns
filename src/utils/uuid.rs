use uuid::Uuid;

fn with_random_id(path: &str) -> String {
    let id = Uuid::new_v4();

    if let Some((base, ext)) = path.rsplit_once('.') {
        format!("{}-{}.{}", base, id, ext)
    } else {
        format!("{}-{}", path, id)
    }
}
