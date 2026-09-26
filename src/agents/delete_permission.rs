use serde::Deserialize;

pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    if value == "allow" {
        Ok(true)
    } else {
        Err(serde::de::Error::custom(
            "`DELETE_TOOL` and `DELETE_WITHOUT_CONFIRM` must be `allow`",
        ))
    }
}
