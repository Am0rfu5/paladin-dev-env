#!/usr/bin/env bash
# Snyk credential loader for the Paladin devcontainer.
#
# Sourced (not executed) from ~/.bashrc by .devcontainer/post-start.sh, and safe
# to source repeatedly.
#
# Why this file exists
# --------------------
# The Snyk CLI authenticates from the SNYK_TOKEN environment variable (and reads
# SNYK_API for a self-hosted/regional endpoint). It does NOT read SNYK_API_KEY.
# Paladin stores the secret under the name SNYK_API_KEY, so this loader is the
# single place that maps SNYK_API_KEY -> SNYK_TOKEN. Change the mapping here, not
# in a dozen shell profiles.
#
# Resolution order (first hit wins, so an explicit env var always beats the file):
#   1. SNYK_TOKEN already exported          -> used as-is, nothing to do
#   2. SNYK_API_KEY already exported        -> mirrored into SNYK_TOKEN
#   3. ~/.config/paladin/snyk_api_key       -> read, exported as both
#
# The key file is bind-mounted read-only from the host (see docker-compose.yml),
# so it survives container rebuilds and is shared with any other project using the
# same convention. It is never written to, never committed, and never echoed.
#
# shellcheck shell=bash

# Guard: `return` is only valid in a sourced context.
(return 0 2>/dev/null) || {
    echo "snyk-env.sh must be sourced, not executed: . .devcontainer/snyk-env.sh" >&2
    exit 64
}

__paladin_snyk_key_file="${PALADIN_SNYK_KEY_FILE:-$HOME/.config/paladin/snyk_api_key}"

__paladin_load_snyk_env() {
    # (1) Already authenticated — respect it and leave the mirror consistent.
    if [ -n "${SNYK_TOKEN:-}" ]; then
        [ -z "${SNYK_API_KEY:-}" ] && export SNYK_API_KEY="$SNYK_TOKEN"
        return 0
    fi

    # (2) Host passthrough via docker-compose `environment:`.
    if [ -n "${SNYK_API_KEY:-}" ]; then
        export SNYK_TOKEN="$SNYK_API_KEY"
        return 0
    fi

    # (3) The bind-mounted key file.
    [ -r "$__paladin_snyk_key_file" ] || return 0

    local __key
    # Strip a trailing newline plus any surrounding whitespace — a key pasted with
    # `echo` or saved by an editor otherwise carries a \n into the auth header.
    __key="$(tr -d '\r\n' < "$__paladin_snyk_key_file" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')"

    # An empty or placeholder file is not an error — the developer simply has not
    # filled it in yet. Stay silent so every shell does not nag.
    case "$__key" in
        '' | '<'*'>' | 'REPLACE_ME'* | '#'*) return 0 ;;
    esac

    export SNYK_API_KEY="$__key"
    export SNYK_TOKEN="$__key"
    return 0
}

__paladin_load_snyk_env

# `snyk-status` — report whether credentials resolved, WITHOUT printing the secret.
# Useful in CI logs and when debugging a rebuild.
snyk-status() {
    if ! command -v snyk >/dev/null 2>&1; then
        echo "snyk: CLI not installed (rebuild the devcontainer)"
        return 1
    fi
    printf 'snyk: %s\n' "$(snyk --version 2>/dev/null || echo 'unknown version')"
    if [ -n "${SNYK_TOKEN:-}" ]; then
        printf 'auth: SNYK_TOKEN set (%s chars, ending %s)\n' \
            "${#SNYK_TOKEN}" "…${SNYK_TOKEN: -4}"
    else
        printf 'auth: NOT set — put your key in %s\n' "$__paladin_snyk_key_file"
        printf '      (host path: ~/.config/paladin/snyk_api_key, bind-mounted read-only)\n'
        return 1
    fi
}
