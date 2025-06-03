pub mod u8_as_usize
{
    //! Because the F1 game library uses u8 for indices, we need to convert them to usize
    //! so they can be used to index arrays (car arrays, participant arrays, etc.).

    use serde::{
        Deserialize,
        Deserializer,
        Serializer,
    };

    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: u8 = Deserialize::deserialize(deserializer)?;
        Ok(value as usize)
    }

    pub fn serialize<S>(value: &usize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // For the purposes of this deserializer, we can safely convert the usize to a u8
        serializer.serialize_u8(*value as u8)
    }

    #[cfg(test)]
    mod test
    {
        use serde::{
            Deserialize,
            Serialize,
        };

        use crate::u8_as_usize;

        #[derive(Deserialize, Serialize)]
        struct TestStruct
        {
            #[serde(with = "u8_as_usize")]
            value: usize,
        }

        #[test]
        fn test_u8_as_usize()
        {
            let value: u8 = 10;
            let serialized = bincode::serialize(&value).unwrap();

            let deserialized: TestStruct = bincode::deserialize(&serialized).unwrap();
            assert_eq!(10, deserialized.value);
        }
    }
}
