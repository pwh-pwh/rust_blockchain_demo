use chrono::Utc;
use serde::Serialize;
use utils::coder;

#[derive(Serialize,Debug)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&self.header);
        self.hash = coder::get_hash(&header);
    }
    pub fn new_block(data: String, pre_hash: String) -> Block {
        let tx_hash = coder::get_hash(&coder::my_serialize(&data));
        let mut block = Block {
            header: BlockHeader {
                time: Utc::now().timestamp(),
                tx_hash,
                pre_hash,
            },
            hash: Default::default(),
            data,
        };
        block.set_hash();
        block
    }
}
