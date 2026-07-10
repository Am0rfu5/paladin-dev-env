#!/usr/bin/env python3
"""
Spec-strict mock MCP server for testing STDIO transport (Phase 12.1 D-04/D-06).

Implements a real MCP JSON-RPC 2.0 lifecycle: it tracks whether `initialize`
has been handled AND whether the `notifications/initialized` notification has
been received, and returns a JSON-RPC error for ANY `tools/list`/`tools/call`
request that arrives before BOTH have completed. This proves the client
sequences the MCP handshake correctly rather than merely being able to speak
the protocol (see 12.1-RESEARCH.md Pitfall 2) -- a permissive server that
skips this check would let a client with a broken/missing handshake pass
hermetic tests anyway.
"""

import sys
import json
import logging

# Configure logging to stderr (stdout is used for JSON-RPC)
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(levelname)s - %(message)s',
    stream=sys.stderr
)

# Handshake state. Both must be True before tools/list or tools/call are
# accepted -- this is what makes the server "spec-strict" rather than
# permissive.
handshake_state = {
    "received_initialize": False,
    "received_initialized_notification": False,
}


def handshake_complete():
    return (
        handshake_state["received_initialize"]
        and handshake_state["received_initialized_notification"]
    )


def jsonrpc_error(request_id, code, message):
    return {
        "jsonrpc": "2.0",
        "id": request_id,
        "error": {"code": code, "message": message},
    }


def handle_initialize(request):
    handshake_state["received_initialize"] = True
    request_id = request.get("id")
    return {
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {
            "protocolVersion": "2025-11-25",
            "capabilities": {
                "tools": {"listChanged": False}
            },
            "serverInfo": {
                "name": "paladin-mcp-test-server",
                "version": "0.1.0",
            },
        },
    }


def handle_tools_list(request):
    request_id = request.get("id")
    if not handshake_complete():
        logging.warning("Rejecting tools/list: MCP handshake not complete")
        return jsonrpc_error(
            request_id,
            -32600,
            "tools/list received before MCP handshake (initialize + "
            "notifications/initialized) completed",
        )

    return {
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {
            "tools": [
                {
                    "name": "echo",
                    "description": "Echoes the input back",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "message": {"type": "string"}
                        },
                        "required": ["message"],
                    },
                },
                {
                    "name": "calculator",
                    "description": "Performs basic arithmetic",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "operation": {"type": "string"},
                            "a": {"type": "number"},
                            "b": {"type": "number"},
                        },
                        "required": ["operation", "a", "b"],
                    },
                },
            ]
        },
    }


def handle_tools_call(request):
    request_id = request.get("id")
    if not handshake_complete():
        logging.warning("Rejecting tools/call: MCP handshake not complete")
        return jsonrpc_error(
            request_id,
            -32600,
            "tools/call received before MCP handshake (initialize + "
            "notifications/initialized) completed",
        )

    params = request.get("params") or {}
    tool_name = params.get("name")
    arguments = params.get("arguments") or {}

    if tool_name == "echo":
        message = arguments.get("message", "")
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {
                "content": [{"type": "text", "text": f"Echo: {message}"}]
            },
        }

    if tool_name == "calculator":
        operation = arguments.get("operation")
        a = arguments.get("a", 0)
        b = arguments.get("b", 0)

        result = 0
        if operation == "add":
            result = a + b
        elif operation == "subtract":
            result = a - b
        elif operation == "multiply":
            result = a * b
        elif operation == "divide":
            result = a / b if b != 0 else 0

        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {
                "content": [{"type": "text", "text": f"Result: {result}"}]
            },
        }

    return jsonrpc_error(request_id, -32601, f"Tool not found: {tool_name}")


def handle_request(request):
    """Handle a JSON-RPC 2.0 request (a message carrying an `id`)."""
    logging.debug(f"Received request: {request}")
    method = request.get("method")

    if method == "initialize":
        return handle_initialize(request)
    if method == "tools/list":
        return handle_tools_list(request)
    if method == "tools/call":
        return handle_tools_call(request)

    return jsonrpc_error(request.get("id"), -32601, f"Method not found: {method}")


def handle_notification(notification):
    """Handle a JSON-RPC 2.0 notification (a message with no `id`)."""
    method = notification.get("method")
    logging.debug(f"Received notification: {notification}")
    if method == "notifications/initialized":
        handshake_state["received_initialized_notification"] = True
        logging.debug("Handshake complete: notifications/initialized received")


def main():
    """Main loop - read from stdin, write to stdout"""
    logging.info("Spec-strict mock MCP server starting...")

    try:
        for line in sys.stdin:
            line = line.strip()
            if not line:
                continue

            try:
                message = json.loads(line)
            except json.JSONDecodeError as e:
                logging.error(f"Invalid JSON: {e}")
                print(
                    json.dumps(jsonrpc_error(None, -32700, "Parse error")),
                    flush=True,
                )
                continue

            # JSON-RPC 2.0 notifications never carry an `id`; requests always do.
            if "id" in message and message["id"] is not None:
                response = handle_request(message)
                print(json.dumps(response), flush=True)
                logging.debug(f"Sent response: {response}")
            else:
                handle_notification(message)

    except KeyboardInterrupt:
        logging.info("Server shutting down...")
    except Exception as e:
        logging.error(f"Unexpected error: {e}", exc_info=True)
        sys.exit(1)


if __name__ == "__main__":
    main()
