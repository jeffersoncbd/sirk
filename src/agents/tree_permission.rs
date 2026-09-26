use serde::Deserialize;

pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    if value == "allow" {
        Ok(true)
    } else {
        Err(serde::de::Error::custom("`TREE_TOOL` must be `allow`"))
    }
}
