mod pagination {
    use serialization::Serializable;

    struct Page {
        size: u32,
        content: &[u8],
        next: u32
    }
}