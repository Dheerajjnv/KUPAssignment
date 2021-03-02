pub fn is_rotation(str1: String, str2: String) -> bool {
    let  len1 = str1.len();
    let  len2 = str2.len();
    let  string1: Vec<char> = str1.chars().collect();
    let  string2: Vec<char> = str2.chars().collect();
    if len1 != len2 {
        return false;
    }
    let mut longest_prefix_suffix = Vec::with_capacity(len1);

    let mut prev_len = 0;
    let mut i = 1;


    longest_prefix_suffix[0] = 0;


    while i < len1 {
        if string1[i] == string2[prev_len] {
            longest_prefix_suffix[i] = prev_len + 1;
            i = i + 1;
        } else {
            if prev_len == 0 {
                longest_prefix_suffix[i] = 0;
                i = i + 1;
            } else {
                prev_len = longest_prefix_suffix[prev_len - 1];
            }
        }

    }

    i = 0;


    let mut k = longest_prefix_suffix[len1 - 1];
    while k < len2 {
        if string2[k] != string1[i] {
            i = i + 1;
            return false;
        }
        k = k + 1;
    }
    return true;
}