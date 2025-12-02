# Code Evidence: Fake VRF in LODE Project

## 🔴 THE SMOKING GUN

### Location: `programs/lode-program/src/constants.rs`

**Lines showing the fake VRF:**

```rust
// Line ~48-52 in constants.rs
pub const MINT_ADDRESS: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

// Line ~54-58 in constants.rs
pub const ENTROPY_PROGRAM_ID: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
```

**Direct link to code:**
https://github.com/lode-supply/lode-program/blob/ef4ca1e6ee89a7a4e3d3918e58d9fbcabc3724ce/programs/lode-program/src/constants.rs#L48-L58

---

## 🚨 What This Address Actually Is

**Address:** `So11111111111111111111111111111111111111112`

**Real identity:**
- **Wrapped SOL (wSOL)** - Native SPL Token Program
- **NOT a VRF provider**
- Used for wrapping native SOL into SPL token format

**Verify yourself:**
- Solana Explorer: https://explorer.solana.com/address/So11111111111111111111111111111111111111112
- Solscan: https://solscan.io/token/So11111111111111111111111111111111111111112
- Description: "Wrapped SOL"

---

## ✅ What REAL VRF Addresses Look Like

### Legitimate VRF Providers on Solana:

**Switchboard VRF:**
```
Program ID: SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f
```
- Website: switchboard.xyz
- Docs: docs.switchboard.xyz

**ORAO VRF:**
```
Program ID: VRFzZoJdhFWL8rkvu87LpKM3RbcVezpMEc6X5GVDr7y
```
- Website: orao.network
- GitHub: github.com/orao-network/solana-vrf

---

## 📊 Side-by-Side Comparison

| Project | VRF Address | Valid? |
|---------|-------------|--------|
| **LODE** | `So111...112` (Wrapped SOL) | ❌ FAKE |
| **Switchboard** | `SW1TCH...64f` | ✅ Real VRF |
| **ORAO** | `VRFzZ...7y` | ✅ Real VRF |

---

## 💀 How This is Used in the Code

### Step 1: Constants defined (constants.rs)
```rust
pub const ENTROPY_PROGRAM_ID: Pubkey = pubkey!("So111...112");
```

### Step 2: Used in reset function (instructions/reset.rs)
```rust
// The code checks if var account matches config address
require!(
    entropy_var.key() == ctx.accounts.config.var_address,
    AppError::InvalidEntropyVar
);

// ❌ BUT NEVER CHECKS:
// require!(entropy_var.owner == ENTROPY_PROGRAM_ID);
```

### Step 3: Admin sets var_address (instructions/set_var_address.rs)
```rust
/// CHECK: No validation performed  // ← RED FLAG!
pub var: UncheckedAccount<'info>

pub fn set_var_address(ctx: Context<SetVarAddress>) -> Result<()> {
    ctx.accounts.config.var_address = ctx.accounts.var.key();
    // ❌ No validation on what 'var' actually is
    Ok(())
}
```

---

## 🎯 The Problem

**What should happen:**
```rust
// Verify the var account is owned by a real VRF program
require!(
    entropy_var.owner == &SWITCHBOARD_PROGRAM_ID ||
    entropy_var.owner == &ORAO_PROGRAM_ID,
    AppError::InvalidVrfProvider
);

// Verify the VRF proof
verify_vrf_proof(&entropy_var, &proof)?;
```

**What actually happens:**
```rust
// Only checks if address matches - anyone can create this
require!(entropy_var.key() == config.var_address);

// config.var_address can be set to ANY address by admin
// No owner check ❌
// No proof verification ❌
```

---

## 📷 Screenshot Evidence (for Twitter)

**File locations to screenshot:**

1. **Fake VRF constant:**
   - File: `programs/lode-program/src/constants.rs`
   - Lines: ~48-58
   - Shows: `ENTROPY_PROGRAM_ID = So111...112`

2. **No validation:**
   - File: `programs/lode-program/src/instructions/set_var_address.rs`
   - Shows: `/// CHECK: No validation performed`

3. **Address-only check:**
   - File: `programs/lode-program/src/instructions/reset.rs`
   - Shows: Only `entropy_var.key() == config.var_address`

---

## 🔍 How to Verify This Yourself

### Step 1: Check the constant
```bash
curl https://raw.githubusercontent.com/lode-supply/lode-program/ef4ca1e/programs/lode-program/src/constants.rs | grep -A2 "ENTROPY_PROGRAM_ID"
```

**Expected output:**
```
pub const ENTROPY_PROGRAM_ID: Pubkey =
    pubkey!("So11111111111111111111111111111111111111112");
```

### Step 2: Verify what this address is
```bash
solana account So11111111111111111111111111111111111111112
```

**Expected output:**
```
Program Id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
(This is the SPL Token Program - Wrapped SOL)
```

### Step 3: Compare with real VRF
```bash
solana account SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f
```

**Expected output:**
```
Program Id: BPFLoaderUpgradeab1e11111111111111111111111
(This is a real VRF program)
```

---

## 📌 For Your Twitter Post

**Quote this exact code:**

```rust
// From LODE's constants.rs
pub const ENTROPY_PROGRAM_ID: Pubkey =
    pubkey!("So11111111111111111111111111111111111111112");

// This is Wrapped SOL, not a VRF provider!
// Verify: solscan.io/token/So11111111111111111111111111111111111111112
```

**Direct link:**
`github.com/lode-supply/lode-program/blob/ef4ca1e/programs/lode-program/src/constants.rs`

**One-line summary:**
"LODE claims VRF but uses Wrapped SOL address as ENTROPY_PROGRAM_ID"
