//! Help module - embedded multilingual help documentation

use e_client_config::language::Language;

/// A single help section with its subsections
#[derive(Debug, Clone)]
pub struct HelpSection {
    pub title: String,
    pub content: String, // The full markdown content for this section
    pub subsections: Vec<HelpTopic>,
}

/// A help topic (subsection)
#[derive(Debug, Clone)]
pub struct HelpTopic {
    pub title: String,
    pub content: String, // Markdown content for this subsection
}

/// Get help content for the specified language
pub fn get_help_content(lang: Language) -> &'static str {
    match lang {
        Language::Chinese => include_str!("content/help.zh.md"),
        Language::English => include_str!("content/help.en.md"),
    }
}

/// Parse help content into sections (H2 blocks)
pub fn get_help_sections(lang: Language) -> Vec<HelpSection> {
    let content = get_help_content(lang);
    let mut sections = Vec::new();

    // Split content by ## headers
    let mut current_title = String::new();
    let mut current_content = String::new();
    let mut subsections = Vec::new();

    for line in content.lines() {
        if line.starts_with("## ") {
            // Save previous section if exists
            if !current_title.is_empty() {
                sections.push(HelpSection {
                    title: current_title.clone(),
                    content: current_content.trim().to_string(),
                    subsections: subsections.clone(),
                });
                current_content.clear();
                subsections.clear();
            }

            // Start new section
            current_title = line.trim_start_matches("## ").trim().to_string();
        } else if line.starts_with("### ") {
            // Subsection - save to list and append to content
            let sub_title = line.trim_start_matches("### ").trim().to_string();
            subsections.push(HelpTopic {
                title: sub_title,
                content: String::new(),
            });
            current_content.push_str(line);
            current_content.push('\n');
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Don't forget the last section
    if !current_title.is_empty() {
        sections.push(HelpSection {
            title: current_title,
            content: current_content.trim().to_string(),
            subsections,
        });
    }

    sections
}

/// Get all main section titles for the sidebar
pub fn get_main_topics(lang: Language) -> Vec<String> {
    get_help_sections(lang)
        .into_iter()
        .map(|s| s.title)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_help_sections() {
        let sections = get_help_sections(Language::English);
        assert!(!sections.is_empty());
        assert!(sections.len() >= 8); // Should have 8 main sections

        // Check first section
        let first = &sections[0];
        assert!(first.title.contains("Overview") || first.title.contains("概述"));
    }

    #[test]
    fn test_get_main_topics() {
        let topics = get_main_topics(Language::English);
        assert_eq!(topics.len(), 8);
    }
}
