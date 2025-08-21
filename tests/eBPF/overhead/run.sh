#!/bin/bash
set -euo pipefail
trap 'echo "Error: Command failed with exit code $? at line $LINENO"' ERR

readonly BIN_DIR="./bin"
readonly SRC_DIR="./src"

# Display usage information
show_help() {
    cat <<EOF
Usage: $0 [COMMAND]

Supported commands:
  write        Compile and run write test
  read         Compile and run read test
  sendmsg      Compile sendmsg test program
  sendmmsg     Compile sendmmsg test program
  recvmsg      Compile recvmsg test program
  recvmmsg     Compile recvmmsg test program
  writev       Compile writev test program
  readv        Compile readv test program
  sendto       Compile sendto test program
  recvfrom     Compile recvfrom test program
  ssl_read     Compile and run SSL read test
  ssl_write    Compile and run SSL write test
  ssl          Compile and run generic SSL test
  -h, --help   Show this help message

Examples:
  $0 write
  $0 ssl_read
EOF
  exit 0
}

# Generic compilation function
compile_program() {
  local source_file="$1"
  local output_file="$2"
  local extra_flags="${3:-}"
  
  [[ ! -f "$source_file" ]] && { echo "Error: Source file $source_file not found"; exit 1; }

  echo "Compiling $source_file → $output_file"
  if ! gcc -Wall -Wextra -O2 $extra_flags -o "$output_file" "$source_file" -I./src/include; then
    echo "Compilation failed"
    exit 2
  fi
  echo "Compilation successful"
}

# Handle SSL-related commands
handle_ssl_command() {
  local command_name="$1"
  local source_file="${SRC_DIR}/${command_name}.c"
  local output_file="${BIN_DIR}/${command_name}"

  [[ ! -f "$source_file" ]] && { echo "Error: SSL source file $source_file not found"; exit 1; }

  compile_program "$source_file" "$output_file" "-lssl -lcrypto"
  echo "Executing $output_file..."
  "$output_file"
}

# Main execution flow
main() {
  # Ensure binary directory exists
  mkdir -p "$BIN_DIR" || { echo "Error: Failed to create directory $BIN_DIR"; exit 1; }

  case "${1:-}" in
    write|read|sendmsg|sendmmsg|recvmsg|recvmmsg|writev|readv|sendto|recvfrom)
      local command_name="$1"
      local source_file="${SRC_DIR}/${command_name}.c"
      local output_file="${BIN_DIR}/${command_name}"
      compile_program "$source_file" "$output_file"
      echo "Executing program: $output_file"
      "$output_file"
      ;;
    ssl|ssl_read|ssl_write)
      handle_ssl_command "$1" 
      ;;
    -h|--help|"")
      show_help
      ;;
    *)
      echo "Error: Unknown command '$1'"
      echo "Available commands:"
      $0 --help | sed -n '/Supported commands:/,/^$/p'
      exit 1
      ;;
  esac
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  [[ $# -eq 0 ]] && { show_help; exit 0; }
  main "$@"
fi