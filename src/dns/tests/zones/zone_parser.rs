#[cfg(test)]
mod tests {
    use crate::dns;
    use crate::dns::zones::Zone;
    use crate::dns::zones::zone_parser::zone_parser;

    #[test]
    fn test_zone_parser() {
        let expected_zone: Zone = Zone {
            origin: Some("onihilist.moe".to_string()),
            name: "".to_string(),
            ttl: 0,
            soa: None,
            records: Default::default(),
        };

        let result = zone_parser("onihilist.moe");

        //assert_eq!(result.is_ok(), true);
        assert_eq!(expected_zone, result.unwrap());
    }
}
