pub struct DeserializationError
    {
        error: String
    }

    pub trait Serializable {
        fn serialize(&self) -> &[u8];
        fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized;
    }

    impl Serializable for String {
        fn serialize(&self) -> &[u8] {
            self.as_bytes()
        }

        fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError>
        {
            match String::from_utf8(bytes.to_vec()) {
                Ok(value) => Result::Ok(value),
                _ => Result::Err(DeserializationError {
                    error: String::from("Error to deserialize string!")
                })
            }
        }
    }