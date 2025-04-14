#!/bin/bash

set -e

checkcommand() {
    if ! command -v "$1" &>/dev/null; then
        echo "$1 could not be found. Please install it."
        exit 1
    fi
}


checkcommand fd

MAX_SIZE=1048576

pushd ~/wiki/vimwiki



# Find all files with "sync-conflict" in their names using fd
fd 'sync-conflict' -t f | while read -r conflict_file; do
  # Check if the file is a plain text file
  if file "$conflict_file" | grep -q "text"; then
    # Check if the file size is within the limit
    if [ "$(stat -c%s "$conflict_file")" -le "$MAX_SIZE" ]; then
      # Derive the original file name (assuming a naming convention)
      original_file="$(
          sed 's/\.sync-conflict-[A-Z0-9-]*\.md$/.md/' <<< "$conflict_file"
      )"
      echo "conflict_file: $conflict_file"
      echo "original_file: $original_file"

      if [ "$conflict_file"  = "$original_file" ]; then
        echo "conflict file and original file are the same file, wtf"
        echo "skipping"
        continue
      fi
      
      # Check if the original file exists
      if [ -f "$original_file" ]; then
        # Open both files in Neovim diff mode
        nvim -d "$conflict_file" "$original_file"
        
        # After Neovim exits, compare the files
        if cmp -s "$conflict_file" "$original_file"; then
          # If files are identical, remove the conflict file
          trash "$conflict_file"
          echo "Removed identical conflict file: $conflict_file"
        else
          echo "Files are different. Conflict file not removed: $conflict_file"
        fi
      else
        echo "Original file not found for: $conflict_file"
      fi
    fi
  fi
done

