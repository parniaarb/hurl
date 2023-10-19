/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use crate::ast::*;
use crate::parser::combinators::*;
use crate::parser::error::*;
use crate::parser::primitives::*;
use crate::parser::reader::Reader;
use crate::parser::string::*;
use crate::parser::{filename, ParseResult};

pub fn parse(reader: &mut Reader) -> ParseResult<EntryOption> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let pos = reader.state.pos.clone();
    let option = reader.read_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.');
    let space1 = zero_or_more_spaces(reader)?;
    try_literal(":", reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let kind = match option.as_str() {
        "aws-sigv4" => option_aws_sigv4(reader)?,
        "cacert" => option_cacert(reader)?,
        "cert" => option_cert(reader)?,
        "compressed" => option_compressed(reader)?,
        "connect-to" => option_connect_to(reader)?,
        "delay" => option_delay(reader)?,
        "insecure" => option_insecure(reader)?,
        "http1.0" => option_http_10(reader)?,
        "http1.1" => option_http_11(reader)?,
        "http2" => option_http_2(reader)?,
        "http3" => option_http_3(reader)?,
        "ipv4" => option_ipv4(reader)?,
        "ipv6" => option_ipv6(reader)?,
        "key" => option_key(reader)?,
        "location" => option_follow_location(reader)?,
        "max-redirs" => option_max_redirect(reader)?,
        "path-as-is" => option_path_as_is(reader)?,
        "proxy" => option_proxy(reader)?,
        "resolve" => option_resolve(reader)?,
        "retry" => option_retry(reader)?,
        "retry-interval" => option_retry_interval(reader)?,
        "variable" => option_variable(reader)?,
        "verbose" => option_verbose(reader)?,
        "very-verbose" => option_very_verbose(reader)?,
        _ => {
            return Err(Error {
                pos,
                recoverable: true,
                inner: ParseError::InvalidOption,
            });
        }
    };
    let line_terminator0 = line_terminator(reader)?;

    Ok(EntryOption {
        line_terminators,
        space0,
        space1,
        space2,
        kind,
        line_terminator0,
    })
}

fn option_aws_sigv4(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = aws_sigv4(reader)?;
    Ok(OptionKind::AwsSigV4(value))
}

fn option_cacert(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::CaCertificate(value))
}

fn option_cert(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::ClientCert(value))
}

fn option_compressed(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Compressed(value))
}

fn option_connect_to(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = connect_to(reader)?;
    Ok(OptionKind::ConnectTo(value))
}

fn option_delay(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural, reader)?;
    Ok(OptionKind::Delay(value))
}

fn option_follow_location(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::FollowLocation(value))
}

fn option_http_10(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http10(value))
}

fn option_http_11(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http11(value))
}

fn option_http_2(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http2(value))
}

fn option_http_3(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http3(value))
}

fn option_insecure(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Insecure(value))
}

fn option_ipv4(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::IpV4(value))
}

fn option_ipv6(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::IpV6(value))
}

fn option_key(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::ClientKey(value))
}

fn option_max_redirect(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural, reader)?;
    // FIXME: try to not unwrap redirect value
    // and returns an error if not possible
    let value = usize::try_from(value).unwrap();
    Ok(OptionKind::MaxRedirect(value))
}

fn option_path_as_is(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::PathAsIs(value))
}

fn option_proxy(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = proxy(reader)?;
    Ok(OptionKind::Proxy(value))
}

fn option_resolve(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = resolve(reader)?;
    Ok(OptionKind::Resolve(value))
}

fn option_retry(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = retry(reader)?;
    Ok(OptionKind::Retry(value))
}

fn option_retry_interval(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural, reader)?;
    Ok(OptionKind::RetryInterval(value))
}

fn option_variable(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = variable_definition(reader)?;
    Ok(OptionKind::Variable(value))
}

fn option_verbose(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Verbose(value))
}

fn option_very_verbose(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::VeryVerbose(value))
}

fn aws_sigv4(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let provider = reader.read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '-');
    if provider.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "aws-sigv4 provider".to_string(),
            },
        });
    }
    Ok(provider)
}

fn proxy(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader
        .read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '.' || *c == '[' || *c == ']');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "proxy name".to_string(),
            },
        });
    }
    Ok(name)
}

fn resolve(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '.');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "resolve".to_string(),
            },
        });
    }
    if !name.contains(':') {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "HOST:PORT:ADDR".to_string(),
            },
        });
    }
    Ok(name)
}

fn connect_to(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '.');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "connect-to".to_string(),
            },
        });
    }
    if !name.contains(':') {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "HOST1:PORT1:HOST2:PORT2".to_string(),
            },
        });
    }
    Ok(name)
}

fn retry(reader: &mut Reader) -> ParseResult<Retry> {
    let pos = reader.state.pos.clone();
    let value = nonrecover(integer, reader)?;
    if value == -1 {
        Ok(Retry::Infinite)
    } else if value == 0 {
        Ok(Retry::None)
    } else if value > 0 {
        Ok(Retry::Finite(value as usize))
    } else {
        Err(Error {
            pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "Expecting a retry value".to_string(),
            },
        })
    }
}
fn variable_definition(reader: &mut Reader) -> ParseResult<VariableDefinition> {
    let name = variable_name(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    literal("=", reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    let value = variable_value(reader)?;
    Ok(VariableDefinition {
        name,
        space0,
        space1,
        value,
    })
}

fn variable_name(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "variable name".to_string(),
            },
        });
    }
    Ok(name)
}

fn variable_value(reader: &mut Reader) -> ParseResult<VariableValue> {
    choice(
        &[
            |p1| match null(p1) {
                Ok(()) => Ok(VariableValue::Null),
                Err(e) => Err(e),
            },
            |p1| match boolean(p1) {
                Ok(value) => Ok(VariableValue::Bool(value)),
                Err(e) => Err(e),
            },
            |p1| match float(p1) {
                Ok(value) => Ok(VariableValue::Float(value)),
                Err(e) => Err(e),
            },
            |p1| match integer(p1) {
                Ok(value) => Ok(VariableValue::Integer(value)),
                Err(e) => Err(e),
            },
            |p1| match quoted_template(p1) {
                Ok(value) => Ok(VariableValue::String(value)),
                Err(e) => Err(e),
            },
            |p1| match unquoted_template(p1) {
                Ok(value) => Ok(VariableValue::String(value)),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
    .map_err(|e| Error {
        pos: e.pos,
        recoverable: false,
        inner: ParseError::Expecting {
            value: "variable value".to_string(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

    #[test]
    fn test_option_insecure() {
        let mut reader = Reader::new("insecure: true");
        let option = parse(&mut reader).unwrap();
        assert_eq!(
            option,
            EntryOption {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 1 },
                        end: Pos { line: 1, column: 1 },
                    },
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 9 },
                        end: Pos { line: 1, column: 9 },
                    },
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo {
                        start: Pos {
                            line: 1,
                            column: 10,
                        },
                        end: Pos {
                            line: 1,
                            column: 11,
                        },
                    },
                },
                kind: OptionKind::Insecure(true),
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 15,
                            },
                            end: Pos {
                                line: 1,
                                column: 15,
                            },
                        },
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 15,
                            },
                            end: Pos {
                                line: 1,
                                column: 15,
                            },
                        },
                    },
                },
            }
        );
    }

    #[test]
    fn test_option_insecure_error() {
        let mut reader = Reader::new("insecure: error");
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable)
    }

    #[test]
    fn test_option_cacert() {
        let mut reader = Reader::new("cacert: /home/foo/cert.pem");
        let option = parse(&mut reader).unwrap();
        assert_eq!(
            option,
            EntryOption {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 1 },
                        end: Pos { line: 1, column: 1 },
                    },
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 7 },
                        end: Pos { line: 1, column: 7 },
                    },
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 8 },
                        end: Pos { line: 1, column: 9 },
                    },
                },
                kind: OptionKind::CaCertificate(Filename {
                    value: "/home/foo/cert.pem".to_string(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 9 },
                        end: Pos {
                            line: 1,
                            column: 27,
                        },
                    },
                }),
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 27,
                            },
                            end: Pos {
                                line: 1,
                                column: 27,
                            },
                        },
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 27,
                            },
                            end: Pos {
                                line: 1,
                                column: 27,
                            },
                        },
                    },
                },
            }
        );
    }

    #[test]
    fn test_option_cacert_error() {
        let mut reader = Reader::new("cacert: ###");
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable)
    }

    #[test]
    fn test_variable_definition() {
        let mut reader = Reader::new("a=1");
        assert_eq!(
            variable_definition(&mut reader).unwrap(),
            VariableDefinition {
                name: "a".to_string(),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 2 },
                        end: Pos { line: 1, column: 2 },
                    },
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 3 },
                        end: Pos { line: 1, column: 3 },
                    },
                },
                value: VariableValue::Integer(1),
            }
        );
    }

    #[test]
    fn test_variable_value() {
        let mut reader = Reader::new("null");
        assert_eq!(variable_value(&mut reader).unwrap(), VariableValue::Null);

        let mut reader = Reader::new("true");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::Bool(true)
        );

        let mut reader = Reader::new("1");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::Integer(1)
        );

        let mut reader = Reader::new("toto");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::String(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "toto".to_string(),
                    encoded: "toto".to_string(),
                }],
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 1 },
                    end: Pos { line: 1, column: 5 },
                },
            })
        );
        let mut reader = Reader::new("\"123\"");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::String(Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "123".to_string(),
                    encoded: "123".to_string(),
                }],
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 1 },
                    end: Pos { line: 1, column: 6 },
                },
            })
        );
    }
}