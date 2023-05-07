use crate::block::Block;

#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis_block() -> Block {
        Block::new_block("new genesis block".into(), "".into())
    }

    pub fn new_blockchain() -> Self {
        Self {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}
