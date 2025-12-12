
#[cfg(test)]
mod tests{
    use strum::IntoEnumIterator;
    use crate::exceptions::SCloudException;
    use crate::exceptions::SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED;

    #[test]
    fn test_exceptions_to_str(){
        let mut i: usize = 0;
        let ex_msg_array: [&'static str; 12] = [
            "Buffer length is less than header length.",
            "The header is empty.",
            "Buffer length is less than question section length.",
            "`q_name` too long.",
            "Buffer length is less than answer section length.",
            "Buffer length is less than authority section length.",
            "Buffer length is less than additional section length.",
            "Impossible to parse the `q_name`, check if a `q_name` is provided.",
            "Impossible to parse the `q_name`, pos is greater than buffer length.",
            "Impossible to parse the `q_name`, pos and len are greater than buffer length.",
            "Impossible to parse the `q_name`, compression 0xC0xx failed.",
            "Unknown `q_type`."
        ];
        for ex in SCloudException::iter() {
            println!("expected: {:?}\ngot: {}", ex_msg_array[i], SCloudException::to_str(&ex));
            assert_eq!(SCloudException::to_str(&ex), ex_msg_array[i]);
            i+=1;
        }
    }

}