use ggez::graphics::Color;

pub fn serialize<S>(color: &Color, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq as _;
    let mut seq = ser.serialize_seq(Some(3))?;
    let (red, green, blue) = &color.to_rgb();
    seq.serialize_element(&red)?;
    seq.serialize_element(&green)?;
    seq.serialize_element(&blue)?;
    seq.end()
}

pub fn deserialize<'de, D>(deser: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;
    let [r, g, b] = <[u8; 3]>::deserialize(deser)?;
    Ok(Color::from_rgb(r, g, b))
}
