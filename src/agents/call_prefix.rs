use serde::Deserialize;

pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum CallPrefix {
        One(String),
        Many(Vec<String>),
    }

    let prefix = Option::<CallPrefix>::deserialize(deserializer)?
        .map(|prefix| match prefix {
            CallPrefix::One(token) => vec![token],
            CallPrefix::Many(tokens) => tokens,
        })
        .unwrap_or_default();
    if prefix.iter().any(|token| token.is_empty()) {
        return Err(serde::de::Error::custom(
            "`call_prefix` cannot contain an empty argument",
        ));
    }
    Ok(prefix)
}
