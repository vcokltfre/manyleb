alias manyleb="cargo run --quiet"

examples=( params simple users parts )

for example in "${examples[@]}"; do
    echo "Processing example: $example"
    manyleb verify "examples/$example.manyleb" || { echo "Verification failed for $example"; exit 1; }
    manyleb format "examples/$example.manyleb" || { echo "Formatting failed for $example"; exit 1; }
    manyleb docs "examples/$example.manyleb" "examples/$example.md" || { echo "Documentation generation failed for $example"; exit 1; }
    echo "Successfully processed $example"
done


