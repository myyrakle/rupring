use crate::request::MultipartFile;

/*
Content-Type: multipart/form-data에 붙어있는 bounary 값을 추출합니다.
*/
pub fn parse_multipart_boundary(header_value: &str) -> Option<String> {
    header_value
        .split(";")
        .find(|s| s.contains("boundary="))
        .map(|s| s.split("boundary=").last())
        .flatten()
        .map(|s| s.trim().to_string())
}

/*
멀티파트 bytes를 파싱해서 파일 형태로 변환합니다.
*/
pub fn parse_multipart(raw_body: &[u8], boundary: &str) -> anyhow::Result<Vec<MultipartFile>> {
    let mut files = vec![];

    let start_boundary = format!("\r\n--{boundary}");

    let mut i = 0;
    loop {
        if i >= raw_body.len() {
            break;
        }

        // 첫번째 바운더리 삼키기
        if i == 0 {
            i += boundary.len() + 2;
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

            if raw_body[i] == b'\r' && raw_body[i + 1] == b'\n' {
                let mut header_end = false;

                if i + 3 < raw_body.len() && (raw_body[i + 2] == b'\r' && raw_body[i + 3] == b'\n')
                {
                    header_end = true;
                }

                let line = String::from_utf8_lossy(&raw_body[start_index..i]);

                if line.starts_with("Content-Disposition") {
                    let parts = line.split(";").map(|s| s.trim());
                    for part in parts {
                        if part.starts_with("name=") {
                            name = part
                                .split('=')
                                .next_back()
                                .unwrap_or_default()
                                .trim_matches('"')
                                .to_owned();
                        } else if part.starts_with("filename=") {
                            filename = part
                                .split('=')
                                .next_back()
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
                    i += 4;
                    break;
                } else {
                    i += 2;
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
                data = raw_body[start_index..i].to_vec();
                i += start_boundary.len() + 1;

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
            content_type,
            data,
        });

        i += 1;

        // 개행 삼키기
        if raw_body.len() > i && raw_body[i] == b'\r' {
            i += 1;
        }
        if raw_body.len() > i && raw_body[i] == b'\n' {
            i += 1;
        }
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
                    raw_body: concat!(
                        "------WebKitFormBoundarywegos5eij6KIxFTB\r\n",
                        "Content-Disposition: form-data; name=\"file1\"; filename=\"6p63l2zrde5zhibddhbfmemrke.json\"\r\n",
                        "Content-Type: application/json\r\n",
                        "\r\n",
                        "{\"Item\":{\"id\":{\"S\":\"1\"},\"value\":{\"S\":\"foo\"}}}\n",
                        "{\"Item\":{\"id\":{\"S\":\"6\"},\"value\":{\"S\":\"rust\"}}}\n",
                        "\r\n",
                        "------WebKitFormBoundarywegos5eij6KIxFTB\r\n",
                        "Content-Disposition: form-data; name=\"file2\"; filename=\"asdfasdf.json\"\r\n",
                        "Content-Type: application/json\r\n",
                        "\r\n",
                        "{\"Item\":{\"id\":{\"S\":\"4444\"},\"value\":{\"S\":\"bar\"}}}\n",
                        "{\"Item\":{\"id\":\"3434\",\"value\":\"go\"}}}\n",
                        "\r\n",
                        "------WebKitFormBoundarywegos5eij6KIxFTB--\r\n"
                    ).as_bytes(),
                    boundary: "----WebKitFormBoundarywegos5eij6KIxFTB",
                    expected: vec![
                        MultipartFile {
                            name: "file1".to_string(),
                            filename: "6p63l2zrde5zhibddhbfmemrke.json".to_string(),
                            content_type: "application/json".to_string(),
                            data: b"{\"Item\":{\"id\":{\"S\":\"1\"},\"value\":{\"S\":\"foo\"}}}\n{\"Item\":{\"id\":{\"S\":\"6\"},\"value\":{\"S\":\"rust\"}}}\n".to_vec(),
                        },
                        MultipartFile {
                            name: "file2".to_string(),
                            filename: "asdfasdf.json".to_string(),
                            content_type: "application/json".to_string(),
                            data: b"{\"Item\":{\"id\":{\"S\":\"4444\"},\"value\":{\"S\":\"bar\"}}}\n{\"Item\":{\"id\":\"3434\",\"value\":\"go\"}}}\n".to_vec(),
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

    #[test]
    fn test_parse_multipart_boundary() {
        #[derive(Debug, PartialEq)]
        struct TestCase {
            name: String,
            header_value: &'static str,
            expected: Option<String>,
        }

        let test_cases = vec![
            TestCase {
                name: "boundary가 있을 때".into(),
                header_value:
                    "multipart/form-data; boundary=----WebKitFormBoundarywegos5eij6KIxFTB",
                expected: Some("----WebKitFormBoundarywegos5eij6KIxFTB".to_string()),
            },
            TestCase {
                name: "boundary가 없을 때".into(),
                header_value: "multipart/form-data",
                expected: None,
            },
        ];

        for tc in test_cases {
            let expected = tc.expected;

            let result = parse_multipart_boundary(tc.header_value);

            assert_eq!(result, expected, "test case: {:?}", tc.name);
        }
    }
}
