use crate::ProofOfWork;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,         // 区块时间戳
    pre_block_hash: String, // 上一区块的哈希值
    hash: String,           // 当前区块的哈希值
    data: String,           // 区块数据
    nonce: i64,             // 计数器
}

impl Block {
    /// 新建一个区块
    pub fn new_block(pre_block_hash: String, data: String) -> Block {
        let mut block = Block {
            timestamp: current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            data,
            nonce: 0,
        };
        // 挖矿计算哈希
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }

    /// 从字节数组反序列化
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    /// 生成创世块
    pub fn new_genesis_block() -> Block {
        return Block::new_block(String::new(), String::from("Genesis Block"));
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}

/// 获取当前时间戳，单位：ms
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn test_new_block() {
        let block = Block::new_block(
            String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
            String::from("ABC"),
        );
        println!("new block hash is {}", block.hash)
    }

    #[test]
    fn test_serialize() {
        let block = Block::new_block(
            String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
            String::from("ABC"),
        );
        let bytes = bincode::serialize(&block).unwrap();
        let desc_block = Block::deserialize(&bytes[..]);
        assert_eq!(block.data, desc_block.data)
    }
}
