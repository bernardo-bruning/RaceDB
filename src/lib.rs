mod serialization;
mod pagination;

#[cfg(test)]
mod tests {
    pub use crate::serialization::Serializable;

    #[test]
    fn test_serialization_and_deserialization_string() {
        let default_value = "teste";
        let value = String::from(default_value);
        let serialized = value.serialize();
        let result = String::deserialize(&serialized);

        match result {
            Ok(value_result) => assert_eq!(value, value_result),
            Err(_) => panic!("Error to serialize")
        };
    }

    #[test]
    fn test_deserialization_corrupt_string() {
        let result = String::deserialize(&[132]);

        match result {
            Ok(_) => panic!("Not detected corrupt string"),
            _ => ()
        };
    }
}
