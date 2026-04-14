# AI Configuration Guide

This document explains how to configure AI models in Redis Client. All AI providers are configured as custom API endpoints.

## Table of Contents

- [AI Interaction Modes](#ai-interaction-modes)
- [Security Note](#security-note)
- [OpenAI](#openai)
- [Anthropic (Claude)](#anthropic-claude)
- [Ollama (Local)](#ollama-local)
- [OpenRouter](#openrouter)
- [Other Compatible APIs](#other-compatible-apis)

---

## AI Interaction Modes

Redis Client supports two distinct modes for AI interaction:

### Chat Mode (Stateless)

**Use Case**: Quick command translation without state persistence.

**Features**:
- **Stateless**: Each message is processed independently. Previous conversation context is NOT sent to the AI.
- **No Redis Access**: The AI does not have direct access to your Redis database or tools.
- **Command Translation**: Natural language queries are translated into Redis commands that you can review before executing.
- **Confirmation Required**: When the AI responds with a Redis command, you'll see a confirmation dialog before execution.

**Example Interaction**:
- User: "How many keys do I have?"
- AI Response: `DBSIZE` (shown in confirmation dialog)
- User confirms → Command executes

**Tip**: You can also copy the suggested command and paste it into the command line manually as an alternative workflow.

### Agent Mode (Stateful)

**Use Case**: Complex operations with multi-round conversation and direct Redis access.

**Features**:
- **Stateful**: Conversation context is maintained across messages for more natural interactions.
- **Redis Tools**: The AI has access to all Redis tools including:
  - `filter_keys`: Search keys with patterns
  - `get_key_info`: Get detailed key information
  - `execute_redis_command`: Execute any Redis command
  - `delete_keys`, `set_string`, `set_ttl`, and more
- **Direct Answers**: The AI can fetch data directly from Redis and provide answers without requiring user confirmation for each operation.
- **Multi-round Conversation**: Follow-up questions work naturally with context.

> [!IMPORTANT]
> **Agent Mode Requirements**: This mode requires LLMs with robust **tool-calling (function calling)** and **reasoning capabilities**. Not all models support tool calling. Recommended models include:
> - OpenAI: GPT-4o, GPT-4o-mini
> - Anthropic: Claude Sonnet 4, Claude 3.5 Sonnet
> - Local: Llama 3.x with tool calling support

**Example Interaction**:
- User: "Show me all user keys"
- AI: Uses `filter_keys` tool with pattern `user:*`, gets results, and displays them directly.

**Interruption**: If the AI Agent is taking many turns or performing unintended operations, you can immediately stop it by clicking the red square (**🟥**) button next to the status message.

### Switching Modes

You can switch between modes using the mode toggle in the command line panel (Chat/Agent buttons). Mode selection is per-tab and not persisted - each new CLI session starts in Agent mode.

**Switching modes** will automatically clear the existing agent state to ensure statelessness.

### Model Selection in CLI

You can select a different AI model per CLI session using the model dropdown. This selection is temporary and does not affect the global active model setting.

---

## Security Note

**API Key Storage**: Your API keys are securely stored in your operating system's native credential store:

| Platform | Credential Store |
|----------|-----------------|
| macOS | Keychain |
| Windows | Credential Manager |
| Linux | Secret Service (libsecret) |

API keys are **never** stored in plain text configuration files. This ensures your credentials remain secure even if the configuration file is accessed by other applications or accidentally shared.

---

## OpenAI

### API URL
```
https://api.openai.com/v1
```

### Model ID
Common models:
- `gpt-4o` - Latest GPT-4 Omni (recommended)
- `gpt-4o-mini` - Smaller, faster GPT-4
- `gpt-4-turbo` - GPT-4 Turbo
- `gpt-3.5-turbo` - GPT-3.5 Turbo (cheaper)

Full list: https://platform.openai.com/docs/models

### How to Get API Key
1. Go to https://platform.openai.com/api-keys
2. Sign in or create an account
3. Click "Create new secret key"
4. Copy the key (it won't be shown again)

### Example Configuration
| Field | Value |
|-------|-------|
| Name | My GPT-4 |
| URL | `https://api.openai.com/v1` |
| Model ID | `gpt-4o` |
| API Key | `sk-...` |

---

## Anthropic (Claude)

### API URL
```
https://api.anthropic.com/v1
```

### Model ID
Common models:
- `claude-sonnet-4-20250514` - Latest Claude 4 Sonnet (recommended)
- `claude-3-5-sonnet-20241022` - Claude 3.5 Sonnet
- `claude-3-opus-20240229` - Claude 3 Opus (most capable)
- `claude-3-haiku-20240307` - Claude 3 Haiku (fastest)

Full list: https://docs.anthropic.com/en/docs/models-overview

### How to Get API Key
1. Go to https://console.anthropic.com/
2. Sign in or create an account
3. Click "API Keys" in the left sidebar
4. Click "Create Key"
5. Copy the key

### Example Configuration
| Field | Value |
|-------|-------|
| Name | My Claude |
| URL | `https://api.anthropic.com/v1` |
| Model ID | `claude-sonnet-4-20250514` |
| API Key | `sk-ant-api03-...` |

**Note**: Anthropic requires adding `x-api-key` and `anthropic-version` headers. The client should automatically handle this when using `api.anthropic.com/v1` URL.

---

## Ollama (Local)

### API URL
```
http://localhost:11434
```

If Ollama is running on a different machine, use:
```
http://<hostname>:11434
```

### Model ID
List available models by running:
```bash
ollama list
```

Common models:
- `llama3` - Llama 3
- `llama3.2` - Llama 3.2
- `mistral` - Mistral
- `codellama` - Code Llama
- `qwen2.5` - Qwen 2.5
- `deepseek-r1` - DeepSeek R1

### How to Install Ollama
1. Go to https://ollama.com/
2. Download and install
3. Run `ollama serve` to start the server
4. Run `ollama pull <model>` to download models

### Example Configuration
| Field | Value |
|-------|-------|
| Name | Local Llama |
| URL | `http://localhost:11434` |
| Model ID | `llama3` |
| API Key | (leave empty) |

---

## OpenRouter

[OpenRouter](https://openrouter.ai/) is a unified API that provides access to 300+ AI models from various providers through a single endpoint.

### API URL
```
https://openrouter.ai/api/v1
```

### Model ID
OpenRouter uses model identifiers in the format `provider/model-name`:

**Popular Models:**
- `openai/gpt-4o` - OpenAI GPT-4 Omni
- `openai/gpt-4o-mini` - OpenAI GPT-4 Mini
- `anthropic/claude-sonnet-4-20250514` - Anthropic Claude 4 Sonnet
- `anthropic/claude-3.5-sonnet` - Anthropic Claude 3.5 Sonnet
- `google/gemini-pro-1.5` - Google Gemini Pro
- `meta-llama/llama-3.1-70b-instruct` - Meta Llama 3.1
- `deepseek/deepseek-chat` - DeepSeek Chat
- `qwen/qwen-2.5-72b-instruct` - Qwen 2.5

**Free Models:**
- `google/gemma-2-9b-it` - Gemma 2 (free tier available)
- `mistralai/mistral-7b-instruct` - Mistral 7B (free tier available)

Full list: https://openrouter.ai/docs/models

### How to Get API Key
1. Go to https://openrouter.ai/
2. Sign in (supports Google, GitHub, email)
3. Click "Keys" in the left sidebar
4. Click "Create Secret Key"
5. Copy the key

### Example Configuration
| Field | Value |
|-------|-------|
| Name | OpenRouter GPT-4 |
| URL | `https://openrouter.ai/api/v1` |
| Model ID | `openai/gpt-4o` |
| API Key | `sk-or-v1-...` |

**Note**: OpenRouter provides free credits for new users and supports models from many providers in one unified API.

---

## Other Compatible APIs

Many AI APIs are compatible with OpenAI's format and can be configured similarly:

### Azure OpenAI
| Field | Value |
|-------|-------|
| URL | `https://<your-resource>.openai.azure.com/openai/deployments/<deployment-name>` |
| Model ID | `gpt-4o` (or your deployment name) |

### Google Gemini (via OpenAI Compatibility)
Some proxy services provide OpenAI-compatible access to Gemini.

### Cohere
| Field | Value |
|-------|-------|
| URL | `https://api.cohere.ai/v1` |
| Model ID | `command-r-plus` |

### Cloudflare Workers AI
| Field | Value |
|-------|-------|
| URL | `https://api.cloudflare.com/client/v4/accounts/<account-id>/ai/v1` |
| Model ID | `@cf/meta/llama-3.1-8b-instruct` |

---

## General Tips

1. **Temperature**: Controls randomness (0.0 = deterministic, 2.0 = very creative). Recommended: 0.7 for general use.

2. **API Key Security**: API keys are securely stored in your operating system's native credential store (Keychain on macOS, Credential Manager on Windows, Secret Service on Linux). They are never saved to plain text configuration files.

3. **Testing**: After configuring, test with a simple prompt to verify the connection works.

4. **Rate Limits**: Be aware of API rate limits for each provider.

5. **Costs**: Monitor your API usage to avoid unexpected charges.

## Bulk Model Management (JSON Import/Export)

You can manage your list of AI models in bulk using JSON import and export. This is useful for sharing configurations between machines or quickly adding multiple models.

### Exporting Models
1. Open Settings -> **AI Settings**.
2. Click the **Export Models** button at the bottom of the model list.
3. Your current model list (excluding API keys) will be copied to your clipboard as a JSON array.

### Importing Models
1. Prepare a JSON file or copy a JSON array of model definitions to your clipboard.
2. Click the **Import Models** button.
3. A preview dialog will appear showing the models found in the JSON.
4. You can select which models to import. The systems automatically handles conflicts:
   - If a model with the same Name and URL already exists, it will be skipped.
   - You can choose to "Select All", "Invert Selection", or manually toggle models.
5. Click **Import Selection** to add them to your configuration.

> [!NOTE]
> **API Key Security**: For security reasons, API keys are **never included** in JSON exports. After importing a model, you must manually enter its API key to activate it.

---

## Troubleshooting

### API Key Not Found
If you encounter "API key not found" errors after configuring a model:
- Ensure your system's credential store is accessible
- On Linux, you may need to install `libsecret` and a compatible secret service (like GNOME Keyring or KWallet)
- Try re-entering the API key in the model configuration

### Migrating from Old Versions
If you previously had API keys stored in the configuration file:
1. Re-enter your API keys in the model configuration UI
2. The new keys will be automatically migrated to the secure credential store
3. Old keys in the configuration file will be ignored
