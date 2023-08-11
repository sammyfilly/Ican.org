use anyhow::Result;
use turbo_tasks::{primitives::BoolVc, ValueToString};
use turbopack_core::chunk::{ChunkItem, ChunkingContext, ChunkingContextVc, ModuleId, ModuleIdVc};

use super::item::EcmascriptChunkItemVc;

/// [`EcmascriptChunkingContext`] must be implemented by [`ChunkingContext`]
/// implementors that want to operate on [`EcmascriptChunk`]s.
#[turbo_tasks::value_trait]
pub trait EcmascriptChunkingContext: ChunkingContext {
    /// Whether chunk items generated by this chunking context should include
    /// the `__turbopack_refresh__` argument.
    fn has_react_refresh(&self) -> BoolVc {
        BoolVc::cell(false)
    }

    async fn chunk_item_id(&self, chunk_item: EcmascriptChunkItemVc) -> Result<ModuleIdVc> {
        let layer = self.layer();
        let mut ident = chunk_item.asset_ident();
        if !layer.await?.is_empty() {
            ident = ident.with_modifier(layer)
        }
        Ok(ModuleId::String(ident.to_string().await?.clone_value()).cell())
    }
}
