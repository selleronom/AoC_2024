import os
from datetime import datetime
from aoc.client import AoCClient
from oai.client import OpenAIClient
from claude.client import ClaudeClient
from rust.manager import RustManager
from dotenv import load_dotenv
import time
from utils.solution import SolutionSet
from utils.logger import setup_logger

logger = setup_logger(__name__)


class AoCRunner:
    """Main runner class for Advent of Code solutions."""

    def __init__(self):
        load_dotenv()
        self.aoc_client = AoCClient(os.environ.get("AOC_SESSION"))
        self.openai_client = OpenAIClient(os.environ.get("OPENAI_API_KEY"))
        self.claude_client = ClaudeClient(os.environ.get("ANTHROPIC_API_KEY"))
        self.rust_manager = RustManager("../rust")
        self.results_dir = "results"
        self.models = ["claude-3-5-sonnet-20241022", "o1-mini"]
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
                    self.rust_manager.reset_cargo_toml()
                    self.rust_manager.update_cargo_toml(day, part, solution_code)

                    # Execute solution
                    while True:  # Keep trying until we get a successful build or hit execution error
                        try:
                            result = self.rust_manager.execute_solution(day, part)
                            # If result contains build error or execution error
                            if isinstance(result, str) and (
                                result.startswith("BUILD ERROR:")
                                or result.startswith("EXECUTION ERROR:")
                            ):
                                logger.info(result)
                                # Extract the error message
                                error_message = result.replace(
                                    "BUILD ERROR: ", ""
                                ).replace("EXECUTION ERROR: ", "")
                                # Get fixed solution using the error message
                                if model.startswith("claude"):
                                    solution_code = self.claude_client.get_solution(
                                        challenge=f"You already given me the code for the challenge but it had issues, this is the code:\n\n {solution_code}\n\nThe error is:\n{error_message}",
                                        part=part,
                                        model=model,
                                    )
                                else:
                                    solution_code = self.openai_client.get_solution(
                                        challenge=f"You already given me the code for the challenge but it had issues, this is the code:\n\n {solution_code}\n\nThe error is:\n{error_message}",
                                        part=part,
                                        model=model,
                                    )
                                # Update solution and try again
                                self.rust_manager.update_solution(
                                    day, part, solution_code
                                )
                                self.rust_manager.reset_cargo_toml()
                                self.rust_manager.update_cargo_toml(
                                    day, part, solution_code
                                )
                                continue

                            solution_set.add_solution(model, solution_code, result)
                            logger.info(
                                f"Model {model} (attempt {attempt + 1}) produced result: {result}"
                            )
                            break  # Success - exit the while loop

                        except Exception as e:
                            logger.info(
                                f"Execution failed for {model} (attempt {attempt + 1}): {e}"
                            )
                            solution_set.add_solution(model, solution_code, None)
                            break  # Exit the while loop on execution error

                except Exception as e:
                    logger.info(
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
        logger.info(f"Solving part {part}...")

        max_attempts = 3
        attempt = 0

        while attempt < max_attempts:
            attempt += 1
            logger.info(f"Attempt {attempt}/{max_attempts} to solve Part {part}")

            # Generate and test solutions from all models
            solution_set = self.generate_and_test_solutions(
                year, day, part, challenge_text
            )

            # Get most common result
            most_common_result, count = solution_set.get_most_common_result()

            if most_common_result is None:
                logger.info("No valid solutions generated")
                continue

            logger.info(
                f"Most common result ({count} occurrences): {most_common_result}"
            )

            # If at least two results are the same, submit the most common one

            if count >= 2:
                # Wait for leaderboard to fill
                while not self.aoc_client.global_leaderboard_full(year, day):
                    logger.info(
                        "Global leaderboard not yet full. Checking again in 30 seconds..."
                    )
                    time.sleep(30)

                # Submit the most common result
                result = self.aoc_client.submit_solution(
                    year, day, part, most_common_result
                )
                logger.info(
                    f"Submission result: {result['status']} - {result['message']}"
                )

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
                    logger.info(f"Waiting {wait_time} seconds before next attempt...")
                    time.sleep(wait_time)
            else:
                logger.info("No clear majority solution found")

            if attempt < max_attempts:
                logger.info("Waiting 60 seconds before next full attempt...")
                time.sleep(60)

        logger.info(f"Failed to solve Part {part} after {max_attempts} attempts.")

    def run(self):
        """Main execution flow."""
        now = datetime.now()
        year, day = now.year, now.day

        # Solve Part 1 if not already solved
        if not self.has_solved_part(year, day, 1):
            challenge_part1 = self.aoc_client.fetch_challenge(year, day)
            self.solve_part(year, day, 1, challenge_part1)
        else:
            logger.info("Part 1 already solved. Skipping...")

        # Check if part 2 is available before solving
        full_challenge = self.aoc_client.fetch_challenge(year, day)
        if self.aoc_client.has_part_two(full_challenge):
            if not self.has_solved_part(year, day, 2):
                self.solve_part(year, day, 2, full_challenge)
            else:
                logger.info("Part 2 already solved. Skipping...")
        else:
            logger.info("Part 2 is not yet available.")


def main():
    runner = AoCRunner()
    runner.run()


if __name__ == "__main__":
    main()
