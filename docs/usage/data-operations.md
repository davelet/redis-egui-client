# Data Operations

## Database Operations

### Switching Databases

After connecting, use the **DB** dropdown in the top panel to switch databases (0-15).

### Browsing Keys

The **left sidebar** displays all keys in the current database:

- **Load Statistics**: Shows loaded keys / total keys count
- **Scroll Loading**: Loads keys in batches, automatically loads more when scrolling
- **Load All**: Click the load button to load all keys at once (use with caution for large datasets)

### Key Filtering

Enter keywords in the filter box at the top of the left sidebar:

- Supports `*` wildcard (e.g., `user:*` matches all user-related keys)
- Automatically adds wildcards to both ends if no `*` is present
- Clear the filter to display all keys

The actual SCAN is **debounced** — it fires shortly after you stop typing rather than on every keystroke, so rapid typing is collapsed into a single Redis round-trip. The delay is configurable in **Settings → Display → Filter debounce** (0–2000 ms, default 300 ms; 0 = fire on every keystroke). The pending window is rendered as a "Loading" state and the **Load More** button is suppressed while pending.

### Group Keys by Colon

Keys can be organized in a tree structure based on the colon separator. For example, `user:123:profile` would be displayed as:

```
user/
└── 123/
    └── profile
```

This makes it easier to navigate keys with hierarchical naming conventions.

## Key Operations

### Viewing Key Values

1. Click a key name in the left sidebar
2. The central panel displays detailed key information:
   - **Key Name** (copyable)
   - **Data Type** (String, List, Hash, Set, ZSet)
   - **TTL** expiration time
   - **Value Content**

### Data Type Support

| Type | Display Format | Special Features |
|------|----------------|------------------|
| **String** | Text editor | View full content directly |
| **List** | Indexed list | Lazy loading, click elements to edit |
| **Hash** | Field-value table | Field filter search, field-level editing, binary-safe value preview |
| **Set** | Member list | Lazy loading, click elements to edit |
| **ZSet** | Score-member list | Display sorted by score |

#### Binary-Safe Hash Fields

Hash field values are read as raw bytes, so non-UTF-8 content (serialized protobuf, gzipped blobs, image bytes, etc.) is shown as a hex preview placeholder `[Binary data, N bytes]...` instead of failing or being mis-decoded. UTF-8 values render normally and remain fully editable.

Field listing itself is also robust against very large hashes:

- **Redis 7.4+** uses `HSCAN ... NOVALUES` so the server only transfers field names.
- **Redis < 7.4** falls back to `HKEYS` with a client-side glob match. The previous plain `HSCAN` returned alternating field/value pairs and could exceed the response timeout on multi-MB hashes; the new path keeps the per-call payload small enough to always fit in the budget.

A safety cap (100,000 fields per call) prevents a pathological key from exhausting memory.

### Editing Keys

1. Click the **Edit** button to enter edit mode
2. Modify the key name or value content
3. Click **Save** to apply changes, or **Cancel** to discard

### Creating New Keys

1. Click the **➕** button at the top of the left sidebar
2. Select the data type (String, List, Hash, Set, ZSet)
3. Enter the key name, initial value, and TTL (-1 for no expiration)
4. Click **Create** to create the key

### Deleting Keys

- Click the **Delete** button in view mode to delete the current key
- Deletion is irreversible, please proceed with caution

### Refreshing Keys

Click the **🔄** button to reload the current key's value and TTL.

### Modifying TTL

1. Click the edit button next to the TTL
2. Enter the new expiration time in seconds
3. Click **Save** to apply, or leave empty to remove expiration