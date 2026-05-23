# On-Chain Todo Manager

A decentralized task management smart contract deployed on the Stellar blockchain using the Soroban SDK. All todo data is stored directly on-chain — transparent, immutable, and trustless.

Only the admin wallet that initialized the contract can modify data. Anyone can read the todo list publicly.

---

## Features

- Admin-only write access via `require_auth()` — only the deployer's wallet can mutate data
- Create todos with title, description, and priority level (`Low` / `Medium` / `High`)
- Update task status: `Pending` → `InProgress` → `Done`
- Update priority of existing todos at any time
- Delete todos by ID
- Fetch all todos or a single todo by ID (public read access)
- Sequential, collision-free ID system using an on-chain counter
- Timestamps via `env.ledger().timestamp()` — trustless, no client input needed

---

## Data Structures

**Todo**
| Field | Type | Description |
|---|---|---|
| `id` | `u64` | Auto-incremented unique identifier |
| `title` | `String` | Short title of the task |
| `description` | `String` | Detailed description of the task |
| `priority` | `Priority` | `Low` \| `Medium` \| `High` |
| `status` | `Status` | `Pending` \| `InProgress` \| `Done` |
| `created_at` | `u64` | Ledger timestamp at creation time |

---

## Contract Functions

| Function | Auth | Description |
|---|---|---|
| `init(admin)` | — | Initialize contract and set admin wallet. One-time only. |
| `get_todos()` | — | Returns all todos stored on-chain. |
| `get_todo(id)` | — | Returns a single todo by ID, or `None` if not found. |
| `create_todo(title, description, priority)` | ✅ Admin | Creates a new todo with `Pending` status. |
| `update_status(id, new_status)` | ✅ Admin | Updates the status of an existing todo. |
| `update_priority(id, new_priority)` | ✅ Admin | Updates the priority of an existing todo. |
| `delete_todo(id)` | ✅ Admin | Deletes a todo by ID. |

---

## Deployment Info

| Property | Value |
|---|---|
| Network | Stellar Testnet |
| Contract ID | `CBTPU3DRPIK6HAYTJSO22ZE52EHSAVXPTBLFSTXONECYGBTZ754KLIAF` |
| Horizon URL | `https://horizon-testnet.stellar.org` |
| Stellar Lab | `https://lab.stellar.org` |
| Language | Rust (`no_std`) |
| SDK | Soroban SDK |

---

## Invoking via Stellar Lab

Go to [lab.stellar.org](https://lab.stellar.org), switch network to **Testnet**, paste the Contract ID above.

**Read (no signature needed)**
- `get_todos` — no arguments, click Simulate
- `get_todo` — argument: `id` → `u64` → e.g. `1`

**Write (requires admin wallet)**
- `create_todo` — `title` (String), `description` (String), `priority` (Low/Medium/High)
- `update_status` — `id` (u64), `new_status` (Pending/InProgress/Done)
- `update_priority` — `id` (u64), `new_priority` (Low/Medium/High)
- `delete_todo` — `id` (u64)

---

## Tech Stack

| Technology | Purpose |
|---|---|
| Rust (`no_std`) | Smart contract language |
| Soroban SDK | Stellar smart contract framework |
| Stellar Testnet | Blockchain deployment environment |
| Stellar Lab | Web-based contract interaction UI |