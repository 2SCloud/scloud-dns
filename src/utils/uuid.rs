use uuid::Uuid;

pub(crate) fn with_random_id(path: &str) -> String {
    let id = Uuid::new_v4();

    if let Some((base, ext)) = path.rsplit_once('.') {
        format!("{}-{}.{}", base, id, ext)
    } else {
        format!("{}-{}", path, id)
    }
}

pub(crate) fn generate_uuid() -> &'static str {
    Box::leak(Uuid::new_v4().to_string().into_boxed_str())
}
