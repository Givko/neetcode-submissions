impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = "".to_owned();
        for str in strs {
            encoded += &str.len().to_string();
            encoded += "#";
            encoded += &str;
        }

        return encoded;
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded = vec![];
        let mut i = 0;
        let mut len = "".to_owned();
        while i < s.len() {
            let ch = s.chars().nth(i).unwrap();
            if ch == '#' {
                i += 1;
                let int_len: usize = len.parse().unwrap();
                let string = &s[i..i + int_len];
                decoded.push(string.to_owned());
                i += int_len;
                len = "".to_owned();
            } else {
                len += &ch.to_string();
                i += 1;
            }
        }
        return decoded;
    }
}