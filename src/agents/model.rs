use serde::Deserialize;

pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    Ok(Option::<String>::deserialize(deserializer)?.map(|model| model.trim().to_lowercase()))
}
