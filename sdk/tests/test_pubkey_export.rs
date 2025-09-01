// Simple test to make sure we haven't broken the re-export of the pubkey macro in atlas_sdk
#[test]
fn test_sdk_pubkey_export() {
    assert_eq!(
        atlas_sdk::pubkey!("ZkTokenProof1111111111111111111111111111111"),
        atlas_pubkey::pubkey!("ZkTokenProof1111111111111111111111111111111")
    );
}
