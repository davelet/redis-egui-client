# AI Assistant

**Talk to Redis in natural language.** No need to memorize commands.

Press `Cmd/Ctrl + E` to open the AI panel and start chatting.

---

## Quick Navigation

| Topic | Description |
|-------|-------------|
| [Configuration](configuration.md) | Set up AI models, API keys, and behavior options |
| [Tools Reference](tools.md) | Complete list of 18+ AI tools and capabilities |

---

## Example Conversations

```
You: "Show me all keys matching user:*"
AI:  → SCAN 0 MATCH user:* COUNT 100

You: "What's the TTL of session:abc123?"
AI:  → TTL session:abc123
     → Result: 3600 seconds (1 hour)

You: "How many keys are in this database?"
AI:  → DBSIZE
     → Result: 15,234 keys

You: "Delete all expired test keys"
AI:  → Analyzing keys matching test:*...
     → Found 23 expired keys
     → Suggest: DEL test:expired:1 test:expired:2 ...
```

---

## Two AI Modes

| Mode | Behavior | Best For |
|------|----------|----------|
| **Chat Mode** | Translates natural language → Redis command (requires confirmation) | Learning Redis, safe exploration |
| **Agent Mode** | Full tool access, executes operations directly | Power users, complex workflows |

### Chat Mode (Default)

1. Type your question in natural language
2. AI suggests a Redis command
3. Review and confirm before execution
4. Safe for learning and exploration

### Agent Mode

1. AI has direct access to 18+ Redis tools
2. Executes operations without confirmation
3. Maintains conversation context
4. Best for experienced users

---

## AI Capabilities (18+ Tools)

### Query Operations
- Filter keys by pattern
- Get detailed key information
- Check key existence
- Database statistics (key count, memory usage)

### Write Operations
- Set key values
- Set/update TTL
- Rename keys
- Delete keys

### Data Structure Operations
- **Hash**: HSET, HDEL, HGET, HGETALL
- **List**: LSET, LPUSH, RPUSH, LPOP, RPOP
- **Set**: SADD, SREM, SMEMBERS
- **ZSet**: ZADD, ZREM, ZRANGE

### Database Operations
- Execute raw Redis commands
- Switch databases (SELECT)
- Get server info

👉 See [Tools Reference](tools.md) for complete documentation.

---

## Context Awareness

The AI understands your current context:

- **Connection**: Which Redis instance you're connected to
- **Database**: Current DB number (0-15)
- **Selected Key**: The key you're viewing (if any)

This allows context-aware commands like:

```
You: "Delete this key"
AI:  → DEL user:123:profile  (uses currently selected key)
```

---

## Opening the AI Panel

Press `Cmd/Ctrl + E` or click the command line icon at the bottom.

The panel includes:
- **History area**: Previous commands and AI responses
- **Input field**: Enter Redis commands or natural language
- **Mode toggle**: Switch between Chat and Agent modes
- **Manual Interrupt**: A red square button (**🟥**) appears next to the "Thinking..." status. Click it to immediately abort long-running requests or stop the AI agent.

---

## Manual Interruption

If the AI is taking too long to think or if an Agent starts performing unintended operations, you can manually stop the process:

1. Look for the red square button (**🟥**) appearing next to the status text in the history area.
2. Click the button to immediately **abort** the remote request and **stop** log capture.
3. The status will update to **(Interrupted)**.

> [!NOTE]
> To prevent accidental interruption, no keyboard shortcut is provided for this action. It must be triggered by manually clicking the button.

---

## Smart Input Detection

The AI intelligently detects your input type:

| Input | Detection | Behavior |
|-------|-----------|----------|
| `GET mykey` | Redis command | Execute directly |
| `show all keys` | Natural language | Send to AI |
| `SET foo bar` | Redis command | Execute directly |
| `delete expired keys` | Natural language | Send to AI |

---

## Configuration

1. Open Settings (`Cmd/Ctrl + ,`)
2. Go to **AI** tab
3. Configure:
   - Enable/disable AI features
   - Select default model
   - Toggle confirmation before execute
   - Customize system prompt

👉 See [Configuration](configuration.md) for detailed setup instructions.