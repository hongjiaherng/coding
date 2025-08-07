#!/bin/bash

# Input string
input="q138_copy_list_with_random_pointer;
q141_linked_list_cycle;
q143_reorder_list;
q146_lru_cache;
q19_remove_nth_node_from_end_of_list;
q206_reverse_linked_list;
q21_merge_two_sorted_lists;
q23_merge_k_sorted_lists;
q25_reverse_nodes_in_k_group;
q287_find_the_duplicate_number;
q2_add_two_numbers;"
output_dir="src/linked_list"

input_cleaned=$(echo "$input" | tr '\n' ' ')
IFS=';' read -r -a files <<< "$input_cleaned"
for filename in "${files[@]}"; do
  # Remove any leading/trailing whitespace
  filename=$(echo "$filename" | xargs)
  # Skip empty strings
  [ -z "$filename" ] && continue
  # Create the file
  touch "$output_dir/${filename}.rs"
  echo "Created file: $output_dir/${filename}.rs"
done

echo "All files created in src/linked_list/"
