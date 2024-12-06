import requests
import openai
import os
import subprocess
from datetime import datetime
from bs4 import BeautifulSoup
import toml
import re

# Configure OpenAI API
client = openai.OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))

# Advent of Code session cookie
AOC_SESSION = os.environ.get("AOC_SESSION")


def fetch_aoc_challenge(year, day):
    url = f"https://adventofcode.com/{year}/day/{day}"
    response = requests.get(url, cookies={"session": AOC_SESSION})
    soup = BeautifulSoup(response.text, "html.parser")
    challenge_text = soup.find("article", class_="day-desc").get_text()
    return challenge_text


def get_openai_solution(challenge, part):
    response = client.chat.completions.create(
        model="gpt-4o",
        messages=[
            {
                "role": "system",
                "content": """Advent of Code prompts. Answer with code suggestions in Rust language. My main.rs is calling a helper function to download the input data from AoC. It then tries to solve each part with sub modules, for example part1.rs and part2.rs. See example:

            //dayXX.rs
            use advent_of_code_2024::utils::helpers::get_input;
            use advent_of_code_2024::dayXX::partX;
            use std::error::Error;

            #[tokio::main]
            async fn main() -> Result<(), Box<dyn Error>> {
                let input_data = get_input().await?;

                println!("Part X: {}", partX::solve(&input_data));

                Ok(())
            }
            Suggest only the Rust code for the partX, no explanations, just the code.
            Make sure to include the necessary imports and use statements.
            """,
            },
            {
                "role": "user",
                "content": f"Solve part {part} of this Advent of Code challenge in Rust:\n\n{challenge}",
            },
        ],
    )
    return response.choices[0].message.content


def remove_markdown(code):
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


def update_rust_solution(day, part, code):
    day_str = f"day{day:02d}"
    file_path = f"../rust/src/{day_str}/part{part}.rs"
    mod_path = f"../rust/src/{day_str}/mod.rs"

    # Ensure the directory exists
    os.makedirs(os.path.dirname(file_path), exist_ok=True)

    # Update the partX.rs file
    cleaned_code = remove_markdown(code)
    with open(file_path, "w") as f:
        f.write(cleaned_code)

    # Create or update the mod.rs file
    if os.path.exists(mod_path):
        with open(mod_path, "r") as f:
            mod_content = f.read()
        if f"pub mod part{part};" not in mod_content:
            mod_content += f"\npub mod part{part};"
    else:
        mod_content = f"pub mod part{part};"

    with open(mod_path, "w") as f:
        f.write(mod_content)


def execute_rust_solution(day, part):
    day_str = f"day{day:02d}_part{part}"
    cwd = "../rust"

    # Clean step (optional)
    subprocess.run(["cargo", "clean"], check=True, cwd=cwd)
    subprocess.run(["cargo", "fmt"], check=True, cwd=cwd)

    # Build step
    build_result = subprocess.run(
        ["cargo", "build", "--release"], capture_output=True, text=True, cwd=cwd
    )
    if build_result.returncode != 0:
        return f"Build failed: {build_result.stderr}"

    # Run step
    run_result = subprocess.run(
        ["cargo", "run", "--release", "-p", "advent-of-code-2024", "--bin", day_str],
        capture_output=True,
        text=True,
        cwd=cwd,
    )

    return run_result.stdout.strip()


def submit_solution(year, day, level, answer):
    url = f"https://adventofcode.com/{year}/day/{day}/answer"
    data = {"level": level, "answer": answer}
    response = requests.post(url, cookies={"session": AOC_SESSION}, data=data)
    return response.text


def extract_part2(challenge_text):
    part_two_index = challenge_text.find("--- Part Two ---")
    if part_two_index == -1:
        return ""
    return challenge_text[part_two_index:]


def create_day_directory(day, part):
    # Directories and files for the new day
    day_str = f"{day:02d}"
    src_dir = "../rust/src"
    day_dir = f"{src_dir}/day{day_str}"
    bin_dir = f"{src_dir}/bin"
    lib_path = f"{src_dir}/lib.rs"

    # Ensure the directories exist
    os.makedirs(day_dir, exist_ok=True)
    os.makedirs(bin_dir, exist_ok=True)

    # Create or update dayXX.rs in bin directory
    bin_path = f"{bin_dir}/day{day_str}_part{part}.rs"
    with open(bin_path, "w") as f:
        f.write(f"""use advent_of_code_2024::day{day_str}::part{part};
use advent_of_code_2024::utils::helpers::get_input;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {{
    let input_data = get_input().await?;

    println!("{{}}", part{part}::solve(&input_data));

    Ok(())
}}
""")

    # Update lib.rs
    with open(lib_path, "r+") as f:
        content = f.read()
        if f"pub mod day{day_str};" not in content:
            f.seek(0, 0)
            f.write(f"pub mod day{day_str};\n" + content)


def update_cargo_toml(day, part, openai_solution):
    day_str = f"{day:02d}"
    cargo_path = "../rust/Cargo.toml"

    # Read existing Cargo.toml
    with open(cargo_path, "r") as f:
        cargo_data = toml.load(f)

    # Update dependencies based on OpenAI solution
    dependencies = cargo_data.get("dependencies", {})
    new_deps = extract_dependencies(openai_solution)
    for dep, version in new_deps.items():
        if dep not in dependencies:
            dependencies[dep] = version

    cargo_data["dependencies"] = dependencies

    # Update or add binary section
    bin_name = f"day{day_str}_part{part}"
    bin_path = f"src/bin/day{day_str}_part{part}.rs"

    if "bin" not in cargo_data:
        cargo_data["bin"] = []

    # Check if the binary entry already exists
    bin_entry = next((b for b in cargo_data["bin"] if b.get("name") == bin_name), None)

    if bin_entry:
        bin_entry["path"] = bin_path
    else:
        cargo_data["bin"].append({"name": bin_name, "path": bin_path})

    # Write updated Cargo.toml
    with open(cargo_path, "w") as f:
        toml.dump(cargo_data, f)

    print(f"Cargo.toml updated for Day {day_str}, Part {part}")


def extract_dependencies(solution):
    # This is a simple regex-based extraction. You might need to enhance this
    # based on the complexity of the OpenAI solutions.
    deps = {}
    matches = re.findall(r"use\s+(\w+)", solution)
    for match in matches:
        if match not in ["std", "self"]:
            deps[match] = "*"  # Using "*" as a placeholder version
    return deps


def main():
    now = datetime.now()
    year = now.year
    day = now.day

    # Fetch and solve Part 1
    challenge_part1 = fetch_aoc_challenge(year, day)
    print(f"Challenge Part 1: {challenge_part1}")

    print("Solving part 1...")
    solution_code_part1 = get_openai_solution(challenge_part1, 1)
    create_day_directory(day, 1)
    update_rust_solution(day, 1, solution_code_part1)

    # Update Cargo.toml for Part 1
    update_cargo_toml(day, 1, solution_code_part1)

    answer_part1 = execute_rust_solution(day, 1)
    print(f"Part 1 solution: {answer_part1}")
    submission_result_part1 = submit_solution(year, day, 1, answer_part1)
    print(f"Day {day}, Part 1 solution submitted. Result: {submission_result_part1}")

    input("Press Enter to continue to part 2...")

    # Fetch and solve Part 2
    full_challenge = fetch_aoc_challenge(year, day)
    challenge_part2 = extract_part2(full_challenge)
    print(f"Challenge Part 2: {challenge_part2}")

    print("Solving part 2...")
    solution_code_part2 = get_openai_solution(challenge_part2, 2)
    create_day_directory(day, 2)
    update_rust_solution(day, 2, solution_code_part2)

    # Update Cargo.toml for Part 2
    update_cargo_toml(day, 2, solution_code_part2)

    answer_part2 = execute_rust_solution(day, 2)
    print(f"Part 2 solution: {answer_part2}")
    submission_result_part2 = submit_solution(year, day, 2, answer_part2)
    print(f"Day {day}, Part 2 solution submitted. Result: {submission_result_part2}")


if __name__ == "__main__":
    main()
