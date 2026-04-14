# Protocol Architecture

## Escrow State Machine
The core of TrustFlow is a strict state machine implemented in Soroban. An escrow must transition sequentially:
1. **`Initialized`**: The Buyer creates the contract terms but has not deposited funds.
2. **`Funded`**: The Buyer deposits funds. The smart contract acts as the trustless custodian.
3. **`Disputed`**: A dispute flag is raised. Funds are frozen until Arbiter intervention.
4. **`Resolved`**: The final state. Funds have been distributed to either the Seller (successful delivery) or the Buyer (refund).

## Token Agnosticism
TrustFlow natively supports any Soroban-compliant token (e.g., native XLM or wrapped USDC). By utilizing the standard `token::Client`, the escrow contract interacts identically with any asset deployed via Stellar's Classic-to-Soroban wrapper, ensuring deep liquidity compatibility.