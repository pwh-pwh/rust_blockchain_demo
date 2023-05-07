use core::blockchain;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();
    bc.add_block("b=>c 3btc".into());
    for b in bc.blocks {
        println!("{:#?}", b);
    }
}
