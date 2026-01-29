#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <input_file>"
    exit 1
fi

input_file="$1"

if [ ! -f "$input_file" ]; then
    echo "Error: File '$input_file' not found"
    exit 1
fi

counter=40010
temp_array=()
line_count=0

while IFS= read -r line || [ ${#temp_array[@]} -gt 0 ]; do
    if [ -n "$line" ]; then
        temp_array+=("$line")
        ((line_count++))
    fi
    
    if [ ${#temp_array[@]} -eq 4 ]; then
        printf "%05d DATA %s,%s,%s,%s\n" "$counter" "${temp_array[0]}" "${temp_array[1]}" "${temp_array[2]}" "${temp_array[3]}"
        temp_array=()
        ((counter += 10))
    fi
done < "$input_file"

# Handle remaining lines if file doesn't have multiple of 4 lines
if [ ${#temp_array[@]} -gt 0 ]; then
    output_line=$(printf "%05d DATA " "$counter")
    for i in "${!temp_array[@]}"; do
        if [ $i -eq 0 ]; then
            output_line+="${temp_array[$i]}"
        else
            output_line+=",${temp_array[$i]}"
        fi
    done
    echo "$output_line"
fi
