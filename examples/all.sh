for dir in *; do
  if [ -d "$dir" ]; then
    echo "Building project in $dir"
    (cd "$dir" && cargo clippy && cd ..)
  fi
done

