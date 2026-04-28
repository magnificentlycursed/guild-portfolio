pub fn validate_title(raw: &str) -> Result<String, String> {
    todo!()
}

pub fn next_id(existing_ids: &[u64]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_empty_after_trim_is_rejected() {
        assert!(validate_title("").is_err());
        assert!(validate_title("   ").is_err());
    }

    #[test]
    fn title_trimmed_before_storage() {
        assert_eq!(validate_title("  Fix bug  ").unwrap(), "Fix bug");
    }

    #[test]
    fn id_assignment_first_issue_is_1() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn id_assignment_increments_from_max() {
        assert_eq!(next_id(&[1, 3, 5]), 6);
    }
}
