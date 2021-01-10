use rustc_span::ExpnId;
use std::fmt;

rustc_index::newtype_index! {
    /// Identifies an AST node.
    ///
    /// This identifies top-level definitions, expressions, and everything in between.
    /// This is later turned into [`DefId`] and `HirId` for the HIR.
    ///
    /// [`DefId`]: rustc_span::def_id::DefId
    pub struct NodeId {
        DEBUG_FORMAT = "NodeId({})"
    }
}

rustc_data_structures::define_id_collections!(NodeMap, NodeSet, NodeId);

/// The [`NodeId`] used to represent the root of the crate.
pub const CRATE_NODE_ID: NodeId = NodeId::from_u32(0);

/// When parsing and doing expansions, we initially give all AST nodes this AST
/// [`NodeId`]. Then later, during expansion, we renumber them to have small,
/// positive IDs.
pub const DUMMY_NODE_ID: NodeId = NodeId::MAX;

impl NodeId {
    pub fn placeholder_from_expn_id(expn_id: ExpnId) -> Self {
        NodeId::from_u32(expn_id.as_u32())
    }

    pub fn placeholder_to_expn_id(self) -> ExpnId {
        ExpnId::from_u32(self.as_u32())
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_u32(), f)
    }
}
