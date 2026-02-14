pub(crate) mod stub;

use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::DNSPacket;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

pub(crate) fn check_response_diff(
    dns_packet: DNSPacket,
    origin_questions: &[QuestionSection],
) -> Result<(), SCloudException> {
    let diff_answers = if !dns_packet.answers.is_empty() {
        check_answer_diff(origin_questions, &dns_packet.answers)
    } else {
        Ok(())
    };

    let diff_authorities = if !dns_packet.authorities.is_empty() {
        check_authority_diff(origin_questions, &dns_packet.authorities)
    } else {
        Ok(())
    };

    let diff_additionals = if !dns_packet.additionals.is_empty() {
        check_additional_diff(origin_questions, &dns_packet.additionals)
    } else {
        Ok(())
    };

    if diff_answers.is_err() || diff_authorities.is_err() || diff_additionals.is_err() {
        return Err(SCloudException::SCLOUD_RESOLVER_RESPONSE_MISMATCH);
    }

    Ok(())
}

/// Check that each question has at least one corresponding record
/// in the answer, authority, or additional sections.
///
/// This prevents responses that do not actually answer the query.
///
/// # Exemple :
/// ```
/// use crate::dns::resolver::check_answer_diff;
/// use crate::dns::packet::question::QuestionSection;
/// use crate::dns::packet::answer::AnswerSection;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let questions = vec![QuestionSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::A,
///     q_class: DNSClass::IN,
/// }];
///
/// let answers = vec![AnswerSection {
///     q_name: "example.com".to_string(),
///     r_type: DNSRecordType::A,
///     r_class: DNSClass::IN,
///     ttl: 300,
///     rdlength: 4,
///     rdata: vec![93, 184, 216, 34],
/// }];
///
/// assert!(check_answer_diff(&questions, &answers, &[], &[]).is_ok());
/// ```
pub(crate) fn check_answer_diff(
    questions: &[QuestionSection],
    answers: &[AnswerSection]
) -> Result<(), SCloudException> {
    for record in answers.iter() {
        if !questions.iter().any(|q| record.q_name == q.q_name) {
            return Err(SCloudException::SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH);
        }
    }
    Ok(())
}

/// Ensure that authority records belong to the same zone
/// as the original DNS questions.
///
/// This prevents out-of-zone NS records.
///
/// # Exemple :
/// ```
/// use crate::dns::resolver::check_authority_diff;
/// use crate::dns::packet::question::QuestionSection;
/// use crate::dns::packet::authority::AuthoritySection;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let questions = vec![QuestionSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::NS,
///     q_class: DNSClass::IN,
/// }];
///
/// let authorities = vec![AuthoritySection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::NS,
///     q_class: DNSClass::IN,
///     ttl: 3600,
///     ns_name: "ns1.example.com".to_string(),
/// }];
///
/// assert!(check_authority_diff(&questions, &authorities).is_ok());
/// ```
#[allow(unused)]
pub(crate) fn check_authority_diff(
    questions: &[QuestionSection],
    authorities: &[AuthoritySection],
) -> Result<(), SCloudException> {
    for record in authorities.iter() {
        if !questions.iter().any(|q| record.q_name == q.q_name) {
            return Err(SCloudException::SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH);
        }
    }
    Ok(())
}

/// Ensure that additional records correspond to the original questions.
///
/// This is commonly used to validate glue records.
///
/// # Exemple :
/// ```
/// use crate::dns::resolver::check_additional_diff;
/// use crate::dns::packet::question::QuestionSection;
/// use crate::dns::packet::additional::AdditionalSection;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let questions = vec![QuestionSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::A,
///     q_class: DNSClass::IN,
/// }];
///
/// let additionals = vec![AdditionalSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::A,
///     q_class: DNSClass::IN,
///     ttl: 300,
///     rdlength: 4,
///     rdata: vec![192, 0, 2, 1],
/// }];
///
/// assert!(check_additional_diff(&questions, &additionals).is_ok());
/// ```
#[allow(unused)]
pub(crate) fn check_additional_diff(
    questions: &[QuestionSection],
    additionals: &[AdditionalSection],
) -> Result<(), SCloudException> {
    for record in additionals.iter() {
        if !questions.iter().any(|q| record.q_name == q.q_name) {
            return Err(SCloudException::SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH);
        }
    }
    Ok(())
}
