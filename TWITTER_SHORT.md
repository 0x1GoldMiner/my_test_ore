# Short Twitter Posts - LODE Scam Alert

## Option 1: Direct & Technical (Recommended)

```
🚨 SCAM ALERT: LODE mining project claims "Entropy VRF" for fair randomness.

THE PROOF (constants.rs, line ~54):
pub const ENTROPY_PROGRAM_ID: Pubkey =
  pubkey!("So11111111111111111111111111111111111111112");

This is Wrapped SOL token, NOT a VRF provider.

Real VRF providers:
✅ Switchboard: SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f
✅ ORAO: VRFzZoJdhFWL8rkvu87LpKM3RbcVezpMEc6X5GVDr7y
❌ LODE: So111...112 (Wrapped SOL!)

Code: github.com/lode-supply/lode-program/blob/ef4ca1e/programs/lode-program/src/constants.rs

Verify Wrapped SOL: solscan.io/token/So11111111111111111111111111111111111111112

Admin can manipulate all "random" results.

DO NOT USE.
```

## Option 2: Question Format (More Engaging)

```
Question for LODE developers:

constants.rs line 54:
ENTROPY_PROGRAM_ID = "So11111111111111111111111111111111111111112"

This is Wrapped SOL (verify: solscan.io/token/So111...112)

Real VRF:
- Switchboard: SW1TCH...
- ORAO: VRFzZ...

How is Wrapped SOL a VRF provider?

Code: github.com/lode-supply/lode-program/blob/ef4ca1e/programs/lode-program/src/constants.rs

Your set_var_address() has /// CHECK: No validation performed

Explain or this is fraud.
```

## Option 3: Warning Only (Shortest)

```
⚠️ WARNING: "LODE" Solana mining project

Fake VRF: Uses Wrapped SOL address instead of real VRF provider
No validation: Admin controls "random" outcomes
Clone: Copied from legitimate ORE project

Evidence: github.com/lode-supply/lode-program (check constants.rs)

This is designed to steal your SOL.
```

## Option 4: Call Out with @mention

```
@[LODE_DEV_HANDLE]

Your ENTROPY_PROGRAM_ID = So11111111111111111111111111111111111111112

That's Wrapped SOL, not a VRF.

Why?
- No owner validation
- Admin controls var_address
- Copied ORE but removed security

You can manipulate every "random" result.

Explain or this is fraud.
```

## Option 5: Three-Tweet Thread (Minimal)

**Tweet 1:**
```
🚨 LODE mining project analysis:

Claims: "Fair random VRF"
Reality: Fake VRF address (Wrapped SOL token)

The code allows operators to control "random" outcomes and steal user funds.

Thread 👇
```

**Tweet 2:**
```
Proof:

ENTROPY_PROGRAM_ID in their code = "So111...112" (Wrapped SOL)

Real VRF providers:
• Switchboard: SW1TCH7q...
• ORAO: VRFzZoJ...

set_var_address() has /// CHECK: No validation performed

Admin can use any fake account as randomness source.
```

**Tweet 3:**
```
How the scam works:

1. Admin sets fake Var account
2. Fills it with chosen "random" value
3. Knows winning square before round starts
4. Bets on winner
5. Takes all losing players' SOL

Code: github.com/lode-supply/lode-program

This is fraud. Don't use.
```
