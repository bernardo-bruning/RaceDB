mod serialization;
mod pagination;
mod storage;

#[cfg(test)]
mod tests {
    use crate::serialization::Serializable;
    use crate::pagination::Page;
    use crate::pagination::paginate;
    use crate::pagination::fill;
    use crate::pagination::mount_data;
    use crate::storage::allocate;
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
        let pages = paginate("test".to_string(), 5);
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
        let pages = paginate(string, 3);
        assert_eq!(pages.len(), 4);
    }

    #[test]
    fn test_values_from_page() {
        let string = "this is a test".to_string();
        let pages = paginate(string, 7);
        let first_page: &Page = pages.first().unwrap();
        let last_page: &Page = pages.last().unwrap();
        
        assert_eq!(String::from_utf8_lossy(&first_page.content), "this is");
        assert_eq!(String::from_utf8_lossy(&last_page.content), " a test");
    }

    #[test]
    fn test_create_paginate_full() {
        let data = "this is a test!".to_string();
        let pages = paginate(data, 15);
        let page = pages.first().unwrap();
        assert_eq!(page.content.len(), 15);
    }

    #[test]
    fn test_create_paginate_partial_content() {
        let data = "this is a test".to_string();
        let pages = paginate(data, 15);
        let page = pages.first().unwrap();
        assert_eq!(page.content.len(), 15);
    }

    #[test]
    fn test_mount_data_from_pages() {
        let pages = [
            // Page with string "tes"
            Page {
                id: 0,
                next: 1,
                size: 2,
                content: [116, 101, 115].to_vec() 
            },
            // Page with string "te"
            Page {
                id: 1,
                next: 0,
                size: 2,
                content: [116, 101].to_vec() 
            }
        ];

        let text = mount_data::<String>(&pages).unwrap();
        assert_eq!(text, "teste".to_string());
    }

    #[test]
    fn test_allocator_pages() {
        let string = "this is a test".to_string();
        let pages = paginate(string, 2);
        let mut cursor = Cursor::new(Vec::new() as Vec<u8>);
        let index_file = allocate(&mut cursor, pages);
        assert_eq!(index_file.unwrap(), 0);
        assert_eq!(cursor.seek(SeekFrom::End(0)).unwrap(), 70);
    }

    #[test]
    fn test_fill_vec() {
        let value: Vec<u8> = Vec::new();
        let value_filled = fill(&value, 0, 3);
        
        assert_eq!(value_filled.capacity(), 3);
        assert_eq!(value_filled, vec![0, 0, 0])
    }
}
