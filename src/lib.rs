extern crate regex;
#[derive(Debug, Clone)]
pub struct Document {
    /// Text before the first headline in the document also belongs to a
    /// section.
    first_section: Option<Section>,
    headlines: Vec<Headline>,
}

/// A headline contains directly one section (optionally), followed by any
/// number of deeper level headlines.
///
/// Syntactically, a headline is defined as:
///
/// ```ignore
/// STARS KEYWORD PRIORITY TITLE TAGS
/// ```
///
/// - STARS is a string starting at column 0, containing at least one asterisk
/// (and up to org-inlinetask-min-level if org-inlinetask library is loaded) and
/// ended by a space character. The number of asterisks is used to define the
/// level of the headline. It's the sole compulsory part of a headline.
///
/// - KEYWORD is a TODO keyword, which has to belong to the list defined in
/// org-todo-keywords-1. Case is significant.
///
/// - PRIORITY is a priority cookie, i.e. a single letter preceded by a hash
/// sign # and enclosed within square brackets.
///
/// - TITLE can be made of any character but a new line. Though, it will match
/// after every other part have been matched.
///
/// - TAGS is made of words containing any alpha-numeric character, underscore,
/// at sign, hash sign or percent sign, and separated with colons.
#[derive(Debug, Clone)]
struct Headline {
    level: u32,
    keyword: Option<String>,
    priority: Option<char>,
    title: String,
    tags: Vec<String>,
    section: Option<Section>,
    headlines: Vec<Headline>,
}

/// A section contains directly any greater element or element. Only a headline
/// can contain a section.
#[derive(Debug, Clone)]
struct Section {
    contents: Vec<GreaterElement>,
}

// @Todo: Implement greater elements
#[allow(unused)]
#[derive(Debug, Clone)]
enum GreaterElement {
    Block,
    Drawer,
    DynamicBlock,
    Footnote,
    Inlinetask,
    PlainList,
    PropertyDrawer,
    Table,
}

#[allow(unused)]
#[derive(Debug, Clone)]
enum Element {
    BabelCall,
    Block,
    Planning,
}

struct DocumentParser {
    todo_keywords: Vec<String>,
}

impl DocumentParser {
    pub fn new() -> Self {
        DocumentParser {
            todo_keywords: Vec::new(),
        }
    }

    pub fn todo_keywords<S: Into<String>>(mut self, keywords: Vec<S>) -> Self {
        self.todo_keywords = keywords.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn parse(&self, text: &str) -> Result<Document, ()> {
        let headline_matcher =
            regex::Regex::new(r"(?mx)
^(\*+)\s                     # STARS
(?:(\S+)\s                   # KEYWORD
   \[\#(.)\]\s)?             # PRIORITY
(.*?)\s*                     # TITLE
(:(?:[a-zA-Z0-9_@\#%]+:)+)?  # TAGS
$");
        // println!("{:?}", headline_matcher);
        let headline_matcher = headline_matcher.unwrap();
        let mut headlines = Vec::new();
        for headline in headline_matcher.captures_iter(text) {
            let stars = &headline[1];
            let priority = headline.get(3)
                .map(|x| text[x.start()..x.end()].chars().next().unwrap());
            let mut title: String = headline.get(4)
                .map(|x| text[x.start()..x.end()].trim().into())
                .unwrap_or_default();
            let keyword = match headline.get(2).map(|x| &text[x.start()..x.end()]) {
                None => {
                    let mut keyword_out = None;
                    for keyword in &self.todo_keywords {
                        if title.starts_with(keyword) {
                            keyword_out = Some(keyword.clone());
                            break;
                        }
                    }
                    if let Some(ref kwd) = keyword_out {
                        title = title[kwd.len()..].trim().into();
                    }
                    keyword_out
                }
                Some(kwd) => Some(kwd.into()),
            };
            let tags: Vec<_> = headline.get(5)
                .map(|x| &text[x.start()..x.end()])
                .map(|x| x[1..x.len()-1].split(':').map(String::from).collect())
                .unwrap_or_default();
            headlines.push(Headline {
                level: stars.len() as u32,
                priority: priority,
                keyword: keyword,
                title: title,
                tags: tags,
                section: None,
                headlines: Vec::new(),
            });
        }

        // @Todo: Reorganize the sections hierarchically
        // @Todo: Start parsing the sections

        Ok(Document {
            first_section: None,
            headlines: headlines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_parser() {
        println!();
        println!("{:#?}", DocumentParser::new().parse("* Hello!
** This is a second heading

Let's write a "));

        println!("{:#?}", DocumentParser::new()
            .todo_keywords(vec!["TODO", "DONE"]).parse("*

** DONE

*** Some e-mail

**** TODO [#A] COMMENT Title :tag:a2%:"));


        println!("{:#?}", DocumentParser::new().parse("An introduction.

* A Headline

  Some text.

** Sub-Topic 1

** Sub-Topic 2

*** Additional entry"));

        assert!(false);
    }
}
