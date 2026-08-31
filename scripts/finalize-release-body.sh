#!/usr/bin/env bash
# finalize-release-body.sh
#
# Binds real upstream job outputs -- the container image's immutable digest,
# the tag reference the run actually pushed, and the measured image size --
# into the GitHub release body `create-or-reuse-release.sh` already created
# from the curated CHANGELOG.md section (ARTIFACT-03, ARTIFACT-04). Assembly
# order is inverted from the old shape: `create-release` publishes the
# curated notes alone, and this script -- run from a terminal
# `finalize-release-body` job -- appends the artifact sections afterwards,
# from what the run actually produced, never from a hand-reconstructed
# guess (D-04).
#
# Idempotency (Phase 20 D-03: every job must be safe to run twice): the
# current release body is read back and truncated at a fixed literal marker
# (`<!-- paladin:release-artifacts -->`) using bash's longest-match
# parameter-expansion trim (`${body%%"$MARKER"*}`) -- never a regex, so a
# curated section containing regex-special characters can never perturb the
# cut point. The body is then fully REBUILT from current inputs, never
# appended to -- `gh release edit --notes-file` replaces the whole body, and
# an append-based implementation would duplicate the artifact block on every
# re-run. If the marker is absent (the first run over a tag), the entire
# current body is treated as the curated section and is preserved in full.
# Running this script N times over the same upstream outputs reproduces a
# byte-identical body every time.
#
# Digest normalisation: docker/build-push-action's `digest` output's exact
# wire format (whether it already carries the `sha256:` prefix) is
# unconfirmed (RESEARCH.md Assumption A1) -- a value already prefixed
# `sha256:` is used verbatim; a bare hex value is prefixed exactly once, so
# neither answer breaks the pull line. The digest-pinned image reference is
# derived by stripping the tag suffix from the supplied --image-ref, never
# by rebuilding it from the repository name (the existing `Verify image
# size` step's own comment documents why: ghcr requires lowercase,
# github.repository does not, and metadata-action's own JSON output is the
# only value that has actually been proven correct).
#
# The image-size figure is always advisory prose -- stated against the
# 500 MB target -- and never affects this script's exit code (D-10): a run
# whose image exceeds the target still ends green, honestly reporting the
# measurement in the release body instead of a `::warning::` annotation
# nobody reads.
#
# A container-image section is emitted only when BOTH a digest and an image
# reference are present -- a half-populated section (e.g. a digest with no
# reference) is never advertised. When no complete artifact section can be
# built at all, the body states plainly that no artifacts were recorded for
# this run, rather than an artifact heading with nothing under it.
#
# The `gh` executable is resolved through GH_BIN (default: `gh`) -- the same
# testability seam scripts/create-or-reuse-release.sh's regression harness
# uses to stub the GitHub API, so this script's own harness never touches
# the network either.
#
# Sourcing seam: set FINALIZE_RELEASE_BODY_LIB_ONLY=1 before sourcing this
# file to load finalize_release_body_main (and its helpers, including the
# pure compose_release_body) without executing it -- this file's own
# regression harness uses this to exercise composition directly.
#
# Usage:  ./scripts/finalize-release-body.sh --tag <vX.Y.Z>
#             [--image-digest <value>] [--image-ref <reference>]
#             [--image-size-mb <integer>] [--output <path>]
#         --tag is required. --image-digest, --image-ref and
#         --image-size-mb are optional; when omitted (or empty) the
#         corresponding section is omitted from the body -- a caller (the
#         `finalize-release-body` workflow job) passes empty strings rather
#         than omitting the flag when an upstream leg did not succeed.
#         --output defaults to "${RUNNER_TEMP:-/tmp}/final-body.md". An
#         unrecognised flag is a usage error.
# Output: writes the fully-composed release body to --output, then
#         publishes it with `$GH_BIN release edit "$TAG" --notes-file
#         <output>`.
# Exit:   0 on success; non-zero for a usage error or any non-zero `gh`
#         exit (a non-2xx response from `gh release edit` is already a hard
#         failure -- no status-code introspection is needed here, unlike
#         create-or-reuse-release.sh's create/reuse decision table).

set -euo pipefail

# The fixed literal marker separating the curated changelog section (never
# touched) from the artifact sections this script owns (always rebuilt).
# Consumer-facing, so it does not carry this planning tool's own `gsd:`
# namespace the way RESEARCH.md/PATTERNS.md sketch it -- deliberately
# deviated here (see 21-03-PLAN.md's flagged assumptions).
MARKER='<!-- paladin:release-artifacts -->'

# _frb_normalize_digest VALUE -> prints VALUE with exactly one `sha256:`
# prefix: verbatim if VALUE already carries it, prefixed otherwise. Prints
# nothing when VALUE is empty.
_frb_normalize_digest() {
    local value="$1"
    if [ -z "${value}" ]; then
        return 0
    fi
    case "${value}" in
        sha256:*) printf '%s' "${value}" ;;
        *) printf 'sha256:%s' "${value}" ;;
    esac
}

# _frb_build_container_section IMAGE_REF NORMALIZED_DIGEST -> prints a
# `### Container image` section with a fenced, copy-paste-runnable
# `docker pull` line pinned to the digest, and the pushed tag reference that
# line was derived from. IMAGE_REF's tag suffix is stripped with bash's
# shortest-match trim (`${image_ref%:*}`) -- correct even when the registry
# host itself carries a port (`host:port/repo:tag` keeps `host:port`,
# strips only the trailing `:tag`).
_frb_build_container_section() {
    local image_ref="$1" digest="$2"
    local image_without_tag="${image_ref%:*}"
    printf '### Container image\n\n```\ndocker pull %s@%s\n```\n\nPushed tag: `%s`\n' \
        "${image_without_tag}" "${digest}" "${image_ref}"
}

# _frb_build_size_section SIZE_MB -> prints a `### Image size` section
# stating the measured whole-megabyte figure and its advisory verdict
# against the 500 MB target. At exactly 500 the verdict is "within target";
# above it, "over target". Never affects the caller's exit code -- purely
# informational prose (D-10).
_frb_build_size_section() {
    local size_mb="$1"
    local verdict="within target"
    if [ "${size_mb}" -gt 500 ]; then
        verdict="over target"
    fi
    printf '### Image size\n\nMeasured image size: %s MB -- %s (500 MB target, advisory only).\n' \
        "${size_mb}" "${verdict}"
}

# compose_release_body CURATED DIGEST IMAGE_REF SIZE_MB OUTPUT
#
# Pure composition: writes the full new release body to OUTPUT from CURATED
# (the already-truncated curated section) plus the artifact inputs. Touches
# no network. DIGEST is normalised here (see _frb_normalize_digest) so
# callers may pass either wire format. A container-image section is emitted
# only when both DIGEST and IMAGE_REF are non-empty; an image-size section
# only when SIZE_MB is non-empty. When neither section is emitted, a single
# line states that no artifacts were recorded -- never an empty heading.
#
# CURATED is normalised to end with exactly one trailing newline before the
# marker is appended (adding one only if missing, never adding a second) --
# this is what keeps a second run over this function's own previous output
# byte-identical to the first: re-extracting the curated section from a
# composed body always yields text that already ends in a single newline,
# so no further growth occurs on subsequent runs.
compose_release_body() {
    local curated="$1" digest="$2" image_ref="$3" size_mb="$4" output="$5"

    local norm_digest=""
    norm_digest="$(_frb_normalize_digest "${digest}")"

    local has_container=0 has_size=0
    local container_section="" size_section=""

    if [ -n "${norm_digest}" ] && [ -n "${image_ref}" ]; then
        has_container=1
        container_section="$(_frb_build_container_section "${image_ref}" "${norm_digest}")"
    fi

    if [ -n "${size_mb}" ]; then
        has_size=1
        size_section="$(_frb_build_size_section "${size_mb}")"
    fi

    local curated_out="${curated}"
    if [ -n "${curated}" ]; then
        case "${curated}" in
            *$'\n') : ;;
            *) curated_out="${curated}"$'\n' ;;
        esac
    fi

    {
        printf '%s' "${curated_out}"
        printf '%s\n' "${MARKER}"
        printf '\n---\n\n## Release Artifacts\n\n'
        if [ "${has_container}" -eq 0 ] && [ "${has_size}" -eq 0 ]; then
            printf 'No artifacts were recorded for this run.\n'
        else
            if [ "${has_container}" -eq 1 ]; then
                printf '%s\n' "${container_section}"
            fi
            if [ "${has_size}" -eq 1 ]; then
                printf '%s\n' "${size_section}"
            fi
        fi
    } > "${output}"
}

finalize_release_body_main() {
    local TAG="" IMAGE_DIGEST="" IMAGE_REF="" IMAGE_SIZE_MB=""
    local OUTPUT="${RUNNER_TEMP:-/tmp}/final-body.md"
    local GH="${GH_BIN:-gh}"

    while [ "$#" -gt 0 ]; do
        case "$1" in
            --tag)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --tag requires a value." >&2
                    return 1
                fi
                TAG="$2"
                shift 2
                ;;
            --image-digest)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --image-digest requires a value." >&2
                    return 1
                fi
                IMAGE_DIGEST="$2"
                shift 2
                ;;
            --image-ref)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --image-ref requires a value." >&2
                    return 1
                fi
                IMAGE_REF="$2"
                shift 2
                ;;
            --image-size-mb)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --image-size-mb requires a value." >&2
                    return 1
                fi
                IMAGE_SIZE_MB="$2"
                shift 2
                ;;
            --output)
                if [ "$#" -lt 2 ]; then
                    echo "ERROR: --output requires a value." >&2
                    return 1
                fi
                OUTPUT="$2"
                shift 2
                ;;
            *)
                echo "ERROR: unknown flag: $1" >&2
                echo "Usage: finalize-release-body.sh --tag <vX.Y.Z> [--image-digest <value>] [--image-ref <reference>] [--image-size-mb <integer>] [--output <path>]" >&2
                return 1
                ;;
        esac
    done

    if [ -z "${TAG}" ]; then
        echo "ERROR: --tag is required." >&2
        return 1
    fi

    local current_body=""
    current_body="$("${GH}" release view "${TAG}" --json body -q .body)"

    local curated_section=""
    curated_section="${current_body%%"${MARKER}"*}"

    compose_release_body "${curated_section}" "${IMAGE_DIGEST}" "${IMAGE_REF}" "${IMAGE_SIZE_MB}" "${OUTPUT}"

    "${GH}" release edit "${TAG}" --notes-file "${OUTPUT}"
}

if [ "${FINALIZE_RELEASE_BODY_LIB_ONLY:-0}" != "1" ]; then
    finalize_release_body_main "$@"
fi
