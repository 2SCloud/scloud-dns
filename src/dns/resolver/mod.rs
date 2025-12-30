use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

pub(crate) mod stub;

pub(crate) fn check_answer_diff(
    questions: &[QuestionSection],
    answers: &[AnswerSection],
) -> Result<bool, SCloudException> {
    let mut found = false;
    for question in questions {
        for answer in answers {
            if answer.q_name == question.q_name
                && answer.r_type == question.q_type
                && answer.r_class == question.q_class
            {
                found = true;
                break;
            }
        }

        if !found {
            return Err(SCloudException::SCLOUD_STUB_RESOLVER_ANSWER_MISMATCH);
        }
    }
    Ok(found)
}
