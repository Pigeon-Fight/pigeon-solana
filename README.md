# Run localnet

```bash
solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s ./programs/mpl_metadata.so --bpf-program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA ./programs/spl_token.so
```

# Switch between networks

```bash
solana config set --url http://127.0.0.1:8899
solana config set --url https://api.devnet.solana.com
solana airdrop 10
```

# Compile And Deploy

```bash
anchor build
anchor deploy
PROGRAM_ID=96khaF5rSYhuyWKFFGsMBjZtg49pSJAoSyxKN5FAjWEp
anchor idl init --filepath ./target/idl/pigeon_battle.json $PROGRAM_ID --provider.cluster devnet
```
