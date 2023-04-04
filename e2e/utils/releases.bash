# Usage: with_retries <nr_retries> <command>
with_retries() {
    local -r nr_retries=$1
    shift
    local -r cmd=$@
    for i in $(seq 1 $nr_retries); do
        ($cmd) && break;
        sleep 2;
    done
}

# Print the latest dfx version available on GitHub releases
get_latest_dfx_version() {
    latest_version=$(with_retries 5 (curl -sL "https://api.github.com/repos/dfinity/sdk/releases/latest" | jq -r ".tag_name"))
    echo "$latest_version"
}

# Extract a particular file from the latest release tarball, and save it to the specified destination
# Usage: get_from_latest_release_tarball <file_path> <destination>
get_from_latest_release_tarball() {
    local -r file_path=$1
    local -r destination=$2
    local -r tarball_url=$(with_retries 5 (curl -sL "https://api.github.com/repos/dfinity/sdk/releases/latest" | jq -r ".tarball_url"))

    local -r temp_dir=$(mktemp -d)
    with_retries 5 (curl -sL "$tarball_url" -o "${temp_dir}/release.tar.gz")

    if [ "$(uname)" == "Darwin" ]; then
        tar -xzf "$temp_dir/release.tar.gz" -C "$temp_dir" "*/$file_path"
    elif [ "$(uname)" == "Linux" ]; then
        tar -xzf "$temp_dir/release.tar.gz" -C "$temp_dir" --wildcards "*/$file_path"
    fi

    local -r file_name=$(basename "$file_path")
    local -r extracted_file=$(find "$temp_dir" -type f -name "$file_name")
    mv "$extracted_file" "$destination"
}
