# AI Tools Guide

This document describes the AI-powered tools available in Redis Client. The AI assistant can help you manage your Redis database through natural language commands.

## Table of Contents

- [Available Tools](#available-tools)
- [Usage Examples](#usage-examples)
- [Tool Details](#tool-details)

---

## Available Tools

The AI assistant has access to the following Redis operations:

### Query Tools
| Tool | Description |
|------|-------------|
| `filter_keys` | List keys matching a pattern |
| `get_key_info` | Get detailed key information |
| `key_exists` | Check if a key exists |
| `get_db_stats` | Get database statistics |

### Write Tools
| Tool | Description |
|------|-------------|
| `set_string` | Set a string value |
| `set_ttl` | Set or remove key expiration |
| `rename_key` | Rename a key |
| `delete_keys` | Delete one or more keys |

### Data Structure Tools
| Tool | Data Type | Operation |
|------|-----------|-----------|
| `hset` / `hdel` | Hash | Set/delete field |
| `lset` | List | Set element by index |
| `sadd` / `srem` | Set | Add/remove member |
| `zadd` / `zrem` | Sorted Set | Add/remove member |
| `rpush` | List | Push to end |

### Database Tools
| Tool | Description |
|------|-------------|
| `execute_redis_command` | Execute any Redis command |
| `select_db` | Switch to another database |

---

## Usage Examples

### Browsing Keys

```
User: Show me all keys starting with "user:"
AI: Uses filter_keys with pattern "user:*"
```

```
User: What's in key "session:123"?
AI: Uses get_key_info to show type, TTL, and value preview
```

### Managing Data

```
User: Set my cache key to expire in 1 hour
AI: Uses set_ttl with ttl=3600
```

```
User: Add "user:100" to the users set
AI: Uses sadd with key="users", member="user:100"
```

### Database Operations

```
User: How many keys are in the current database?
AI: Uses get_db_stats to get total key count
```

```
User: Switch to database 1
AI: Uses select_db with db=1
```

---

## Tool Details

### filter_keys
Filters Redis keys by glob pattern.

| Parameter | Type | Description |
|-----------|------|-------------|
| pattern | string | Glob pattern (e.g., `user:*`, `*`, `*:10`) |
| limit | number | Maximum keys to return (default: 100) |

### get_key_info
Gets detailed information about a key.

| Parameter | Type | Description |
|-----------|------|-------------|
| key | string | The key name |

Returns: key type, TTL, TTL description, value preview

### set_string
Sets a string value for a key.

| Parameter | Type | Description |
|-----------|------|-------------|
| key | string | The key to set |
| value | string | The string value |

### set_ttl
Sets the expiration time for a key.

| Parameter | Type | Description |
|-----------|------|-------------|
| key | string | The key |
| ttl | number | Seconds until expiration (-1 to remove) |

### delete_keys
Deletes one or more keys.

| Parameter | Type | Description |
|-----------|------|-------------|
| keys | array | Array of key names to delete |

### hset / hdel
Set or delete a field in a Hash.

| Parameter | Type | Description |
|-----------|------|-------------|
| key | string | The hash key |
| field | string | The field name |
| value | string | The value (hset only) |

### sadd / srem
Add or remove a member from a Set.

### zadd / zrem
Add or remove a member from a Sorted Set.

| Parameter | Type | Description |
|-----------|------|-------------|
| key | string | The sorted set key |
| score | number | The score |
| member | string | The member |

### execute_redis_command
Execute any Redis command directly.

| Parameter | Type | Description |
|-----------|------|-------------|
| command | string | Complete Redis command (e.g., `GET mykey`) |

---

## Notes

- All tools require an active Redis connection
- The AI assistant maintains conversation context for multi-turn dialogues
- Tools automatically handle errors and provide clear feedback
- Use natural language - the AI will choose the appropriate tool
