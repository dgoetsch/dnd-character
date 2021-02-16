use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FeaturePath {
    path: Vec<String>,
    include_children: bool,
}

impl Display for FeaturePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tail = if self.include_children { "..." } else { "" };
        write!(f, "{}{}", self.path.join(" "), tail)
    }
}

impl FeaturePath {
    pub fn include_children(&self) -> bool {
        self.include_children
    }
    pub fn of(path: Vec<String>) -> FeaturePath {
        FeaturePath {
            path,
            include_children: false,
        }
    }
    pub fn empty() -> FeaturePath {
        FeaturePath {
            path: vec![],
            include_children: false,
        }
    }
    pub fn matches(&self, feature: String) -> (bool, FeaturePath) {
        let mut path = self.path.clone();
        let head = path.get(0).map(|s| s.clone());
        match head {
            Some(head) => {
                path.remove(0);
                if (feature == head) {
                    (
                        true,
                        FeaturePath {
                            path,
                            include_children: self.include_children,
                        },
                    )
                } else {
                    (
                        false,
                        FeaturePath {
                            path: vec![],
                            include_children: false,
                        },
                    )
                }
            }
            None => (self.include_children, self.clone()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn with_child(&self, child: String) -> FeaturePath {
        let mut path = self.path.clone();
        path.push(child);
        FeaturePath {
            path,
            include_children: self.include_children,
        }
    }

    pub fn with_include_children(&self, include: bool) -> FeaturePath {
        FeaturePath {
            path: self.path.clone(),
            include_children: include,
        }
    }
}
