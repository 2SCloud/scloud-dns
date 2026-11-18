use crate::exceptions::SCloudException;

/// Maximum length of a domain name on the wire, length octets and
/// terminating zero included (RFC 1035, section 2.3.4).
const MAX_NAME_WIRE_LEN: usize = 255;

/// Parse a DNS QNAME from a DNS message buffer.
///
/// This function supports:
/// - Standard DNS label encoding
/// - Name compression using pointers (RFC 1035, section 4.1.4)
///
/// # Arguments
/// * `buf` - Full DNS message buffer
/// * `pos` - Offset in the buffer where the QNAME starts
///
/// # Returns
/// * `(String, usize)`
///   - Parsed domain name (e.g. "www.example.com")
///   - Number of bytes consumed at the original position
///
/// # Errors
/// Returns an error if:
/// - The buffer is too short
/// - A label length is invalid (reserved label types 0x40 and 0x80)
/// - The name is longer than 255 bytes on the wire
/// - Compression pointers are malformed, point forward, or form a loop
///
/// # Notes
/// - When compression is used, the returned `usize` corresponds
///   to the position right after the pointer (not the expanded name).
/// - The caller is responsible for passing the correct initial offset
///   (e.g. 12 for the first QNAME in a DNS packet).
/// - Pointers are followed iteratively. Each pointer must target an offset
///   strictly before the labels it terminates, so jump targets strictly
///   decrease and a crafted packet cannot make the parser loop forever
///   (e.g. a pointer to itself).
pub(crate) fn parse_qname(buf: &[u8], mut pos: usize) -> Result<(String, usize), SCloudException> {
    let mut labels = Vec::new();
    let mut jumped = false;
    let mut end_pos = pos;
    // Offset where the labels currently being read start.
    let mut segment_start = pos;
    // Wire length of the expanded name, terminating zero included.
    let mut name_len = 1;

    loop {
        if pos >= buf.len() {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF);
        }

        let len = buf[pos];

        // Compression 0xC0xx
        if len & 0xC0 == 0xC0 {
            if pos + 1 >= buf.len() {
                return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME);
            }

            let offset = (((len as u16 & 0x3F) << 8) | buf[pos + 1] as u16) as usize;

            // A pointer refers to a prior occurrence of a name (RFC 1035, section 4.1.4).
            if offset >= segment_start {
                return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED);
            }

            if !jumped {
                end_pos = pos + 2;
            }

            jumped = true;
            pos = offset;
            segment_start = offset;
            continue;
        }

        // 0x40 and 0x80 are reserved label types: a plain label is at most 63 bytes.
        if len & 0xC0 != 0 {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME);
        }

        if len == 0 {
            if !jumped {
                end_pos = pos + 1;
            }
            break;
        }

        name_len += len as usize + 1;
        if name_len > MAX_NAME_WIRE_LEN {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME);
        }

        pos += 1;

        if pos + len as usize > buf.len() {
            return Err(
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF,
            );
        }

        let label = &buf[pos..pos + len as usize];
        let s = String::from_utf8(label.to_vec())
            .map_err(|_| SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME)?;

        labels.push(s);
        pos += len as usize;
    }

    Ok((labels.join("."), end_pos))
}

/// Parse a DNS QNAME at a specific offset and return only the name.
///
/// This is mainly used internally when resolving compression pointers.
#[allow(unused)]
pub(crate) fn parse_qname_at(buf: &[u8], offset: usize) -> Result<String, SCloudException> {
    let (name, _consumed) = parse_qname(&buf, offset)?;
    Ok(name)
}
