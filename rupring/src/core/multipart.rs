use crate::request::MultipartFile;

pub fn parse_multipart(raw_body: &[u8], boundary: &str) -> anyhow::Result<Vec<MultipartFile>> {
    let mut files = vec![];

    let start_boundary = format!("--{boundary}");

    let mut i = 0;
    loop {
        if i >= raw_body.len() {
            break;
        }

        // 첫번째 바운더리 삼키기
        if i == 0 {
            i += start_boundary.len() + 1;
        }

        let mut name = "".to_string();
        let mut filename = "".to_string();
        let mut content_type = "".to_string();

        // 개행이 연달아 2개 나올때까지 Header 절 파싱
        let mut start_index = i;
        loop {
            if i + 1 >= raw_body.len() {
                break;
            }

            if raw_body[i] == b'\n' {
                let line = String::from_utf8_lossy(&raw_body[start_index..i]);

                let mut header_end = false;
                if i + 1 < raw_body.len() && raw_body[i + 1] == b'\n' {
                    header_end = true;
                }

                if line.starts_with("Content-Disposition") {
                    let parts = line.split(";").map(|s| s.trim());
                    for part in parts {
                        if part.starts_with("name=") {
                            name = part
                                .split('=')
                                .last()
                                .unwrap_or_default()
                                .trim_matches('"')
                                .to_owned();
                        } else if part.starts_with("filename=") {
                            filename = part
                                .split('=')
                                .last()
                                .unwrap_or_default()
                                .trim_matches('"')
                                .to_owned();
                        }
                    }
                } else if line.starts_with("Content-Type") {
                    content_type = line
                        .split(":")
                        .nth(1)
                        .map(|s| s.trim())
                        .unwrap_or_default()
                        .to_owned();
                }

                if header_end {
                    i += 2;
                    break;
                } else {
                    i += 1;
                    start_index = i;
                    continue;
                }
            }

            i += 1;
        }

        // bounary가 또 나올때까지 데이터 삼키기
        let mut data = vec![];
        let start_index = i;
        loop {
            if i + start_boundary.len() >= raw_body.len() {
                break;
            }

            if raw_body[i..].starts_with(start_boundary.as_bytes()) {
                data = raw_body[start_index..i - 1].to_vec();
                i += start_boundary.len();

                if raw_body[i..i + 2].starts_with(b"--") {
                    i += 2;
                }

                break;
            }

            i += 1;
        }

        files.push(MultipartFile {
            name: name.to_string(),
            filename: filename.to_string(),
            content_type: content_type,
            data,
        });

        i += 1;
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multipart() {
        #[derive(Debug, PartialEq)]
        struct TestCase {
            name: String,
            raw_body: &'static [u8],
            boundary: &'static str,
            expected: Vec<MultipartFile>,
            want_err: bool,
        }

        let test_cases = vec![TestCase {
            name: "json 파일 2개".into(),
            raw_body: br#"------WebKitFormBoundarywegos5eij6KIxFTB
Content-Disposition: form-data; name="file1"; filename="6p63l2zrde5zhibddhbfmemrke.json"
Content-Type: application/json

{"Item":{"id":{"S":"1"},"value":{"S":"foo"}}}
{"Item":{"id":{"S":"6"},"value":{"S":"rust"}}}

------WebKitFormBoundarywegos5eij6KIxFTB
Content-Disposition: form-data; name="file2"; filename="asdfasdf.json"
Content-Type: application/json

{"Item":{"id":{"S":"4444"},"value":{"S":"bar"}}}
{"Item":{"id":{"S":"3434"},"value":{"S":"go"}}}

------WebKitFormBoundarywegos5eij6KIxFTB--
"#,
            boundary: "----WebKitFormBoundarywegos5eij6KIxFTB",
            expected: vec![
                MultipartFile {
                    name: "file1".to_string(),
                    filename: "6p63l2zrde5zhibddhbfmemrke.json".to_string(),
                    content_type: "application/json".to_string(),
                    data: br#"{"Item":{"id":{"S":"1"},"value":{"S":"foo"}}}
{"Item":{"id":{"S":"6"},"value":{"S":"rust"}}}
"#
                    .to_vec(),
                },
                MultipartFile {
                    name: "file2".to_string(),
                    filename: "asdfasdf.json".to_string(),
                    content_type: "application/json".to_string(),
                    data: br#"{"Item":{"id":{"S":"4444"},"value":{"S":"bar"}}}
{"Item":{"id":{"S":"3434"},"value":{"S":"go"}}}
"#
                    .to_vec(),
                },
            ],
            want_err: false,
        }];

        for tc in test_cases {
            let expected = tc.expected;
            let want_err = tc.want_err;

            let result = parse_multipart(tc.raw_body, tc.boundary);

            if want_err {
                assert!(result.is_err());
            } else {
                let result = result.unwrap();

                assert_eq!(result, expected, "test case: {:?}", tc.name);
            }
        }
    }
}
