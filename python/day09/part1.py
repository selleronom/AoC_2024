with open('day_09_input.txt', 'r') as file:
    disk_map = file.read().strip()

# Parse the disk map to create the initial list of blocks
blocks = []
file_id = 0  # File IDs start from 0
is_file = True  # Indicator to switch between file and free space

i = 0
while i < len(disk_map):
    length = int(disk_map[i])
    # Handle multi-digit lengths if the length is zero
    # As per problem description, '0' represents zero-length file or free space
    # So, we can proceed without special handling for multi-digit lengths
    if is_file:
        blocks.extend([file_id] * length)
        file_id += 1
    else:
        blocks.extend(['.'] * length)
    is_file = not is_file
    i += 1

# Simulate the moving process
while True:
    # Find the leftmost free space
    try:
        lfs = blocks.index('.')
    except ValueError:
        # No free spaces left
        break  # Disk is fully compacted

    # Find the rightmost file block
    try:
        rfb = len(blocks) - 1 - blocks[::-1].index(next(b for b in reversed(blocks) if b != '.'))
    except StopIteration:
        # No file blocks found
        break

    if rfb <= lfs:
        # All file blocks are before the leftmost free space; disk is compacted
        break

    # Move one block from rfb to lfs
    blocks[lfs] = blocks[rfb]
    blocks[rfb] = '.'

# Calculate the checksum
checksum = 0
for position, block in enumerate(blocks):
    if block != '.':
        checksum += position * block

print(checksum)
