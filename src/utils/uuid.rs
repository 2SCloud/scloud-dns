use uuid::Uuid;

pub(crate) fn with_random_id(path: &str) -> String {
    let id = Uuid::new_v4();

    if let Some((base, ext)) = path.rsplit_once('.') {
        format!("{}-{}.{}", base, id, ext)
    } else {
        format!("{}-{}", path, id)
    }
}

pub(crate) fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

pub(crate) fn uuid_as_static_str(uuid: Uuid) -> &'static str {
    Box::leak(uuid.to_string().into_boxed_str())
}
