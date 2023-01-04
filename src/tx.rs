
#[derive(Serialize, Deserialize, Debug, Copy, PartialEq, Clone, TryFromByte)]
pub enum TransactionType {
    SimpleTransfer, //1 sender, 1 receiver
    //Fee,
    //VerificationAsset,
    //Issuance,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    //timestamp: u64,    
    //#[serde(with = "serde_bytes")]
    //message: Vec<u8>,
    //transaction_type: TransactionType,
    //#[serde_as(as = "[_; 64]")]
    //signature: HrzSignature,
    //path: Vec<Hop>,
    //hash_for_signature: Option<Hash>,
    //pub total_fees: u64,
    
}

impl Transaction {

}