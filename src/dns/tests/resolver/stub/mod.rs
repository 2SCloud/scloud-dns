
#[cfg(test)]
mod tests{
    use std::net::SocketAddr;
    use crate::dns::packet::authority::AuthoritySection;
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::q_class::DNSClass;
    use crate::dns::q_type::DNSRecordType;
    use crate::dns::resolver::stub::StubResolver;

    #[test]
    fn test_new_stub_resolver() {
        let result = StubResolver::new("192.0.0.245:53".parse().unwrap())
            .resolve(vec![QuestionSection {
                q_name: "github.com".to_string(),
                q_type: DNSRecordType::CNAME,
                q_class: DNSClass::IN,
            }]).unwrap();

        let expected_packet: DNSPacket = DNSPacket{
            header: Header {
                id: result.header.id,
                qr: true,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: true,
                z: 0,
                rcode: 0,
                qdcount: 1,
                ancount: 0,
                nscount: 1,
                arcount: 0,
            },
            questions: vec![QuestionSection {
                q_name: "github.com".to_string(),
                q_type: DNSRecordType::CNAME,
                q_class: DNSClass::IN,
            }],
            answers: vec![],
            authorities: vec![AuthoritySection{
                q_name: "github.com".to_string(),
                q_type: DNSRecordType::SOA,
                q_class: DNSClass::IN,
                ttl: result.authorities[0].ttl,
                ns_name: "dns1.p08.nsone.net".to_string(),
            }],
            additionals: vec![],
        };

        println!("expected: {:?}\ngot: {:?}", expected_packet, result);
        assert_eq!(expected_packet, result)
    }

}