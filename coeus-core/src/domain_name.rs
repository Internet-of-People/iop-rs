use super::*;
use std::fmt::Write;

/// Contains lowercase letters and numbers. Especially does not contain '.' edge separator character.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Edge(String);

impl Edge {
    pub fn new(name: impl AsRef<str>) -> Result<Self> {
        let inner = name.as_ref().to_owned();
        Self::validate_charset(&inner)?;
        Ok(Self(inner))
    }

    fn validate_charset(name: &str) -> Result<()> {
        ensure!(!name.is_empty(), "Edge name cannot be empty");
        ensure!(name.is_ascii(), "Edge name must only contain ASCII characters");
        let valid = name.chars().all(Self::is_lowercase_alphanumeric);
        ensure!(valid, "Edge name must contain only lowercase alphanumeric ASCII characters");
        Ok(())
    }

    fn is_lowercase_alphanumeric(c: char) -> bool {
        ('a' <= c && c <= 'z') || ('0' <= c && c <= '9')
    }
}

impl AsRef<str> for Edge {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DomainName {
    edges: Vec<Edge>,
}

impl DomainName {
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    pub fn iter(&self) -> std::slice::Iter<Edge> {
        self.edges.iter()
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn parent(&self) -> Option<DomainName> {
        let edge_count = self.edges.len();
        if edge_count == 0 {
            None
        } else {
            Some(DomainName::new(self.edges[..edge_count - 1].to_owned()))
        }
    }

    pub fn last_edge(&self) -> Option<&Edge> {
        self.edges.iter().last()
    }

    pub fn is_root(&self) -> bool {
        self.edges.is_empty()
    }
}

impl fmt::Display for DomainName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.edges {
            f.write_char('.')?;
            f.write_str(e.as_ref())?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for DomainName {
    type Error = anyhow::Error;
    fn try_from(name: &str) -> Result<Self> {
        let parts = name.split('.').collect::<Vec<_>>();
        ensure!(!parts.is_empty(), "Split works strange");
        ensure!(parts[0].is_empty(), "DomainName must be absolute and start with '.'");
        let mut edges = vec![];
        for e in &parts[1..] {
            edges.push(Edge::new(*e)?);
        }
        Ok(Self::new(edges))
    }
}

impl FromStr for DomainName {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edge_charset() {
        assert!(Edge::new("r2d2").is_ok());
        assert!(Edge::new("R2-D2").is_err());
        assert!(Edge::new("c3po").is_ok());
        assert!(Edge::new("C-3PO").is_err());

        assert!(Edge::new("johndoe").is_ok());
        assert!(Edge::new("johndöe").is_err());
        assert!(Edge::new("johnDoe").is_err());
        assert!(Edge::new("🍺🍷🍶").is_err());
        assert!(Edge::new("john doe").is_err());
        assert!(Edge::new("john.doe").is_err());
        assert!(Edge::new("john-doe").is_err());
        assert!(Edge::new("john_doe").is_err());
    }

    fn wallet_of_joe() -> DomainName {
        DomainName::new(vec![Edge::new("wallet").unwrap(), Edge::new("joe").unwrap()])
    }

    fn wallet() -> DomainName {
        DomainName::new(vec![Edge::new("wallet").unwrap()])
    }

    #[test]
    fn parse_absolute() {
        let name = DomainName::try_from(".wallet.joe").unwrap();
        assert_eq!(name, wallet_of_joe());
    }

    #[test]
    fn parse_relative_fails() {
        let err = DomainName::try_from("wallet.joe").unwrap_err();
        assert_eq!(err.to_string(), "DomainName must be absolute and start with '.'");
    }

    #[test]
    fn parent_of_joe() {
        let path = wallet_of_joe();
        let parent = path.parent();
        assert_eq!(parent, Some(wallet()));
    }

    #[test]
    fn parent_of_wallet() {
        let path = wallet();
        let parent = path.parent();
        assert_eq!(parent, Some(DomainName::new(vec![])));
    }

    #[test]
    fn parent_of_root_fails() {
        let path = DomainName::new(vec![]);
        assert!(path.parent().is_none());
    }
}
