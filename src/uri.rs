use std::{
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};

/// Newtype struct around `url::Url` with transparent serialization.
///
/// The `url` crate provides a robust, battle-tested implementation of URL parsing
/// that follows the WHATWG URL Standard. It already implements all necessary traits
/// including `Serialize`, `Deserialize`, `Hash`, `Eq`, `Ord`, and `FromStr`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uri(url::Url);

impl Uri {
    // LostEngine -- modify parse_from_file_path
    pub fn parse_from_file_path(path: &Path) -> Result<Self, url::ParseError> {
        let path_str = path
            .to_str()
            .ok_or(url::ParseError::RelativeUrlWithoutBase)?;

        let url_str = if cfg!(windows) {
            format!("file:///{}", path_str.replace('\\', "/"))
        } else {
            format!("file://{}", path_str)
        };

        let url = url::Url::parse(&url_str)?;
        Ok(Uri(url))
    }

    pub fn get_file_path(&self) -> Option<PathBuf> {
        if self.0.scheme() != "file" {
            return None;
        }

        let decoded_path = percent_decode_str(self.0.path())
            .decode_utf8()
            .ok()?
            .to_string();

        let decoded_path = if cfg!(windows) {
            let mut windows_decoded_path = decoded_path.trim_start_matches('/').replace('\\', "/");
            if windows_decoded_path.len() >= 2 && windows_decoded_path.chars().nth(1) == Some(':') {
                let drive = windows_decoded_path.chars().next()?.to_ascii_uppercase();
                windows_decoded_path.replace_range(..2, &format!("{}:", drive));
            }

            windows_decoded_path
        } else {
            decoded_path
        };

        Some(PathBuf::from(decoded_path))
    }
}

impl FromStr for Uri {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::from_str(s).map(Self)
    }
}

impl Deref for Uri {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::Uri;
    use std::{path::Path, str::FromStr};

    #[test]
    #[cfg(windows)]
    fn test_uri_basic() {
        let uri =
            Uri::from_str("file:///c%3a/%E6%96%B0%E5%BB%BA%E6%96%87%E4%BB%B6%E5%A4%B9").unwrap();
        let path = uri.get_file_path().unwrap();
        let result_path = Path::new("c:/新建文件夹");
        assert_eq!(path, result_path);
    }

    #[test]
    fn test_uri_parse_simple_file_path() {
        let uri = Uri::from_str("file:///home/user/document.txt").unwrap();
        assert_eq!(uri.scheme(), "file");
        assert_eq!(uri.path(), "/home/user/document.txt");
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_windows_drive_letter() {
        // Test Windows drive letter
        let uri = Uri::from_str("file:///c:/Users/test/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("C:/Users/test/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_windows_drive_letter_lowercase() {
        // Lowercase drive letter should be converted to uppercase
        let uri = Uri::from_str("file:///d:/projects/main.rs").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("D:/projects/main.rs"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_windows_encoded_drive() {
        // Encoded drive letter (c%3a -> c:)
        let uri = Uri::from_str("file:///c%3a/folder/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("C:/folder/file.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_percent_encoding_space() {
        // Test percent encoding for space (Unix)
        let uri = Uri::from_str("file:///path/with%20spaces/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/with spaces/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_percent_encoding_space() {
        // Test percent encoding for space (Windows)
        let uri = Uri::from_str("file:///path/with%20spaces/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/with spaces/file.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_chinese_characters() {
        // Test Chinese characters (Unix)
        let uri = Uri::from_str("file:///home/%E7%94%A8%E6%88%B7/%E6%96%87%E6%A1%A3.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/home/用户/文档.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_chinese_characters() {
        // Test Chinese characters (Windows)
        let uri = Uri::from_str("file:///home/%E7%94%A8%E6%88%B7/%E6%96%87%E6%A1%A3.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("home/用户/文档.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_special_characters() {
        // Test special characters (Unix)
        let uri = Uri::from_str("file:///path/with%21%40%23%24%25/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/with!@#$%/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_special_characters() {
        // Test special characters (Windows)
        let uri = Uri::from_str("file:///path/with%21%40%23%24%25/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/with!@#$%/file.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_unicode_emoji() {
        // Test Unicode emoji (Unix)
        let uri = Uri::from_str("file:///folder/%F0%9F%98%80/test.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/folder/😀/test.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_unicode_emoji() {
        // Test Unicode emoji (Windows)
        let uri = Uri::from_str("file:///folder/%F0%9F%98%80/test.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("folder/😀/test.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_parse_from_file_path() {
        // Test creating URI from file path (Unix)
        let path = Path::new("/home/user/test.txt");
        let uri = Uri::parse_from_file_path(path).unwrap();
        assert_eq!(uri.scheme(), "file");
        assert!(uri.path().contains("test.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_parse_from_file_path() {
        // Test creating URI from file path (Windows)
        let path = Path::new("C:\\Users\\test\\file.txt");
        let uri = Uri::parse_from_file_path(path).unwrap();
        assert_eq!(uri.scheme(), "file");
        assert!(uri.path().contains("file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_parse_from_windows_path() {
        // Test creating URI from Windows path
        let path = Path::new("C:\\Users\\test\\file.txt");
        let uri = Uri::parse_from_file_path(path).unwrap();
        assert_eq!(uri.scheme(), "file");
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_roundtrip_simple() {
        // Test roundtrip conversion - simple path (Unix)
        let original_path = Path::new("/tmp/test.txt");
        let uri = Uri::parse_from_file_path(original_path).unwrap();
        let recovered_path = uri.get_file_path().unwrap();
        assert_eq!(original_path, recovered_path);
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_roundtrip_simple() {
        // Test roundtrip conversion - simple path (Windows)
        let original_path = Path::new("C:\\tmp\\test.txt");
        let uri = Uri::parse_from_file_path(original_path).unwrap();
        let recovered_path = uri.get_file_path().unwrap();
        assert_eq!(
            original_path.to_string_lossy().replace('\\', "/"),
            recovered_path.to_string_lossy()
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_roundtrip_with_spaces() {
        // Test roundtrip conversion - path with spaces (Unix)
        let original_path = Path::new("/path/with spaces/file.txt");
        let uri = Uri::parse_from_file_path(original_path).unwrap();
        let recovered_path = uri.get_file_path().unwrap();
        assert_eq!(original_path, recovered_path);
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_roundtrip_with_spaces() {
        // Test roundtrip conversion - path with spaces (Windows)
        let original_path = Path::new("C:\\path\\with spaces\\file.txt");
        let uri = Uri::parse_from_file_path(original_path).unwrap();
        let recovered_path = uri.get_file_path().unwrap();
        assert_eq!(
            original_path.to_string_lossy().replace('\\', "/"),
            recovered_path.to_string_lossy()
        );
    }

    #[test]
    fn test_uri_non_file_scheme() {
        // Test non-file scheme
        let uri = Uri::from_str("https://example.com/path").unwrap();
        assert_eq!(uri.scheme(), "https");
        assert_eq!(uri.host_str(), Some("example.com"));
        assert!(uri.get_file_path().is_none());
    }

    #[test]
    fn test_uri_http_scheme() {
        let uri = Uri::from_str("http://localhost:8080/api/v1").unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host_str(), Some("localhost"));
        assert_eq!(uri.port(), Some(8080));
        assert_eq!(uri.path(), "/api/v1");
    }

    #[test]
    fn test_uri_with_query() {
        // Test URI with query parameter
        let uri = Uri::from_str("file:///path/file.txt?version=1").unwrap();
        assert_eq!(uri.query(), Some("version=1"));
    }

    #[test]
    fn test_uri_with_fragment() {
        // Test URI with fragment
        let uri = Uri::from_str("file:///path/file.txt#section").unwrap();
        assert_eq!(uri.fragment(), Some("section"));
    }

    #[test]
    fn test_uri_equality() {
        // Test URI equality
        let uri1 = Uri::from_str("file:///path/file.txt").unwrap();
        let uri2 = Uri::from_str("file:///path/file.txt").unwrap();
        assert_eq!(uri1, uri2);
    }

    #[test]
    fn test_uri_ordering() {
        // Test URI ordering
        let uri1 = Uri::from_str("file:///aaa.txt").unwrap();
        let uri2 = Uri::from_str("file:///bbb.txt").unwrap();
        assert!(uri1 < uri2);
    }

    #[test]
    fn test_uri_hash() {
        // Test URI can be used in HashMap
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let uri1 = Uri::from_str("file:///path1.txt").unwrap();
        let uri2 = Uri::from_str("file:///path2.txt").unwrap();
        let uri3 = Uri::from_str("file:///path1.txt").unwrap();

        set.insert(uri1.clone());
        set.insert(uri2.clone());
        set.insert(uri3.clone());

        assert_eq!(set.len(), 2); // uri1 and uri3 are the same
        assert!(set.contains(&uri1));
        assert!(set.contains(&uri2));
    }

    #[test]
    fn test_uri_clone() {
        // Test URI clone
        let uri1 = Uri::from_str("file:///path/file.txt").unwrap();
        let uri2 = uri1.clone();
        assert_eq!(uri1, uri2);
    }

    #[test]
    fn test_uri_deref() {
        // Test Deref trait
        let uri = Uri::from_str("file:///path/file.txt").unwrap();
        assert_eq!(uri.scheme(), "file");
        assert_eq!(uri.path(), "/path/file.txt");
    }

    #[test]
    fn test_uri_serialization() {
        // Test serialization
        let uri = Uri::from_str("file:///path/file.txt").unwrap();
        let json = serde_json::to_string(&uri).unwrap();
        assert!(json.contains("file:///path/file.txt"));
    }

    #[test]
    fn test_uri_deserialization() {
        // Test deserialization
        let json = r#""file:///path/file.txt""#;
        let uri: Uri = serde_json::from_str(json).unwrap();
        assert_eq!(uri.scheme(), "file");
        assert_eq!(uri.path(), "/path/file.txt");
    }

    #[test]
    fn test_uri_roundtrip_serialization() {
        // Test roundtrip serialization
        let original = Uri::from_str("file:///path/test.txt").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Uri = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_uri_invalid_string() {
        // Test invalid URI string
        let result = Uri::from_str("not a valid uri");
        assert!(result.is_err());
    }

    #[test]
    fn test_uri_empty_string() {
        // Test empty string
        let result = Uri::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_uri_get_file_path_non_file_scheme() {
        // Test get_file_path returns None for non-file scheme
        let uri = Uri::from_str("https://example.com").unwrap();
        assert!(uri.get_file_path().is_none());
    }

    #[test]
    fn test_uri_vscode_style() {
        // Test VS Code style URI
        let uri = Uri::from_str("file:///c%3A/Users/test/project/src/main.rs").unwrap();
        assert_eq!(uri.scheme(), "file");
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_nested_directories() {
        // Test deeply nested directories (Unix)
        let uri = Uri::from_str("file:///a/b/c/d/e/f/g/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/a/b/c/d/e/f/g/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_nested_directories() {
        // Test deeply nested directories (Windows)
        let uri = Uri::from_str("file:///a/b/c/d/e/f/g/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("a/b/c/d/e/f/g/file.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_dot_in_filename() {
        // Test dot in filename (Unix)
        let uri = Uri::from_str("file:///path/file.test.backup.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/file.test.backup.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_dot_in_filename() {
        // Test dot in filename (Windows)
        let uri = Uri::from_str("file:///path/file.test.backup.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/file.test.backup.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_underscore_and_dash() {
        // Test underscore and dash (Unix)
        let uri = Uri::from_str("file:///path/my_file-name.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/my_file-name.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_underscore_and_dash() {
        // Test underscore and dash (Windows)
        let uri = Uri::from_str("file:///path/my_file-name.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/my_file-name.txt"));
    }

    #[test]
    fn test_uri_debug_format() {
        // Test Debug trait
        let uri = Uri::from_str("file:///path/file.txt").unwrap();
        let debug_str = format!("{:?}", uri);
        assert!(debug_str.contains("file"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_unix_absolute_path() {
        // Test Unix absolute path
        let uri = Uri::from_str("file:///usr/local/bin/program").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/usr/local/bin/program"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_unix_home_directory() {
        // Test Unix home directory style
        let uri = Uri::from_str("file:///home/user/.config/settings.json").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/home/user/.config/settings.json"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_multiple_percent_encodings() {
        // Test multiple percent encodings (Unix)
        let uri = Uri::from_str("file:///path/%E6%B5%8B%E8%AF%95/%E6%96%87%E4%BB%B6.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/测试/文件.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_multiple_percent_encodings() {
        // Test multiple percent encodings (Windows)
        let uri = Uri::from_str("file:///path/%E6%B5%8B%E8%AF%95/%E6%96%87%E4%BB%B6.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/测试/文件.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_japanese_characters() {
        // Test Japanese characters (Unix)
        let uri = Uri::from_str("file:///path/%E3%83%86%E3%82%B9%E3%83%88.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/テスト.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_japanese_characters() {
        // Test Japanese characters (Windows)
        let uri = Uri::from_str("file:///path/%E3%83%86%E3%82%B9%E3%83%88.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/テスト.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_korean_characters() {
        // Test Korean characters (Unix)
        let uri = Uri::from_str("file:///path/%ED%85%8C%EC%8A%A4%ED%8A%B8.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/테스트.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_korean_characters() {
        // Test Korean characters (Windows)
        let uri = Uri::from_str("file:///path/%ED%85%8C%EC%8A%A4%ED%8A%B8.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/테스트.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_parentheses_in_path() {
        // Test parentheses in path (Unix)
        let uri = Uri::from_str("file:///path/folder%20%28copy%29/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/folder (copy)/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_parentheses_in_path() {
        // Test parentheses in path (Windows)
        let uri = Uri::from_str("file:///path/folder%20%28copy%29/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/folder (copy)/file.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_square_brackets() {
        // Test square brackets (Unix)
        let uri = Uri::from_str("file:///path/%5Btest%5D/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/[test]/file.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_square_brackets() {
        // Test square brackets (Windows)
        let uri = Uri::from_str("file:///path/%5Btest%5D/file.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/[test]/file.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_plus_sign() {
        // Test plus sign (Unix)
        let uri = Uri::from_str("file:///path/file%2Bbackup.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/file+backup.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_plus_sign() {
        // Test plus sign (Windows)
        let uri = Uri::from_str("file:///path/file%2Bbackup.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/file+backup.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_ampersand() {
        // Test ampersand (&) (Unix)
        let uri = Uri::from_str("file:///path/A%26B.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/A&B.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_ampersand() {
        // Test ampersand (&) (Windows)
        let uri = Uri::from_str("file:///path/A%26B.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/A&B.txt"));
    }

    #[test]
    #[cfg(unix)]
    fn test_uri_equals_sign() {
        // Test equals sign (=) (Unix)
        let uri = Uri::from_str("file:///path/key%3Dvalue.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("/path/key=value.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_equals_sign() {
        // Test equals sign (=) (Windows)
        let uri = Uri::from_str("file:///path/key%3Dvalue.txt").unwrap();
        let path = uri.get_file_path().unwrap();
        assert_eq!(path, Path::new("path/key=value.txt"));
    }

    #[test]
    #[cfg(windows)]
    fn test_uri_windows_unc_path_not_supported() {
        // UNC paths are usually not directly supported by file:// URI
        // This test ensures our code does not panic
        let uri = Uri::from_str("file://server/share/file.txt");
        // Just don't panic
        let _ = uri.map(|u| u.get_file_path());
    }

    #[test]
    fn test_uri_collection_operations() {
        // Test URI operations in collections
        let mut uris = vec![
            Uri::from_str("file:///c.txt").unwrap(),
            Uri::from_str("file:///a.txt").unwrap(),
            Uri::from_str("file:///b.txt").unwrap(),
        ];
        uris.sort();
        assert_eq!(uris[0].path(), "/a.txt");
        assert_eq!(uris[1].path(), "/b.txt");
        assert_eq!(uris[2].path(), "/c.txt");
    }

    #[test]
    fn test_uri_with_port() {
        // Test URI with port (file:// usually does not have port)
        let uri = Uri::from_str("http://localhost:3000/path").unwrap();
        assert_eq!(uri.port(), Some(3000));
    }

    #[test]
    fn test_uri_git_scheme() {
        // Test git scheme
        let uri = Uri::from_str("git://github.com/user/repo.git").unwrap();
        assert_eq!(uri.scheme(), "git");
        assert_eq!(uri.host_str(), Some("github.com"));
    }

    #[test]
    fn test_uri_trailing_slash() {
        // Test trailing slash
        let uri1 = Uri::from_str("file:///path/folder/").unwrap();
        let uri2 = Uri::from_str("file:///path/folder").unwrap();
        // url crate may normalize these, so they may be equal or not
        // Just ensure both can be parsed
        assert!(uri1.path().contains("folder"));
        assert!(uri2.path().contains("folder"));
    }
}
