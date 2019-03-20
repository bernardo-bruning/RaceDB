mod serialization;
mod pagination;

#[cfg(test)]
mod tests {
    use crate::serialization::Serializable;
    use crate::pagination::Page;
    use crate::pagination::paginate;

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

    #[test]
    fn test_len_paginate_from_simple_string() {
        let string = "string test".to_string();
        let pages = paginate(string, 3);
        assert_eq!(pages.len(), 4);
    }
}
