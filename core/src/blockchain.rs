use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(pre_block.hash.clone(), data);
        self.blocks.push(new_block);
    }

    pub fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is genesis blcok".to_string(), String::from(""))
    }

    pub fn new_blockchain() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}