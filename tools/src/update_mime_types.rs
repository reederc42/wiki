use std::{collections::HashMap, fs::File, io::{BufWriter, Write}};

use regex::Regex;

const DEFAULT_OUTPUT: &str = "wiki/mime_types.yaml";
const DEFAULT_SOURCE_URL: &str = 
    "https://raw.githubusercontent.com/nginx/nginx/master/conf/mime.types";

const TYPES_RE: &str = r"(?s)\s*types \{(.*)\}";
const LINE_RE: &str = r"(?m)^\s*(\S+)[\s]+(.*)$";

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Output file for MIME types
    #[arg(short, long, default_value = DEFAULT_OUTPUT)]
    output: String,

    /// Source URL from NGINX
    #[arg(
        short,
        long,
        default_value = DEFAULT_SOURCE_URL,
    )]
    source: String,
}

pub async fn cmd(args: Args) {
    let mut f = File::create(args.output).unwrap();

    let body = reqwest::get(args.source)
        .await.unwrap()
        .text()
        .await.unwrap();

    let mut m = nginx_to_map(&body);

    // map is not included so it must be added manually
    m.insert("map", "application/json");

    let s = serde_yaml::to_string(&m).unwrap();
    let mut lines: Vec<&str> = s
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .collect();
    lines.sort();

    f.write(lines.join("\n").as_bytes()).unwrap();
    f.write(b"\n").unwrap();
}

fn nginx_to_map(nginx_mime_types: &str) -> HashMap<&str, &str> {
    let types_re = Regex::new(TYPES_RE).unwrap();
    let line_re = Regex::new(LINE_RE).unwrap();

    let (_, [types]) = types_re
        .captures(nginx_mime_types)
        .map(|m| m.extract())
        .unwrap();

    let mut m = HashMap::new();
    types.split(';').for_each(|line| {
        if let Some(t) = line_to_tuples(line, &line_re) {
            t.into_iter().for_each(|e| {
                m.insert(e.0, e.1);
            });
        }
    });

    m
}

fn line_to_tuples<'a>(
    line: &'a str,
    re: &Regex,
) -> Option<Vec<(&'a str, &'a str)>> {
    let (_, [mime_type, extensions]) = re.captures(line).map(|m| m.extract())?;
    Some(extensions.split(' ').map(|ext| (ext, mime_type)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nginx_to_json() {
        let mime_types = r"
        types {
            application/json    json;
            text/html       html htm;
            text/plain
                        txt;
        }
        ";

        let expected = HashMap::from([
            ("json", "application/json"),
            ("html", "text/html"),
            ("htm", "text/html"),
            ("txt", "text/plain"),
        ]);

        let actual = nginx_to_map(mime_types);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_to_tuple() {
        let re = Regex::new(LINE_RE).unwrap();
        let line = "    application/json    json";
        let expected = vec![("json", "application/json")];

        let actual = line_to_tuples(line, &re).unwrap();

        assert_eq!(actual, expected);

        let line = "text/html   html htm";
        let expected = vec![
            ("html", "text/html"),
            ("htm", "text/html"),
        ];

        let actual = line_to_tuples(line, &re).unwrap();

        assert_eq!(actual, expected);

        let line = r"text/plain
            txt";
        let expected = vec![
            ("txt", "text/plain"),
        ];

        let actual = line_to_tuples(line, &re).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_re() {
        let re = Regex::new(LINE_RE).unwrap();

        let line = "  \t abc def hij";
        let expected_mime_type = "abc";
        let expected_extensions = "def hij";

        let (_, [mime_type, extensions]) =
            re.captures(line).map(|m| m.extract()).unwrap();

        assert_eq!(mime_type, expected_mime_type);
        assert_eq!(extensions, expected_extensions);
    }

    #[test]
    fn test_types_re() {
        let re = Regex::new(TYPES_RE).unwrap();

        let types_response = r"
        types {
            This is my stuff
        }";

        let (_, [types]) =
            re.captures(types_response).map(|m| m.extract()).unwrap();

        assert!(types.contains("This is my stuff"));
        assert!(!types.contains('}'));
    }
}
