use ggez::event::Button;

pub fn serialize<S>(button: &Button, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match button {
        Button::Start => ser.serialize_str("Start"),
        Button::DPadRight => ser.serialize_str("DPadRight"),
        Button::DPadLeft => ser.serialize_str("DPadLeft"),
        _ => Err(serde::ser::Error::custom("unknown button pressed")),
    }
}

pub fn deserialize<'de, D>(deser: D) -> Result<Button, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;
    let button_string = <String>::deserialize(deser)?;
    match button_string.as_str() {
        "Start" => Ok(Button::Start),
        "DPadRight" => Ok(Button::DPadRight),
        "DPadLeft" => Ok(Button::DPadLeft),
        _ => Err(serde::de::Error::custom("Unknown button")),
    }
}
