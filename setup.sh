#!/bin/bash

# Function to print a boundary with the filename
print_boundary() {
    echo -e "\n...........$1............"
    echo -e "File: $2\n"
    cat "$2"
    echo -e "\n...........$1 END............\n"
}

# Use git ls-files to get only tracked files, respecting .gitignore, excluding lock files
git ls-files | grep -E 'Dockerfile|\.(rs)|toml$' | grep -v -E '(lock|Lock)' | while read -r file; do
    # Get just the filename without the path
    filename=$(basename "$file")
    # Print the boundary and contents
    print_boundary "$filename" "$file"
done