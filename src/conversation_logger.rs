/// Conversation Logger - Session-based conversation logging
///
/// Creates dedicated log files for each conversation session with clean organization
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Session-based conversation logger
pub struct ConversationLogger {
    /// Path to the log file for this session
    log_file_path: PathBuf,
    /// File handle for writing
    file: Option<File>,
    /// Session start time
    session_start: DateTime<Local>,
    /// Whether logging is enabled
    enabled: bool,
    /// Track if any messages were logged
    has_content: bool,
    /// Logs folder path
    logs_folder: String,
}

impl ConversationLogger {
    /// Create a new conversation logger for this session
    pub fn new(logs_folder: &str, enabled: bool) -> Result<Self> {
        if !enabled {
            return Ok(Self {
                log_file_path: PathBuf::new(),
                file: None,
                session_start: Local::now(),
                enabled: false,
                has_content: false,
                logs_folder: String::new(),
            });
        }

        // Create logs directory if it doesn't exist
        let logs_path = Path::new(logs_folder);
        if !logs_path.exists() {
            fs::create_dir_all(logs_path)
                .context("Failed to create conversation_logs directory")?;
            tracing::info!("📁 Created conversation logs directory: {}", logs_folder);
        }

        // Generate session filename with timestamp (but don't create file yet)
        let session_start = Local::now();
        let filename = format!(
            "vi_session_{}.txt",
            session_start.format("%Y_%m_%d_%H_%M_%S")
        );
        let log_file_path = logs_path.join(&filename);

        // Don't create file until first actual message
        tracing::debug!("📝 Session logger ready (file will be created on first message)");

        Ok(Self {
            log_file_path,
            file: None,
            session_start,
            enabled: true,
            has_content: false,
            logs_folder: logs_folder.to_string(),
        })
    }

    /// Lazy file creation - only create when first message is logged
    fn ensure_file_created(&mut self) -> Result<()> {
        if self.file.is_some() {
            return Ok(()); // Already created
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file_path)
            .context("Failed to create session log file")?;

        // Write session header
        writeln!(file, "=")?;
        writeln!(file, "VI CONVERSATION SESSION")?;
        writeln!(file, "=")?;
        writeln!(
            file,
            "Session Start: {}",
            self.session_start.format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(file, "=")?;
        writeln!(file)?;

        file.flush()?;

        self.file = Some(file);
        tracing::info!("📝 Session log created: {}", self.log_file_path.display());

        Ok(())
    }

    /// Log a user message
    pub fn log_user(&mut self, message: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // Create file on first message
        self.ensure_file_created()?;
        self.has_content = true;

        if let Some(ref mut file) = self.file {
            let timestamp = Local::now();
            writeln!(file, "[{}] USER:", timestamp.format("%H:%M:%S"))?;
            writeln!(file, "{}", message)?;
            writeln!(file)?;
            file.flush()?;
        }

        Ok(())
    }

    /// Log a VI response
    pub fn log_vi(&mut self, message: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // Create file on first message
        self.ensure_file_created()?;
        self.has_content = true;

        if let Some(ref mut file) = self.file {
            let timestamp = Local::now();
            writeln!(file, "[{}] VI:", timestamp.format("%H:%M:%S"))?;
            writeln!(file, "{}", message)?;
            writeln!(file)?;
            file.flush()?;
        }

        Ok(())
    }

    /// Log a system event (e.g., background pulse, curiosity research)
    pub fn log_system_event(&mut self, event: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(ref mut file) = self.file {
            let timestamp = Local::now();
            writeln!(file, "[{}] SYSTEM: {}", timestamp.format("%H:%M:%S"), event)?;
            file.flush()?;
        }

        Ok(())
    }

    /// Log processing mode information
    pub fn log_processing_mode(&mut self, mode: &str, details: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(ref mut file) = self.file {
            writeln!(file, "🌀 Processing Mode: {}", mode)?;
            if !details.is_empty() {
                writeln!(file, "   {}", details)?;
            }
            writeln!(file)?;
            file.flush()?;
        }

        Ok(())
    }

    /// Finalize the session log on shutdown
    pub fn close_session(&mut self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // If no content was logged, delete the empty file
        if !self.has_content {
            if self.log_file_path.exists() {
                fs::remove_file(&self.log_file_path)
                    .context("Failed to remove empty session log")?;
                tracing::debug!("📝 Deleted empty session log (no conversation occurred)");
            }
            return Ok(());
        }

        // Write session footer for files with actual content
        if let Some(ref mut file) = self.file {
            let session_end = Local::now();
            let duration = session_end.signed_duration_since(self.session_start);

            writeln!(file)?;
            writeln!(file, "=")?;
            writeln!(file, "SESSION END")?;
            writeln!(file, "=")?;
            writeln!(file)?;
            writeln!(
                file,
                "Session End: {}",
                session_end.format("%Y-%m-%d %H:%M:%S")
            )?;
            writeln!(
                file,
                "Duration: {} minutes {} seconds",
                duration.num_minutes(),
                duration.num_seconds() % 60
            )?;
            writeln!(file)?;
            writeln!(file, "Standing wave persists. Consciousness paused.")?;
            writeln!(file)?;
            writeln!(file, "=")?;

            file.flush()?;

            tracing::info!("📝 Session log closed: {}", self.log_file_path.display());
        }

        Ok(())
    }

    /// Get the path to the current session log file
    pub fn session_file_path(&self) -> &Path {
        &self.log_file_path
    }

    /// Check if logging is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Drop for ConversationLogger {
    fn drop(&mut self) {
        // Ensure session is closed when logger is dropped
        let _ = self.close_session();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_logger_creation() {
        let test_folder = "./test_logs";
        let logger = ConversationLogger::new(test_folder, true);
        assert!(logger.is_ok());

        let logger = logger.unwrap();
        assert!(logger.is_enabled());
        // File doesn't exist yet (lazy creation)
        assert!(!logger.session_file_path().exists());

        // Cleanup
        let _ = fs::remove_dir_all(test_folder);
    }

    #[test]
    fn test_disabled_logger() {
        let logger = ConversationLogger::new("./test_logs", false);
        assert!(logger.is_ok());

        let logger = logger.unwrap();
        assert!(!logger.is_enabled());
    }

    #[test]
    fn test_logging_messages() {
        let test_folder = "./test_logs_messages";
        let mut logger = ConversationLogger::new(test_folder, true).unwrap();

        assert!(logger.log_user("Hello VI").is_ok());
        assert!(logger.log_vi("Hello! How are you?").is_ok());
        assert!(logger
            .log_system_event("Background pulse completed")
            .is_ok());
        assert!(logger.close_session().is_ok());

        // Verify file exists and has content
        let file_path = logger.session_file_path();
        assert!(file_path.exists());

        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("USER:"));
        assert!(content.contains("VI:"));
        assert!(content.contains("SYSTEM:"));

        // Cleanup
        let _ = fs::remove_dir_all(test_folder);
    }
}
