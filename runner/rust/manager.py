import os
import subprocess
import toml
import utils.helpers as helpers
from utils.logger import setup_logger

logger = setup_logger(__name__)


class RustManager:
    """Manager for Rust project files and execution."""

    def __init__(self, rust_project_path: str):
        self.project_path = rust_project_path

    def create_day_directory(self, day: int, part: int):
        """Create necessary directory structure for new day."""
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

    def update_solution(self, day: int, part: int, code: str):
        """Update Rust solution files."""
        day_str = f"day{day:02d}"
        file_path = f"../rust/src/{day_str}/part{part}.rs"
        mod_path = f"../rust/src/{day_str}/mod.rs"

        # Ensure the directory exists
        os.makedirs(os.path.dirname(file_path), exist_ok=True)

        # Update the partX.rs file
        cleaned_code = helpers.remove_markdown(code)
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

    def read_solution(self, day: int, part: int) -> str:
        """Read Rust solution code."""
        day_str = f"day{day:02d}"
        file_path = f"../rust/src/{day_str}/part{part}.rs"
        with open(file_path, "r") as f:
            code = f.read()
        return code

    def update_cargo_toml(self, day: int, part: int, solution: str):
        """Update Cargo.toml with new dependencies and binaries."""
        day_str = f"{day:02d}"
        cargo_path = "../rust/Cargo.toml"

        # Read existing Cargo.toml
        with open(cargo_path, "r") as f:
            cargo_data = toml.load(f)

        # Update dependencies based on OpenAI solution
        dependencies = cargo_data.get("dependencies", {})
        new_deps = helpers.extract_dependencies(solution)
        for dep, version in new_deps.items():
            # Convert underscores to hyphens in dependency names
            dep_name = dep.replace("_", "-")
            if dep_name not in dependencies:
                dependencies[dep_name] = version

        cargo_data["dependencies"] = dependencies

        # Update or add binary section
        bin_name = f"day{day_str}_part{part}"
        bin_path = f"src/bin/day{day_str}_part{part}.rs"

        if "bin" not in cargo_data:
            cargo_data["bin"] = []

        # Check if the binary entry already exists
        bin_entry = next(
            (b for b in cargo_data["bin"] if b.get("name") == bin_name), None
        )

        if bin_entry:
            bin_entry["path"] = bin_path
        else:
            cargo_data["bin"].append({"name": bin_name, "path": bin_path})

        # Write updated Cargo.toml
        with open(cargo_path, "w") as f:
            toml.dump(cargo_data, f)

        logger.info(f"Cargo.toml updated for Day {day_str}, Part {part}")

    def reset_cargo_toml(self):
        """Reset Cargo.toml to its git state."""
        try:
            subprocess.run(
                ["git", "checkout", "Cargo.toml"], check=True, cwd=self.project_path
            )
            logger.info("Successfully reset Cargo.toml to git state")
        except subprocess.CalledProcessError as e:
            logger.info(f"Failed to reset Cargo.toml: {e}")

    def execute_solution(self, day: int, part: int) -> str:
        """Execute Rust solution and return output."""
        day_str = f"day{day:02d}_part{part}"
        cwd = "../rust"

        # Clean and format code
        subprocess.run(["cargo", "clean"], check=True, cwd=cwd)
        subprocess.run(["cargo", "fmt"], check=True, cwd=cwd)
        subprocess.run(["cargo", "fix", "--allow-dirty"], check=True, cwd=cwd)

        # Build step
        build_result = subprocess.run(
            ["cargo", "build", "--release"], capture_output=True, text=True, cwd=cwd
        )
        if build_result.returncode != 0:
            # Combine stderr and stdout for complete error message
            error_output = build_result.stderr.strip()
            # Extract the relevant error message
            if "error[" in error_output:
                # Get everything from the first error to the "For more information" line
                error_lines = error_output.split("\n")
                relevant_error = []
                capture = False
                for line in error_lines:
                    if line.startswith("error["):
                        capture = True
                    if capture:
                        if line.startswith("For more information"):
                            break
                        relevant_error.append(line)
                return "BUILD ERROR: " + "\n".join(relevant_error)
            return "BUILD ERROR: " + error_output

        # Run step
        try:
            run_result = subprocess.run(
                [
                    "cargo",
                    "run",
                    "-p",
                    "advent-of-code-2024",
                    "--bin",
                    day_str,
                ],
                capture_output=True,
                text=True,
                cwd=cwd,
                timeout=180,  # Timeout after 3 minutes
            )

            if run_result.returncode != 0:
                error_output = run_result.stderr.strip()
                return "EXECUTION ERROR: " + error_output

        except subprocess.TimeoutExpired:
            return "EXECUTION ERROR: Process timed out after 3 minutes"

        return run_result.stdout.strip()
