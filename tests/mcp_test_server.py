#!/usr/bin/env python3
"""
Mock MCP Server for testing STDIO transport
Implements a simple JSON-RPC 2.0 server over stdin/stdout
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

def handle_request(request):
    """Handle a JSON-RPC 2.0 request"""
    logging.debug(f"Received request: {request}")

    method = request.get("method")
    request_id = request.get("id")
    params = request.get("params", {})

    if method == "tools/list":
        # Return list of available tools
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
                            "required": ["message"]
                        }
                    },
                    {
                        "name": "calculator",
                        "description": "Performs basic arithmetic",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "operation": {"type": "string"},
                                "a": {"type": "number"},
                                "b": {"type": "number"}
                            },
                            "required": ["operation", "a", "b"]
                        }
                    }
                ]
            }
        }

    elif method == "tools/call":
        # Execute a tool
        tool_name = params.get("name")
        arguments = params.get("arguments", {})

        if tool_name == "echo":
            message = arguments.get("message", "")
            return {
                "jsonrpc": "2.0",
                "id": request_id,
                "result": {
                    "content": {
                        "type": "text",
                        "text": f"Echo: {message}"
                    }
                }
            }

        elif tool_name == "calculator":
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
                    "content": {
                        "type": "text",
                        "text": f"Result: {result}"
                    }
                }
            }

        else:
            # Tool not found
            return {
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {
                    "code": -32601,
                    "message": f"Tool not found: {tool_name}"
                }
            }

    else:
        # Method not found
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {
                "code": -32601,
                "message": f"Method not found: {method}"
            }
        }

def main():
    """Main loop - read from stdin, write to stdout"""
    logging.info("Mock MCP Server starting...")

    try:
        for line in sys.stdin:
            line = line.strip()
            if not line:
                continue

            try:
                request = json.loads(line)
                response = handle_request(request)

                # Write response as single line JSON
                print(json.dumps(response), flush=True)
                logging.debug(f"Sent response: {response}")

            except json.JSONDecodeError as e:
                logging.error(f"Invalid JSON: {e}")
                error_response = {
                    "jsonrpc": "2.0",
                    "id": None,
                    "error": {
                        "code": -32700,
                        "message": "Parse error"
                    }
                }
                print(json.dumps(error_response), flush=True)

    except KeyboardInterrupt:
        logging.info("Server shutting down...")
    except Exception as e:
        logging.error(f"Unexpected error: {e}", exc_info=True)
        sys.exit(1)

if __name__ == "__main__":
    main()
