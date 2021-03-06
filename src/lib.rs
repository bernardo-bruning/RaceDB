mod serialization;
mod pagination;
mod storage;

#[cfg(test)]
mod tests {
    use crate::serialization::*;
    use crate::pagination::*;
    use crate::storage::*;
    use std::io::Cursor;
    use std::io::Seek;
    use std::io::SeekFrom;

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
        let pages = Pages::from(&"test".to_string(), 5);
        let page = pages.first().unwrap();
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
        let pages = Pages::from(&string, 3);
        assert_eq!(pages.len(), 4);
    }

    #[test]
    fn test_values_from_page() {
        let string = "this is a test".to_string();
        let pages = Pages::from(&string, 7);
        let first_page: &Page = pages.first().unwrap();
        let last_page: &Page = pages.last().unwrap();
        
        assert_eq!(String::from_utf8_lossy(&first_page.content), "this is");
        assert_eq!(String::from_utf8_lossy(&last_page.content), " a test");
    }

    #[test]
    fn test_create_paginate_full() {
        let data = "this is a test!".to_string();
        let pages = Pages::from(&data, 15);
        let page = pages.first().unwrap();
        assert_eq!(page.content.len(), 15);
    }

    #[test]
    fn test_create_paginate_partial_content() {
        let data = "this is a test".to_string();
        let pages = Pages::from(&data, 15);
        let page = pages.first().unwrap();
        assert_eq!(page.content.len(), 15);
    }

    #[test]
    fn test_mount_data_from_pages() {
        let pages = [
            // Page with string "tes"
            Page {
                id: 0,
                next: Option::Some(1),
                size: 2,
                content: "tes".as_bytes().to_vec()
            },
            // Page with string "te"
            Page {
                id: 1,
                next: Option::None,
                size: 2,
                content: "te".as_bytes().to_vec()
            }
        ];

        let text = mount_data::<String>(&pages).unwrap();
        assert_eq!(text, "teste".to_string());
    }

    #[test]
    fn test_allocator_pages() {
        let string = "this is a test".to_string();
        let mut pages = Pages::from(&string, 2);
        let mut cursor = Cursor::new(Vec::new() as Vec<u8>);
        pages = pages.store(&mut cursor).unwrap();
        assert_eq!(cursor.seek(SeekFrom::End(0)).unwrap(), pages.get_byte_size() as u64);
    }

    #[test]
    fn test_allocator_pages_with_id() {
        let string = "this is a test".to_string();
        let mut pages = Pages::from(&string, 2);
        let mut cursor = Cursor::new(Vec::new() as Vec<u8>);
        pages = pages.store(&mut cursor).unwrap();
        let mut pages_iter = pages.iter();

        assert_eq!(pages_iter.next().unwrap().id, Page::get_bytes_size(2) * 0);
        assert_eq!(pages_iter.next().unwrap().id, Page::get_bytes_size(2) * 1);
        assert_eq!(pages_iter.next().unwrap().id, Page::get_bytes_size(2) * 2);
    }

    #[test]
    fn test_fill_vec() {
        let value: Vec<u8> = Vec::new();
        let value_filled = fill(&value, 0, 3);
        
        assert_eq!(value_filled.capacity(), 3);
        assert_eq!(value_filled, vec![0, 0, 0])
    }
}
