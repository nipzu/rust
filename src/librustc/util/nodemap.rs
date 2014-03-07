// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An efficient hash map for node IDs

use collections::{HashMap, HashSet};
use std::hash::{Hasher, Hash};
use std::io;
use syntax::ast;

#[cfg(not(stage0))]
pub type NodeMap<T> = HashMap<ast::NodeId, T, FnvHasher>;
#[cfg(not(stage0))]
pub type DefIdMap<T> = HashMap<ast::DefId, T, FnvHasher>;
#[cfg(not(stage0))]
pub type NodeSet = HashSet<ast::NodeId, FnvHasher>;
#[cfg(not(stage0))]
pub type DefIdSet = HashSet<ast::DefId, FnvHasher>;

// Hacks to get good names
#[cfg(not(stage0))]
pub mod NodeMap {
    use collections::HashMap;
    pub fn new<T>() -> super::NodeMap<T> {
        HashMap::with_hasher(super::FnvHasher)
    }
}
#[cfg(not(stage0))]
pub mod NodeSet {
    use collections::HashSet;
    pub fn new() -> super::NodeSet {
        HashSet::with_hasher(super::FnvHasher)
    }
}
#[cfg(not(stage0))]
pub mod DefIdMap {
    use collections::HashMap;
    pub fn new<T>() -> super::DefIdMap<T> {
        HashMap::with_hasher(super::FnvHasher)
    }
}
#[cfg(not(stage0))]
pub mod DefIdSet {
    use collections::HashSet;
    pub fn new() -> super::DefIdSet {
        HashSet::with_hasher(super::FnvHasher)
    }
}

#[cfg(stage0)]
pub type NodeMap<T> = HashMap<ast::NodeId, T>;
#[cfg(stage0)]
pub type DefIdMap<T> = HashMap<ast::DefId, T>;
#[cfg(stage0)]
pub type NodeSet = HashSet<ast::NodeId>;
#[cfg(stage0)]
pub type DefIdSet = HashSet<ast::DefId>;

// Hacks to get good names
#[cfg(stage0)]
pub mod NodeMap {
    use collections::HashMap;
    pub fn new<T>() -> super::NodeMap<T> {
        HashMap::new()
    }
}
#[cfg(stage0)]
pub mod NodeSet {
    use collections::HashSet;
    pub fn new() -> super::NodeSet {
        HashSet::new()
    }
}
#[cfg(stage0)]
pub mod DefIdMap {
    use collections::HashMap;
    pub fn new<T>() -> super::DefIdMap<T> {
        HashMap::new()
    }
}
#[cfg(stage0)]
pub mod DefIdSet {
    use collections::HashSet;
    pub fn new() -> super::DefIdSet {
        HashSet::new()
    }
}

/// A speedy hash algorithm for node ids and def ids. The hashmap in
/// libcollections by default uses SipHash which isn't quite as speedy as we
/// want. In the compiler we're not really worried about DOS attempts, so we
/// just default to a non-cryptographic hash.
///
/// This uses FNV hashing, as described here:
/// http://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
#[deriving(Clone)]
pub struct FnvHasher;

pub struct FnvState(u64);

impl Hasher<FnvState> for FnvHasher {
    fn hash<T: Hash<FnvState>>(&self, t: &T) -> u64 {
        let mut state = FnvState(0xcbf29ce484222325);
        t.hash(&mut state);
        let FnvState(ret) = state;
        return ret;
    }
}

impl Writer for FnvState {
    fn write(&mut self, bytes: &[u8]) -> io::IoResult<()> {
        let FnvState(mut hash) = *self;
        for byte in bytes.iter() {
            hash = hash * 0x100000001b3;
            hash = hash ^ (*byte as u64);
        }
        *self = FnvState(hash);
        Ok(())
    }
}
