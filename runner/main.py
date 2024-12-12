import os
from datetime import datetime
from aoc.client import AoCClient
from oai.client import OpenAIClient
from claude.client import ClaudeClient
from rust.manager import RustManager
from dotenv import load_dotenv
import time
from utils.solution import SolutionSet


class AoCRunner:
    """Main runner class for Advent of Code solutions."""

    def __init__(self):
        load_dotenv()
        self.aoc_client = AoCClient(os.environ.get("AOC_SESSION"))
        self.openai_client = OpenAIClient(os.environ.get("OPENAI_API_KEY"))
        self.claude_client = ClaudeClient(os.environ.get("ANTHROPIC_API_KEY"))
        self.rust_manager = RustManager("../rust")
        self.results_dir = "results"
        self.models = ["gpt-4", "claude-3-opus-20240229", "o1-mini", "o1-preview"]
        self.attempts_per_model = 3

        if not os.path.exists(self.results_dir):
            os.makedirs(self.results_dir)

    def generate_and_test_solutions(
        self, year: int, day: int, part: int, challenge_text: str
    ) -> SolutionSet:
        """Generate and test multiple solutions from each model"""
        solution_set = SolutionSet()

        for model in self.models:
            for attempt in range(self.attempts_per_model):
                try:
                    # Get solution from AI model
                    if model.startswith("claude"):
                        solution_code = self.claude_client.get_solution(
                            challenge_text, part
                        )
                    else:
                        solution_code = self.openai_client.get_solution(
                            challenge_text, part, model
                        )

                    # Create temporary files and update Rust project
                    self.rust_manager.create_day_directory(day, part)
                    self.rust_manager.update_solution(day, part, solution_code)
                    self.rust_manager.update_cargo_toml(day, part, solution_code)

                    # Execute solution
                    try:
                        result = self.rust_manager.execute_solution(day, part)
                        solution_set.add_solution(model, solution_code, result)
                        print(
                            f"Model {model} (attempt {attempt + 1}) produced result: {result}"
                        )
                    except Exception as e:
                        print(
                            f"Execution failed for {model} (attempt {attempt + 1}): {e}"
                        )
                        solution_set.add_solution(model, solution_code, None)

                except Exception as e:
                    print(
                        f"Failed to generate solution with {model} (attempt {attempt + 1}): {e}"
                    )
                    continue

        return solution_set

    def has_solved_part(self, year: int, day: int, part: int) -> bool:
        """Check if a part has already been solved."""
        return os.path.exists(
            f"{self.results_dir}/{year}_day{day}_part{part}_success.txt"
        )

    def mark_part_as_solved(self, year: int, day: int, part: int):
        """Mark a part as solved by creating a success file."""
        with open(
            f"{self.results_dir}/{year}_day{day}_part{part}_success.txt", "w"
        ) as file:
            file.write("success")

    def solve_part(self, year: int, day: int, part: int, challenge_text: str):
        """Solve and submit a single part of the challenge."""
        print(f"Solving part {part}...")

        max_attempts = 3
        attempt = 0

        while attempt < max_attempts:
            attempt += 1
            print(f"Attempt {attempt}/{max_attempts} to solve Part {part}")

            # Reset Cargo.toml before each full attempt
            self.rust_manager.reset_cargo_toml()

            # Generate and test solutions from all models
            solution_set = self.generate_and_test_solutions(
                year, day, part, challenge_text
            )

            # Get most common result
            most_common_result, count = solution_set.get_most_common_result()

            if most_common_result is None:
                print("No valid solutions generated")
                continue

            print(f"Most common result ({count} occurrences): {most_common_result}")

            # If we have a strong majority (more than 50% of successful solutions)
            total_successful = sum(
                1 for s in solution_set.solutions if s.result is not None
            )
            if count > total_successful / 2:
                # Wait for leaderboard to fill
                while not self.aoc_client.global_leaderboard_full(year, day):
                    print(
                        "Global leaderboard not yet full. Checking again in 30 seconds..."
                    )
                    time.sleep(30)

                # Submit the most common result
                result = self.aoc_client.submit_solution(
                    year, day, part, most_common_result
                )
                print(f"Submission result: {result['status']} - {result['message']}")

                if result["status"] == "ok":
                    # Get the successful solution and save it
                    winning_solution = solution_set.get_solution_with_result(
                        most_common_result
                    )
                    self.rust_manager.update_solution(day, part, winning_solution.code)
                    self.mark_part_as_solved(year, day, part)
                    return

                # Handle wait time
                wait_time = result["wait_time"]
                if wait_time > 0:
                    print(f"Waiting {wait_time} seconds before next attempt...")
                    time.sleep(wait_time)
            else:
                print("No clear majority solution found")

            if attempt < max_attempts:
                print("Waiting 60 seconds before next full attempt...")
                time.sleep(60)

        print(f"Failed to solve Part {part} after {max_attempts} attempts.")

    def run(self):
        """Main execution flow."""
        now = datetime.now()
        year, day = now.year, now.day

        # Solve Part 1 if not already solved
        if not self.has_solved_part(year, day, 1):
            challenge_part1 = self.aoc_client.fetch_challenge(year, day)
            self.solve_part(year, day, 1, challenge_part1)
        else:
            print("Part 1 already solved. Skipping...")

        # Check if part 2 is available before solving
        full_challenge = self.aoc_client.fetch_challenge(year, day)
        if self.aoc_client.has_part_two(full_challenge):
            if not self.has_solved_part(year, day, 2):
                self.solve_part(year, day, 2, full_challenge)
            else:
                print("Part 2 already solved. Skipping...")
        else:
            print("Part 2 is not yet available.")


def main():
    runner = AoCRunner()
    runner.run()


if __name__ == "__main__":
    main()
