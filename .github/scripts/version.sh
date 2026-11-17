#!/usr/bin/env bash
# Version helpers shared by the Version and Release workflows.
#
#   version.sh next stable|beta|alpha   next version for HEAD, from the conventional
#                                       commits since the last release (prints
#                                       nothing when there is nothing to release)
#   version.sh apply <version>          writes <version> into the versioned files
#   version.sh current [<rev>]          version written in the files at <rev> (HEAD)
#   version.sh newer <a> <b>            exit 0 when version <a> is newer than <b>
#   version.sh previous <tag>           last release older than <tag> (release notes)
#   version.sh prerelease <tag>         exit 0 when <tag> is a pre-release (1.2.0-beta.3)
#
# Tags: 1.2.3 = release (main), 1.2.3-beta.N = beta (develop), 1.2.3-alpha.N =
# alpha (pull requests). Legacy 1.2.3-N tags are releases right after 1.2.3.
# Bump: "type!:" or "BREAKING CHANGE:" → major, "feat:" → minor, anything else → patch.

set -euo pipefail

RELEASE_RE='^[0-9]+\.[0-9]+\.[0-9]+(-[0-9]+)?$'
VERSION_RE='^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?$'
IMAGE_RE='ghcr\.io/2scloud/scloud-dns'

# Sort key for `sort -V`: 1.2.3-beta.4 → 1.2.3~beta.4 (before 1.2.3), 1.2.3-4 → 1.2.3.4 (after)
vkey() {
    if [[ "$1" =~ ^([0-9.]+)-([0-9]+)$ ]]; then
        printf '%s.%s' "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}"
    else
        printf '%s' "${1/-/'~'}"
    fi
}

sort_versions() {
    local v
    while IFS= read -r v; do
        if [[ -n "$v" ]]; then printf '%s\t%s\n' "$(vkey "$v")" "$v"; fi
    done | sort -t $'\t' -k1,1V | cut -f2
}

release_tags() {
    git tag -l | { grep -E "$RELEASE_RE" || true; }
}

last_release() {
    release_tags | sort_versions | tail -n1
}

# Highest bump in a revision range: major | minor | patch | none
bump_level() {
    git log --no-merges --format='%x1e%s%n%b' "$1" | awk '
        BEGIN { RS = "\036"; lvl = 0 }
        NF == 0 { next }
        {
            n = index($0, "\n")
            subject = n ? substr($0, 1, n - 1) : $0
            if (subject ~ /^chore\(release\): /) next
            if (subject ~ /^[A-Za-z]+(\([^)]*\))?!:/ || $0 ~ /BREAKING[ -]CHANGE:/) l = 3
            else if (subject ~ /^[Ff]eat(\([^)]*\))?:/) l = 2
            else l = 1
            if (l > lvl) lvl = l
        }
        END { split("none patch minor major", names, " "); print names[lvl + 1] }'
}

next_version() {
    local channel="$1" here done_re last level base major minor patch target n

    # HEAD already versioned for this channel (or a higher one): nothing to do.
    # An alpha on HEAD (fast-forwarded PR) does not prevent its beta or release.
    case "$channel" in
        stable) done_re="$RELEASE_RE" ;;
        beta)   done_re="${RELEASE_RE}|-beta\.[0-9]+$" ;;
        alpha)  done_re="$VERSION_RE" ;;
        *) echo "Unknown channel: ${channel} (stable, beta or alpha)" >&2; return 2 ;;
    esac

    here="$(git tag --points-at HEAD)"
    if grep -qE "$done_re" <<< "$here"; then return 0; fi

    last="$(last_release)"
    level="$(bump_level "${last:+${last}..}HEAD")"
    if [[ "$level" == "none" ]]; then return 0; fi

    base="${last%%-*}"
    IFS=. read -r major minor patch <<< "${base:-0.0.0}"

    case "$level" in
        major) target="$(( major + 1 )).0.0" ;;
        minor) target="${major}.$(( minor + 1 )).0" ;;
        patch) target="${major}.${minor}.$(( patch + 1 ))" ;;
    esac

    if [[ "$channel" == "stable" ]]; then
        echo "$target"
        return 0
    fi

    n="$(git tag -l "${target}-${channel}.*" | sed -nE "s/^${target//./\\.}-${channel}\.([0-9]+)$/\1/p" | sort -n | tail -n1)"
    echo "${target}-${channel}.$(( ${n:-0} + 1 ))"
}

apply_version() {
    local v="$1"

    if [[ ! "$v" =~ $VERSION_RE ]]; then
        echo "Invalid version: ${v}" >&2
        return 1
    fi

    sed -i -E "0,/^version = \"[^\"]*\"/s//version = \"${v}\"/" Cargo.toml
    sed -i -E "/^name = \"scloud-dns\"$/{n;s/^version = \"[^\"]*\"/version = \"${v}\"/}" Cargo.lock
    sed -i -E \
        -e "s#(image:[[:space:]]*${IMAGE_RE}:)[^\"'[:space:]]+#\1${v}#" \
        -e "s#(service\.version=)[^,\"'[:space:]]+#\1${v}#" \
        k3s/scloud-dns.yaml
    sed -i -E "0,/\"version\": \"[^\"]*\"/s//\"version\": \"${v}\"/" config/config.json

    local f
    for f in Cargo.toml Cargo.lock k3s/scloud-dns.yaml config/config.json; do
        if ! grep -qF "$v" "$f"; then
            echo "Version ${v} not written to ${f}" >&2
            return 1
        fi
    done
}

current_version() {
    git show "${1:-HEAD}:Cargo.toml" | sed -nE '0,/^version = /s/^version = "([^"]+)".*/\1/p'
}

newer() {
    [[ "$1" != "$2" && "$(printf '%s\n%s\n' "$1" "$2" | sort_versions | tail -n1)" == "$1" ]]
}

previous_release() {
    { release_tags; echo "$1"; } | sort -u | sort_versions \
        | awk -v t="$1" '$0 == t { print prev; exit } { prev = $0 }'
}

case "${1:-}" in
    next)       next_version "${2:?stable|beta|alpha}" ;;
    apply)      apply_version "${2:?version}" ;;
    current)    current_version "${2:-HEAD}" ;;
    newer)      newer "${2:?version}" "${3:?version}" ;;
    previous)   previous_release "${2:?tag}" ;;
    prerelease) [[ "${2:?tag}" =~ -[A-Za-z] ]] ;;
    *)
        echo "usage: $0 next stable|beta|alpha | apply <version> | current [<rev>] | newer <a> <b> | previous <tag> | prerelease <tag>" >&2
        exit 2
        ;;
esac
