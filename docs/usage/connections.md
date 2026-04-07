# Connection Management

## Creating a New Connection

1. Click the **"+ New"** button in the top panel
2. Fill in the connection details in the dialog:
   - **Connection Name**: Custom identifier (e.g., "Production", "Local Dev")
   - **Host Address**: Redis server IP or hostname
   - **Port**: Default is 6379
   - **Username** (optional): ACL authentication username
   - **Password** (optional): Authentication password
   - **Color Marker**: Choose a high-contrast color to distinguish environments
3. Click **Save** to save the connection configuration

### Quick Connect Mode

You can also connect using a Redis URL string:

1. In the New Connection dialog, toggle to **Quick Connect** mode
2. Enter a Redis URL: `redis://[<username>][:<password>@]<hostname>[:port][/[<db>]]`
3. For TLS connections, use `rediss://` scheme

**Examples**:
- `redis://localhost:6379` - Basic connection
- `redis://:mypassword@localhost:6379/0` - With password and database
- `rediss://prod.redis.example.com:6380` - TLS encrypted connection

### TLS Support

Both Quick Connect and Advanced modes support TLS/SSL connections:

- **Quick Connect**: Use `rediss://` URL scheme
- **Advanced Mode**: Enable the **"Use TLS"** checkbox

## Connect/Disconnect

- Select a saved connection from the dropdown menu to automatically connect
- Click the **Disconnect** button next to the connection name to disconnect
- Supports multiple tabs connecting to different Redis instances simultaneously

## Edit/Delete Connections

- Hover over a connection item in the dropdown and click the edit icon to modify
- Click the delete icon to remove a saved connection