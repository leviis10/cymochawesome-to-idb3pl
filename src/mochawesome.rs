use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]
struct MochawesomeError {
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    uuid: String,
    title: String,
    state: String,
    duration: u64,
    pass: bool,
    fail: bool,
    err: MochawesomeError,
}

#[derive(Serialize, Deserialize, Debug)]
struct Suite {
    title: String,
    tests: Vec<Test>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MochawesomeResult {
    file: String,
    suites: Vec<Suite>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedMochawesome {
    results: Vec<MochawesomeResult>,
}

struct Tags {
    run_id: String,
    spec_file: String,
    suite: String,
    test_name: String,
    status: String,
}

impl Tags {
    fn new(run_id: &str, spec_file: &str, suite: &str, test_name: &str, status: &str) -> Tags {
        Tags {
            run_id: String::from(run_id),
            spec_file: Tags::escape_string(spec_file),
            suite: Tags::escape_string(suite),
            test_name: Tags::escape_string(test_name),
            status: String::from(status),
        }
    }

    fn escape_string(dirty_string: &str) -> String {
        dirty_string
            .replace(',', "\\,")
            .replace('=', "\\=")
            .replace(' ', "\\ ")
            .replace('\\', "\\")
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "runId={},specFile={},suite={},testName={},status={}",
            self.run_id, self.spec_file, self.suite, self.test_name, self.status
        )
    }
}

struct Fields {
    duration: String,
    passes: String,
    failures: String,
    error: Option<String>,
}

impl Fields {
    fn new(duration: u64, passes: bool, failures: bool, error: Option<String>) -> Fields {
        Fields {
            duration: duration.to_string(),
            passes: Fields::parse_boolean(passes),
            failures: Fields::parse_boolean(failures),
            error,
        }
    }

    fn parse_boolean(boolean: bool) -> String {
        if boolean {
            String::from("1i")
        } else {
            String::from("0i")
        }
    }

    fn escape_error(dirty_string: &str) -> String {
        dirty_string.replace('\n', "\\n")
    }
}

impl Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = format!(
            "duration={},passes={},failures={}",
            self.duration, self.passes, self.failures
        );
        match &self.error {
            Some(error_message) => write!(
                f,
                "{result},error=\"{}\"",
                Fields::escape_error(error_message)
            ),
            None => write!(f, "{result}"),
        }
    }
}

impl ParsedMochawesome {
    pub fn to_protocol_line(&self) -> String {
        let mut protocol_lines = String::new();

        for result in &self.results {
            let spec_file = &result.file;

            for suite in &result.suites {
                let suite_title = &suite.title;

                for test in &suite.tests {
                    let run_id = &test.uuid;
                    let test_name = &test.title;
                    let status = &test.state;
                    let duration = &test.duration;
                    let passes = &test.pass;
                    let failures = &test.fail;
                    let error = &test.err.message;

                    let tags = Tags::new(run_id, spec_file, suite_title, test_name, status);
                    let fields = Fields::new(*duration, *passes, *failures, error.clone());
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();
                    protocol_lines.push_str(
                        format!("cypress_test_results,{tags} {fields} {timestamp}\n").as_str(),
                    );
                }
            }
        }

        protocol_lines
    }
}
