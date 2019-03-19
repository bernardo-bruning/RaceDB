mod serialization;
mod pagination;

#[cfg(test)]
mod tests {
    use crate::serialization::Serializable;
    use crate::pagination::Page;
    use std::fmt::Debug;

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

    #[test]
    fn test_serialization_and_deserialization_page() {
        let page = Page::from("teste");
        let serialized = page.serialize();
        let deserialized = Page::deserialize(&serialized);
        let page_deserialized = deserialized.unwrap();
        assert_eq!(page_deserialized.size, page.size);
        assert_eq!(page_deserialized.next, page.next);
        assert_eq!(page_deserialized.content, page.content);
    }
}
