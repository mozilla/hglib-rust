use crate::*;

pub struct Arg {}

impl Default for Arg {
    fn default() -> Self {
        Self {}
    }
}

impl Arg {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(client, "bookmarks", &[""])
    }
}

#[derive(Debug, PartialEq)]
pub struct Bookmark {
    pub name: String,
    pub rev: u64,
    pub node: String,
}

#[derive(Debug, PartialEq)]
pub struct Bookmarks {
    pub bookmarks: Vec<Bookmark>,
    pub current: Option<usize>,
}

impl Client {
    pub fn bookmarks(&mut self, x: Arg) -> Result<Bookmarks, HglibError> {
        let (data, _) = x.run(self)?;
        debug_vec!(data);
        let empty = b"no bookmarks set";
        let mut bookmarks = Vec::new();
        let mut current = None;

        if data.starts_with(empty)
            && data[empty.len()..]
                .iter()
                .all(|x| *x == b' ' || *x == b'\n')
        {
            return Ok(Bookmarks { bookmarks, current });
        }

        for line in data.split(|x| *x == b'\n').filter(|x| x.len() >= 3) {
            let start = unsafe { line.get_unchecked(..3) };
            if start.iter().any(|x| *x == b'*') {
                current = Some(bookmarks.len());
            }
            let line = unsafe { line.get_unchecked(3..) };
            let mut iter = line.split(|x| *x == b' ').filter(|x| !x.is_empty());
            let name = String::from_utf8(iter.next().unwrap().to_vec())?;
            let rev_node = iter.next().unwrap();
            let iter = &mut rev_node.iter();
            let rev = iter
                .take_while(|x| **x != b':')
                .fold(0, |r, x| r * 10 + u64::from(*x - b'0'));
            let node = iter.as_slice();
            let node = String::from_utf8(node.to_vec())?;
            bookmarks.push(Bookmark { name, rev, node });
        }

        Ok(Bookmarks { bookmarks, current })
    }
}
