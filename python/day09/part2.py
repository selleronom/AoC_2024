with open('day_09_input.txt', 'r') as file:
    disk_map = file.read().strip()

# Parse the disk map to create the initial list of blocks and track file lengths
blocks = []
file_lengths = {}  # Dictionary to store file ID and its length
file_id = 0
is_file = True

i = 0
while i < len(disk_map):
    length = int(disk_map[i])
    if is_file:
        blocks.extend([file_id] * length)
        file_lengths[file_id] = length
        file_id += 1
    else:
        blocks.extend(['.'] * length)
    is_file = not is_file
    i += 1

# Process files in decreasing order of file ID
for current_file_id in range(max(file_lengths.keys()), -1, -1):
    if current_file_id not in file_lengths:
        continue

    file_length = file_lengths[current_file_id]

    # Find all blocks of this file
    file_start = -1
    for i in range(len(blocks)):
        if blocks[i] == current_file_id:
            file_start = i
            break

    if file_start == -1:
        continue

    # Look for leftmost suitable free space
    free_space_start = -1
    consecutive_free = 0

    for i in range(file_start):
        if blocks[i] == '.':
            if consecutive_free == 0:
                free_space_start = i
            consecutive_free += 1
            if consecutive_free >= file_length:
                # Found suitable space
                break
        else:
            consecutive_free = 0
            free_space_start = -1

    # If we found suitable space and it's to the left of the file
    if free_space_start != -1 and consecutive_free >= file_length and free_space_start < file_start:
        # Move the file
        # Clear original location
        for i in range(file_start, file_start + file_length):
            blocks[i] = '.'
        # Place in new location
        for i in range(free_space_start, free_space_start + file_length):
            blocks[i] = current_file_id

# Calculate checksum
checksum = 0
for position, block in enumerate(blocks):
    if block != '.':
        checksum += position * block

print(checksum)
