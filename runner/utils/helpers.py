import re


def extract_part2(challenge_text: str) -> str:
    """Extract Part Two section from challenge text."""
    part_two_index = challenge_text.find("--- Part Two ---")
    return challenge_text[part_two_index:] if part_two_index != -1 else ""


def remove_markdown(code: str) -> str:
    """Remove markdown formatting from code."""
    cleaned_code = []
    lines = code.splitlines()
    in_code_block = False

    for line in lines:
        line = line.strip()
        if line.startswith("```"):
            in_code_block = not in_code_block
            continue
        if in_code_block and not line.startswith("//"):
            cleaned_code.append(line)

    return "\n".join(cleaned_code)


def extract_dependencies(solution):
    # This is a simple regex-based extraction. You might need to enhance this
    # based on the complexity of the OpenAI solutions.
    deps = {}
    matches = re.findall(r"use\s+(\w+)", solution)
    for match in matches:
        if match not in ["std", "self"]:
            deps[match] = "*"  # Using "*" as a placeholder version
    return deps
