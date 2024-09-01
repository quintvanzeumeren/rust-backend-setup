use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct UserAgent {
    user_agent: String,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
enum Error {
    #[error("User agent is too long")]
    MalformedUserAgent
}

const USER_AGENT_REGEX: &str = r"\((?P<info>.*?)\)(\s|$)|(?P<name>.*?)/(?P<version>.*?)(\s|$)";

impl FromStr for UserAgent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = regex::Regex::new(USER_AGENT_REGEX).expect("Invalid user agent regex");
        let valid_user_agent = regex.is_match(s);
        if valid_user_agent {
            return Ok(Self {
                user_agent: s.to_string(),
            });
        }

        return Err(Error::MalformedUserAgent);

        // For now just validate the user agent & create it
        // for cap in re.captures_iter(s) {
        //     if let Some(info) = cap.name("info") {
        //         println!("info: {}", info.as_str());
        //     }
        //
        //     if let Some(name) = cap.name("name") {
        //         println!("name: {}", name.as_str());
        //     }
        //
        //     if let Some(version) = cap.name("version") {
        //         println!("version: {}", version.as_str());
        //     }
        // }
    }
}

impl UserAgent {
    fn new(user_agent: &str) -> Result<Self, Error> {
        return UserAgent::from_str(user_agent);
    }

    fn user_agent(&self) -> &str {
        return &self.user_agent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user_agents = vec![
            "Mozilla/5.0 (Linux; Android 6.0.1; RedMi Note 5 Build/RB3N5C; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/68.0.3440.91 Mobile Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36",
            "Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36",
            "Mozilla/5.0 (Windows NT 5.1; rv:7.0.1) Gecko/20100101 Firefox/7.0.1",
            "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:24.0) Gecko/20100101 Firefox/24.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10; rv:33.0) Gecko/20100101 Firefox/33.0",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/17.17134",
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64; Trident/7.0; rv:11.0) like Gecko",
            "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0; KTXN)",
            "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1)",
            "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 1.1.4322)",
            "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0; WOW64; Trident/4.0; SLCC1; .NET CLR 2.0.50727; .NET CLR 3.5.30729; .NET CLR 3.0.30729; .NET4.0C; .NET4.0E)",

            // from mozilla https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
            "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.106 Safari/537.36 OPR/38.0.2220.41",
            "Opera/9.80 (Macintosh; Intel Mac OS X; U; en) Presto/2.2.15 Version/10.00 Opera/9.60 (Windows NT 6.0; U; en) Presto/2.1.1",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1",
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
            "Mozilla/5.0 (compatible; YandexAccessibilityBot/3.0; +http://yandex.com/bots)",
            "curl/7.64.1",
            "PostmanRuntime/7.26.5",
        ];

        struct TestConfig {
            test_name: String,
            user_agent: String,
            expected: Result<UserAgent, Error>,
        }

        let mut tests = user_agents.iter().enumerate().map(|(i, user_agent)| {
            let expected = UserAgent {
                user_agent: user_agent.to_string(),
            };
            return TestConfig {
                test_name: format!("test_{}", i),
                user_agent: user_agent.to_string(),
                expected: Ok(expected),
            }
        }).collect::<Vec<TestConfig>>();

        tests.push(TestConfig {
            test_name: "Incorrect user agent".to_string(),
            user_agent: "incorrect not a user agent".to_string(),
            expected: Err(Error::MalformedUserAgent),
        });

        for test in tests {
            let result = UserAgent::new(&test.user_agent);
            assert_eq!(result, test.expected, "{}", test.test_name);
        }
    }

    #[test]
    fn test_user_agent() {
        let user_agent = "Mozilla/5.0 (Linux; Android 6.0.1; RedMi Note 5 Build/RB3N5C; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/68.0.3440.91 Mobile Safari/537.36";
        let expected = UserAgent {
            user_agent: user_agent.to_string(),
        };

        assert_eq!(expected.user_agent(), user_agent);
        assert_eq!(expected.user_agent(), user_agent);
    }
}