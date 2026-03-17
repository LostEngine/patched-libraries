use serde::{Deserialize, Serialize};

use crate::{
    DocumentSelector, DynamicRegistrationClientCapabilities, Range, TextDocumentIdentifier,
    TextDocumentPositionParams, WorkDoneProgressParams,
};

use std::collections::HashMap;

pub type DocumentFormattingClientCapabilities = DynamicRegistrationClientCapabilities;
pub type DocumentRangeFormattingClientCapabilities = DynamicRegistrationClientCapabilities;
pub type DocumentOnTypeFormattingClientCapabilities = DynamicRegistrationClientCapabilities;

/// Format document on type options
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingOptions {
    /// A character on which formatting should be triggered, like `}`.
    pub first_trigger_character: String,

    /// More trigger characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_trigger_character: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The format options.
    pub options: FormattingOptions,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

/// Value-object describing what options formatting should use.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions {
    /// Size of a tab in spaces.
    pub tab_size: u32,

    /// Prefer spaces over tabs.
    pub insert_spaces: bool,

    /// Signature for further properties.
    #[serde(flatten)]
    pub properties: HashMap<String, FormattingProperty>,

    /// Trim trailing whitespace on a line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_trailing_whitespace: Option<bool>,

    /// Insert a newline character at the end of the file if one does not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_final_newline: Option<bool>,

    /// Trim all newlines after the final newline at the end of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_final_newlines: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FormattingProperty {
    Bool(bool),
    Number(i32),
    String(String),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The range to format
    pub range: Range,

    /// The format options
    pub options: FormattingOptions,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

/// The parameters of a `textDocument/rangesFormatting` request.
///
/// @since 3.18.0
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangesFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The ranges to format
    pub ranges: Vec<Range>,

    /// The format options
    pub options: FormattingOptions,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingParams {
    /// Text Document and Position fields.
    #[serde(flatten)]
    pub text_document_position: TextDocumentPositionParams,

    /// The character that has been typed.
    pub ch: String,

    /// The format options.
    pub options: FormattingOptions,
}

/// Extends TextDocumentRegistrationOptions
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// A character on which formatting should be triggered, like `}`.
    pub first_trigger_character: String,

    /// More trigger characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_trigger_character: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_serialization;
    use crate::{Position, TextDocumentIdentifier, Uri, WorkDoneProgressParams};
    use std::str::FromStr;

    #[test]
    fn formatting_options() {
        test_serialization(
            &FormattingOptions {
                tab_size: 123,
                insert_spaces: true,
                properties: HashMap::new(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
            r#"{"tabSize":123,"insertSpaces":true}"#,
        );

        test_serialization(
            &FormattingOptions {
                tab_size: 123,
                insert_spaces: true,
                properties: vec![("prop".to_string(), FormattingProperty::Number(1))]
                    .into_iter()
                    .collect(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
            r#"{"tabSize":123,"insertSpaces":true,"prop":1}"#,
        );
    }

    #[test]
    fn test_document_formatting_params() {
        let params = DocumentFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: Uri::from_str("file:///test.rs").unwrap(),
            },
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                properties: HashMap::new(),
                trim_trailing_whitespace: Some(true),
                insert_final_newline: Some(true),
                trim_final_newlines: Some(false),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains(r#""textDocument""#));
        assert!(json.contains(r#""file:///test.rs""#));
        assert!(json.contains(r#""tabSize":4"#));
        assert!(json.contains(r#""insertSpaces":true"#));

        let deserialized: DocumentFormattingParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, params);
    }

    #[test]
    fn test_document_range_formatting_params() {
        let params = DocumentRangeFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: Uri::from_str("file:///test.rs").unwrap(),
            },
            range: Range {
                start: Position {
                    line: 10,
                    character: 5,
                },
                end: Position {
                    line: 20,
                    character: 15,
                },
            },
            options: FormattingOptions {
                tab_size: 2,
                insert_spaces: false,
                properties: HashMap::new(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains(r#""range""#));
        assert!(json.contains(r#""line":10"#));
        assert!(json.contains(r#""character":5"#));

        let deserialized: DocumentRangeFormattingParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, params);
    }

    #[test]
    fn test_document_ranges_formatting_params() {
        let params = DocumentRangesFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: Uri::from_str("file:///test.rs").unwrap(),
            },
            ranges: vec![
                Range {
                    start: Position {
                        line: 10,
                        character: 5,
                    },
                    end: Position {
                        line: 20,
                        character: 15,
                    },
                },
                Range {
                    start: Position {
                        line: 30,
                        character: 0,
                    },
                    end: Position {
                        line: 40,
                        character: 10,
                    },
                },
            ],
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                properties: HashMap::new(),
                trim_trailing_whitespace: Some(true),
                insert_final_newline: None,
                trim_final_newlines: None,
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: Some(crate::NumberOrString::String("test-token".to_string())),
            },
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains(r#""ranges""#));
        assert!(json.contains(r#""line":10"#));
        assert!(json.contains(r#""line":30"#));
        assert!(json.contains(r#""workDoneToken":"test-token""#));

        let deserialized: DocumentRangesFormattingParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, params);
    }

    #[test]
    fn test_document_ranges_formatting_params_empty() {
        let params = DocumentRangesFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: Uri::from_str("file:///empty.txt").unwrap(),
            },
            ranges: vec![],
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                properties: HashMap::new(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: DocumentRangesFormattingParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.ranges.len(), 0);
    }

    #[test]
    fn test_document_on_type_formatting_params() {
        let params = DocumentOnTypeFormattingParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Uri::from_str("file:///test.rs").unwrap(),
                },
                position: Position {
                    line: 5,
                    character: 10,
                },
            },
            ch: "}".to_string(),
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                properties: HashMap::new(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains(r#""ch":"}""#));
        assert!(json.contains(r#""line":5"#));
        assert!(json.contains(r#""character":10"#));

        let deserialized: DocumentOnTypeFormattingParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, params);
    }

    #[test]
    fn test_formatting_property_variants() {
        let mut options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            properties: HashMap::new(),
            trim_trailing_whitespace: None,
            insert_final_newline: None,
            trim_final_newlines: None,
        };

        options
            .properties
            .insert("numberProp".to_string(), FormattingProperty::Number(42));
        options.properties.insert(
            "stringProp".to_string(),
            FormattingProperty::String("value".to_string()),
        );
        options
            .properties
            .insert("boolProp".to_string(), FormattingProperty::Bool(true));

        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""numberProp":42"#));
        assert!(json.contains(r#""stringProp":"value""#));
        assert!(json.contains(r#""boolProp":true"#));

        let deserialized: FormattingOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.properties.len(), 3);
    }

    #[test]
    fn test_document_on_type_formatting_options() {
        let options = DocumentOnTypeFormattingOptions {
            first_trigger_character: "{".to_string(),
            more_trigger_character: Some(vec!["}".to_string(), ";".to_string()]),
        };

        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains(r#""firstTriggerCharacter":"{""#));
        assert!(json.contains(r#""moreTriggerCharacter":["}",";"]"#));

        let deserialized: DocumentOnTypeFormattingOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, options);
    }

    #[test]
    fn test_formatting_options_optional_fields() {
        // Test with all optional fields set
        let options_full = FormattingOptions {
            tab_size: 2,
            insert_spaces: false,
            properties: HashMap::new(),
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(false),
            trim_final_newlines: Some(true),
        };

        let json = serde_json::to_string(&options_full).unwrap();
        assert!(json.contains(r#""trimTrailingWhitespace":true"#));
        assert!(json.contains(r#""insertFinalNewline":false"#));
        assert!(json.contains(r#""trimFinalNewlines":true"#));

        // Test with optional fields omitted
        let options_minimal = FormattingOptions {
            tab_size: 2,
            insert_spaces: false,
            properties: HashMap::new(),
            trim_trailing_whitespace: None,
            insert_final_newline: None,
            trim_final_newlines: None,
        };

        let json = serde_json::to_string(&options_minimal).unwrap();
        assert!(!json.contains(r#"trimTrailingWhitespace"#));
        assert!(!json.contains(r#"insertFinalNewline"#));
        assert!(!json.contains(r#"trimFinalNewlines"#));
    }
}
