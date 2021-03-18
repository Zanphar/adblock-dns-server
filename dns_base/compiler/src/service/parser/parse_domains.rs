extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::common::parse;

fn extract(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<domain>.{2,200}\.[a-z]{2,6})").unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| cap.name("domain"))
        .and_then(|d| {
            let mut d_str = d.as_str().to_string();

            let d_s = d.as_str();
            if d_s.starts_with("*.") {
                d_str = d_str.replace("*.", "");
                return d_str
                    .parse::<DomainName>()
                    .ok()
                    .map(|v| "*.".to_string() + v.as_str());
            }

            d_str
                .parse::<DomainName>()
                .ok()
                .map(|v| v.as_str().trim().to_string())
        })
}

pub fn parse_domains(content: &str) -> Vec<String> {
    parse(content, extract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "abc.example.com";
        let output = extract(input);
        let expected = "abc.example.com".to_string();
        assert_eq!(output, Some(expected));

        let input = "Bücher.example.com";
        let output = extract(input);
        let expected = "xn--bcher-kva.example.com".to_string();
        assert_eq!(output, Some(expected));

        let input = "";
        let output = extract(input);
        assert_eq!(output, None);
    }

    #[test]
    fn it_works() {
        let input = "
            # This is a comment
            abc.example.com # this should work
            def.example.com\r

            ghi.example.com\r
        ";

        let output = parse_domains(input);

        let expected = vec![
            "abc.example.com".to_string(),
            "def.example.com".to_string(),
            "ghi.example.com".to_string(),
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn it_still_works() {
        let input = "
            Malvertising list by Disconnect
            # License: GPLv3
            # Contact: support [at] disconnect.me

            malware-check.disconnect.me
            101order.com
            123found.com
            140proof.com
            *.wpc.edgecastcdn.net
        ";

        let output = parse_domains(input);

        let expected = vec![
            "malware-check.disconnect.me".to_string(),
            "101order.com".to_string(),
            "123found.com".to_string(),
            "140proof.com".to_string(),
            "*.wpc.edgecastcdn.net".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
