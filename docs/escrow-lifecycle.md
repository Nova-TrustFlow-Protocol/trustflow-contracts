# TrustFlow Escrow Lifecycle

### 1. Agreement & Initialization
The Buyer initiates the workflow off-chain, negotiating terms with the Seller. The Buyer then calls `initialize_escrow`, recording the agreed-upon Token, Amount, and Arbiter to the ledger.

### 2. Securing Funds
The Buyer calls `fund_escrow`. The Soroban contract transfers the specified token amount from the Buyer's wallet into its own contract address.

### 3. Delivery & Resolution
* **Happy Path:** The Seller delivers the goods/services. The Buyer calls `approve_release`, pulling funds from the contract to the Seller.
* **Dispute Path:** If terms are breached, either party calls `raise_dispute`. The designated Arbiter reviews off-chain evidence and calls `resolve_dispute(award_to_seller)` to force the fund movement.