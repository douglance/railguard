#!/bin/bash
# PostToolUse hook for Railguard - async audit logging
# This script runs asynchronously after tool completion

PLUGIN_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Read the PostToolUse event from stdin
# Format: {"tool_name": "...", "tool_input": {...}, "tool_result": {...}}
read -r event

# For now, just log to a file if RAILGUARD_AUDIT_LOG is set
if [[ -n "$RAILGUARD_AUDIT_LOG" ]]; then
    timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    echo "{\"timestamp\":\"$timestamp\",\"event\":$event}" >> "$RAILGUARD_AUDIT_LOG"
fi

# Future: Could send to railguard.app for cloud analytics
# "${PLUGIN_ROOT}/bin/railguard" audit-log "$event"

exit 0
