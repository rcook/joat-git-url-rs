// Copyright (c) 2020-3 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseGitUrlError(String);

impl Display for ParseGitUrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

impl StdError for ParseGitUrlError {}

#[derive(Clone)]
pub struct GitUrl {
    host: String,
    path: String,
}

impl GitUrl {
    const HTTP_PREFIX: &'static str = "http://";
    const HTTPS_PREFIX: &'static str = "https://";

    #[allow(dead_code)]
    pub fn pop(&self) -> Option<Self> {
        let mut temp = self.clone();
        match temp.pop_mut() {
            true => Some(temp),
            false => None,
        }
    }

    #[allow(dead_code)]
    pub fn pop_mut(&mut self) -> bool {
        Self::pop_helper(&mut self.path)
    }

    #[allow(dead_code)]
    pub fn join(&self, child_path: &str) -> Option<Self> {
        let mut temp = self.clone();
        match temp.join_mut(child_path) {
            true => Some(temp),
            false => None,
        }
    }

    #[allow(dead_code)]
    pub fn join_mut(&mut self, child_path: &str) -> bool {
        let mut path = self.path.clone();
        for part in child_path.split('/') {
            if part.is_empty() {
                return false;
            } else if part == ".." {
                if !Self::pop_helper(&mut path) {
                    return false;
                }
            } else if part != "." {
                if !path.is_empty() {
                    path += "/"
                }
                path += part
            }
        }
        self.path = path;
        true
    }

    fn pop_helper(path: &mut String) -> bool {
        if path.is_empty() {
            false
        } else {
            match path.rfind('/') {
                Some(pos) => path.truncate(pos),
                None => path.clear(),
            }
            true
        }
    }
}

impl FromStr for GitUrl {
    type Err = ParseGitUrlError;

    #[allow(clippy::manual_strip)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opt = if s.starts_with(Self::HTTP_PREFIX) {
            s[Self::HTTP_PREFIX.len()..].find('/').map(|p| Self {
                host: s[..Self::HTTP_PREFIX.len() + p].to_string(),
                path: s[Self::HTTP_PREFIX.len() + p + 1..].to_string(),
            })
        } else if s.starts_with(Self::HTTPS_PREFIX) {
            s[Self::HTTPS_PREFIX.len()..].find('/').map(|p| Self {
                host: s[..Self::HTTPS_PREFIX.len() + p].to_string(),
                path: s[Self::HTTPS_PREFIX.len() + p + 1..].to_string(),
            })
        } else {
            s.find(':').map(|p| Self {
                host: s[..p].to_string(),
                path: s[p + 1..].to_string(),
            })
        };
        opt.ok_or(ParseGitUrlError(String::from(s)))
    }
}

impl Display for GitUrl {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self.path.len() {
                0 => self.host.to_string(),
                _ => self.host.to_string() + ":" + &self.path,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{GitUrl, ParseGitUrlError};
    use std::result::Result as StdResult;

    #[test]
    fn test_pop_https() -> StdResult<(), ParseGitUrlError> {
        let x0 = "https://github.com/user/foo/bar/quux.git".parse::<GitUrl>()?;
        assert_eq!(x0.host, "https://github.com");
        assert_eq!(x0.path, "user/foo/bar/quux.git");

        let x1 = "http://github.com/user/foo/bar/quux.git".parse::<GitUrl>()?;
        assert_eq!(x1.host, "http://github.com");
        assert_eq!(x1.path, "user/foo/bar/quux.git");

        let x2 = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
        assert_eq!(x2.host, "git@github.com");
        assert_eq!(x2.path, "user/foo/bar/quux.git");

        Ok(())
    }

    #[test]
    fn test_pop() -> StdResult<(), ParseGitUrlError> {
        let x0 = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;

        assert_eq!(x0.host, "git@github.com");
        assert_eq!(x0.path, "user/foo/bar/quux.git");
        assert_eq!(x0.to_string(), "git@github.com:user/foo/bar/quux.git");

        let x1 = x0.pop().expect("pop failed");
        assert_eq!(x1.host, "git@github.com");
        assert_eq!(x1.path, "user/foo/bar");
        assert_eq!(x1.to_string(), "git@github.com:user/foo/bar");

        let x2 = x1.pop().expect("pop failed");
        assert_eq!(x2.host, "git@github.com");
        assert_eq!(x2.path, "user/foo");
        assert_eq!(x2.to_string(), "git@github.com:user/foo");

        let x3 = x2.pop().expect("pop failed");
        assert_eq!(x3.host, "git@github.com");
        assert_eq!(x3.path, "user");
        assert_eq!(x3.to_string(), "git@github.com:user");

        let x4 = x3.pop().expect("pop failed");
        assert_eq!(x4.host, "git@github.com");
        assert_eq!(x4.path, "");
        assert_eq!(x4.to_string(), "git@github.com");

        assert!(x4.pop().is_none());

        Ok(())
    }

    #[test]
    fn test_pop_mut() -> StdResult<(), ParseGitUrlError> {
        let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;

        assert_eq!(git_url.host, "git@github.com");
        assert_eq!(git_url.path, "user/foo/bar/quux.git");
        assert_eq!(git_url.to_string(), "git@github.com:user/foo/bar/quux.git");

        assert!(git_url.pop_mut());
        assert_eq!(git_url.host, "git@github.com");
        assert_eq!(git_url.path, "user/foo/bar");
        assert_eq!(git_url.to_string(), "git@github.com:user/foo/bar");

        assert!(git_url.pop_mut());
        assert_eq!(git_url.host, "git@github.com");
        assert_eq!(git_url.path, "user/foo");
        assert_eq!(git_url.to_string(), "git@github.com:user/foo");

        assert!(git_url.pop_mut());
        assert_eq!(git_url.host, "git@github.com");
        assert_eq!(git_url.path, "user");
        assert_eq!(git_url.to_string(), "git@github.com:user");

        assert!(git_url.pop_mut());
        assert_eq!(git_url.host, "git@github.com");
        assert_eq!(git_url.path, "");
        assert_eq!(git_url.to_string(), "git@github.com");

        assert!(!git_url.pop_mut());
        assert_eq!(git_url.host, "git@github.com");
        assert_eq!(git_url.path, "");
        assert_eq!(git_url.to_string(), "git@github.com");

        Ok(())
    }

    #[test]
    fn test_join() -> StdResult<(), ParseGitUrlError> {
        let git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;

        assert_eq!(
            git_url.join("aaa").expect("join failed").to_string(),
            "git@github.com:user/foo/bar/quux.git/aaa"
        );

        assert_eq!(
            git_url.join("aaa/bbb").expect("join failed").to_string(),
            "git@github.com:user/foo/bar/quux.git/aaa/bbb"
        );

        assert_eq!(
            git_url.join(".").expect("join failed").to_string(),
            "git@github.com:user/foo/bar/quux.git"
        );

        assert_eq!(
            git_url.join("..").expect("join failed").to_string(),
            "git@github.com:user/foo/bar"
        );

        assert_eq!(
            git_url.join("../aaa").expect("join failed").to_string(),
            "git@github.com:user/foo/bar/aaa"
        );

        assert_eq!(
            git_url.join("../aaa/bbb").expect("join failed").to_string(),
            "git@github.com:user/foo/bar/aaa/bbb"
        );

        assert_eq!(
            git_url
                .join("../../../aaa/bbb")
                .expect("join failed")
                .to_string(),
            "git@github.com:user/aaa/bbb"
        );

        assert_eq!(
            git_url
                .join("../../../../aaa/bbb")
                .expect("join failed")
                .to_string(),
            "git@github.com:aaa/bbb"
        );

        assert!(git_url.join("/aaa").is_none());

        Ok(())
    }

    #[test]
    fn test_join_mut() -> StdResult<(), ParseGitUrlError> {
        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("aaa"));
            assert_eq!(
                git_url.to_string(),
                "git@github.com:user/foo/bar/quux.git/aaa"
            )
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("aaa/bbb"));
            assert_eq!(
                git_url.to_string(),
                "git@github.com:user/foo/bar/quux.git/aaa/bbb"
            )
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("."));
            assert_eq!(git_url.to_string(), "git@github.com:user/foo/bar/quux.git")
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut(".."));
            assert_eq!(git_url.to_string(), "git@github.com:user/foo/bar")
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("../aaa"));
            assert_eq!(git_url.to_string(), "git@github.com:user/foo/bar/aaa")
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("../aaa/bbb"));
            assert_eq!(git_url.to_string(), "git@github.com:user/foo/bar/aaa/bbb")
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("../../../aaa/bbb"));
            assert_eq!(git_url.to_string(), "git@github.com:user/aaa/bbb")
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(git_url.join_mut("../../../../aaa/bbb"));
            assert_eq!(git_url.to_string(), "git@github.com:aaa/bbb")
        }

        {
            let mut git_url = "git@github.com:user/foo/bar/quux.git".parse::<GitUrl>()?;
            assert!(!git_url.join_mut("/aaa"))
        }

        Ok(())
    }
}
