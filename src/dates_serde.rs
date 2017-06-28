impl serde::Serialize for Date<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        self.format("%Y-%m-%d").to_string()
    }
}

impl<'de> serde::Deserialize<'de> for Date<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Date<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a date string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Value, E>
                where E: serde::de::Error {
                Ok(DateTime::parse_from_str(value, "%Y-%m-%d")
                            .map_err(E::custom("Could not parse the date"))?
                            .date())
            }
        }

        // Deserialize the date from a string.
        deserializer.deserialize_str(Visitor)
    }
}
