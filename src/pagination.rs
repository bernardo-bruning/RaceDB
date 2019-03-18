mod pagination {
    use crate::serialization::Serializable;
    use crate::serialization::DeserializationError;
    use std::vec::Vec;

    fn convert_i32_to_u8(x: u32) -> [u8;4] {
        let b1 : u8 = ((x >> 24) & 0xff) as u8;
        let b2 : u8 = ((x >> 16) & 0xff) as u8;
        let b3 : u8 = ((x >> 8) & 0xff) as u8;
        let b4 : u8 = (x & 0xff) as u8;
        [b1, b2, b3, b4]
    }
    
    struct Page {
        size: u32,
        next: u32,
        content: Vec<u8>
    }

    impl Serializable for Page {
        fn serialize(&self) -> Vec<u8>
        {
            let size:Vec<u8> = convert_i32_to_u8(self.size).to_vec();
            let next:Vec<u8> = convert_i32_to_u8(self.next).to_vec();
            let binary = vec![size, next, self.content.clone()];
            binary.concat()
        }

        fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> where Self: Sized
        {
            Result::Ok(Page {
                size: 0,
                next: 0,
                content: Vec::new()
            })
        }
    }
} 