impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 空字符串
        if s.trim().is_empty() {
            return Person::default();
        }

        let parts: Vec<&str> = s.split(',').collect();

        // 必须正好两个部分（不允许多余逗号或字段）
        if parts.len() != 2 {
            return Person::default();
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Person::default();
        }

        let age_str = parts[1].trim();
        let age = match age_str.parse::<usize>() {
            Ok(age) => age,
            Err(_) => return Person::default(),
        };

        Person {
            name: name.to_string(),
            age,
        }
    }
}