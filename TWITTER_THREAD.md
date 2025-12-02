# Twitter Thread: LODE Project Security Analysis

## Thread 🧵

### Tweet 1/7 (Opening)
```
🚨 SECURITY ALERT: I analyzed the LODE mining project (github.com/lode-supply/lode-program) claiming to use "Entropy VRF" for fair randomness.

What I found is extremely concerning.

A thread on why this appears to be a SCAM 🧵👇
```

### Tweet 2/7 (The Fake VRF)
```
❌ EVIDENCE #1: Fake VRF Provider

The code claims to use "Entropy VRF" but look at constants.rs:

ENTROPY_PROGRAM_ID = "So11111111111111111111111111111111111111112"

This is NOT a VRF provider. This is the Wrapped SOL token address!

Real VRF providers:
- Switchboard: SW1TCH7q...
- ORAO: VRFzZoJ...
```

### Tweet 3/7 (The ORE Clone)
```
❌ EVIDENCE #2: It's a Clone

LODE copied the entire 5x5 grid mechanism from the legitimate ORE project (@HardhatChad).

✅ ORE: Uses real VRF, open source, deployed on mainnet
❌ LODE: Uses fake VRF address, untraceable, no mainnet presence

Side-by-side code comparison shows identical game logic.
```

### Tweet 4/7 (Missing Validations)
```
❌ EVIDENCE #3: No Security Checks

In reset.rs, the code ONLY checks if the address matches:
require!(entropy_var.key() == config.var_address)

Missing validations:
- No owner verification ❌
- No VRF proof verification ❌
- No program ID check ❌

This allows operators to provide ANY account as "randomness source"
```

### Tweet 5/7 (Admin Control)
```
❌ EVIDENCE #4: Admin Manipulation Vector

set_var_address.rs allows admin to change the VRF source to ANY address with ZERO validation:

/// CHECK: No validation performed
pub var: UncheckedAccount<'info>

Admin can:
1. Create fake Var account
2. Fill it with predetermined "random" values
3. Know winners before each round
4. Bet on winning squares
```

### Tweet 6/7 (The Attack)
```
💀 HOW THE SCAM WORKS:

1. Admin sets var_address to fake account (under their control)
2. Before each round, calculate: if value=X → square #12 wins
3. Fill fake account with value=X
4. Deploy SOL on square #12
5. Trigger reset() → reads fake "random" value
6. Square #12 "randomly" wins
7. Collect all losing squares' SOL

Repeat until rug pull.
```

### Tweet 7/7 (Call to Action)
```
🔍 VERIFY YOURSELF:

Code: github.com/lode-supply/lode-program/tree/ef4ca1e

Check:
- constants.rs line ~50: ENTROPY_PROGRAM_ID
- instructions/set_var_address.rs: No validation
- instructions/reset.rs: Only address check

Compare with legitimate ORE: github.com/regolith-labs/ore

⚠️ DO NOT INTERACT WITH THIS PROJECT

If you've seen this project promoted, report it immediately. This is not a legitimate VRF implementation.

#Solana #SecurityAlert #CryptoScam
```

---

## Alternative: Single Comprehensive Tweet

If you prefer a single impactful tweet instead of a thread:

```
🚨 SCAM ALERT: "LODE" mining project on Solana

Claims: "Fair launch with Entropy VRF"
Reality:
- ENTROPY_PROGRAM_ID = Wrapped SOL address (NOT a VRF provider!)
- Clone of legitimate ORE project
- Zero VRF validation
- Admin can manipulate "random" results

Evidence: github.com/lode-supply/lode-program/blob/ef4ca1e/programs/lode-program/src/constants.rs

Real VRF: Switchboard, ORAO
LODE VRF: Fake

DO NOT USE. This is fraud.

#Solana #CryptoScam
```

---

## Direct Challenge Tweet (More Aggressive)

If you want to directly challenge the developers:

```
@[LODE_DEVELOPER_HANDLE]

Your project claims to use "Entropy VRF" for randomness, but your code shows:

ENTROPY_PROGRAM_ID = "So11111111111111111111111111111111111111112"

This is the Wrapped SOL token address, NOT a VRF provider.

Can you explain:
1. Why use a fake VRF address?
2. Why no owner validation on entropy_var?
3. Why clone ORE but remove security?
4. Why is this project completely untraceable online?

Your set_var_address() function allows admin to point to ANY account with zero validation. This enables complete manipulation of "random" outcomes.

Prove me wrong with:
- Real mainnet deployment address
- Third-party security audit
- Explanation of your VRF integration

I've documented the full analysis here: [YOUR GITHUB REPO LINK]

The crypto community deserves transparency.
```

---

## Technical Callout Tweet (For Developer Audience)

```
For Solana devs: A case study in malicious VRF implementation 🧵

I found a project that CLAIMS VRF but actually:

```rust
// constants.rs
ENTROPY_PROGRAM_ID: pubkey!("So111...112") // ← Wrapped SOL!

// set_var_address.rs
/// CHECK: No validation performed
pub var: UncheckedAccount<'info>

// reset.rs
require!(entropy_var.key() == config.var_address) // ← Only this!
```

No owner check. No proof verification. Admin-controlled source.

This is how you DON'T integrate VRF.

Compare with proper implementation using Switchboard/ORAO where you MUST verify:
- Account owner == VRF_PROGRAM_ID
- VRF proof validity
- Callback authentication

Learn from this: Always verify the FULL trust chain, not just addresses.

#SolanaDev #Security
```

---

## Tips for Posting:

1. **Tag relevant accounts**:
   - @solana (official Solana)
   - @HardhatChad (ORE creator - they should know about this clone)
   - Any Solana security accounts

2. **Use hashtags**:
   - #Solana #SolanaScam #CryptoScam
   - #Web3Security #DeFiSecurity
   - #VRF #Blockchain

3. **Include visual evidence**:
   - Screenshot of the fake ENTROPY_PROGRAM_ID
   - Side-by-side comparison of ORE vs LODE
   - Diagram showing the manipulation flow

4. **Link to your analysis**:
   - Your GitHub repo with full analysis
   - Link to the LODE repo showing the problematic code

5. **Be prepared for responses**:
   - They may claim it's "not deployed yet"
   - They may say it's "placeholder code"
   - Stand firm on: "Why is a fake VRF address in production code?"

6. **Follow up with evidence**:
   - If they respond, ask for mainnet address
   - Ask for security audit
   - Ask why they cloned ORE

Would you like me to create any images/diagrams to accompany these tweets?
