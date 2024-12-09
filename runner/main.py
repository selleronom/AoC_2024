import os
from datetime import datetime
from aoc.client import AoCClient
from oai.client import OpenAIClient
from rust.manager import RustManager
from dotenv import load_dotenv
import time


class AoCRunner:
    """Main runner class for Advent of Code solutions."""

    def __init__(self):
        load_dotenv()
        self.aoc_client = AoCClient(os.environ.get("AOC_SESSION"))
        self.openai_client = OpenAIClient(os.environ.get("OPENAI_API_KEY"))
        self.rust_manager = RustManager("../rust")
        self.results_dir = "results"
        self.models = ["gpt-4", "o1-mini", "o1-preview"]

        if not os.path.exists(self.results_dir):
            os.makedirs(self.results_dir)

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

            for model in self.models:
                print(f"Trying with model: {model}")
                # Get solution from OpenAI if the part is not already solved
                try:
                    solution_code = self.openai_client.get_solution(
                        challenge_text, part
                    )

                    # Update Rust project
                    self.rust_manager.create_day_directory(day, part)
                    self.rust_manager.update_solution(day, part, solution_code)
                    self.rust_manager.update_cargo_toml(day, part, solution_code)

                    # Execute and submit
                    answer = self.rust_manager.execute_solution(day, part)
                    print(f"Part {part} solution: {answer}")

                    while not self.aoc_client.global_leaderboard_full(year, day):
                        print(
                            "Global leaderboard not yet full. Checking again in 30 seconds..."
                        )
                        time.sleep(30)

                    result = self.aoc_client.submit_solution(year, day, part, answer)
                    print(
                        f"Day {day}, Part {part} solution submitted. Result: {result['status']} - {result['message']}"
                    )

                    if result["status"] == "ok":
                        self.mark_part_as_solved(year, day, part)
                        return  # Exit function on success

                    # Handle wait time
                    wait_time = result["wait_time"]
                    if wait_time > 0:
                        print(f"Waiting {wait_time} seconds before next attempt...")
                        time.sleep(wait_time)

                    # If wrong answer, try next model
                    if result["status"] == "wrong_answer":
                        continue

                    if result["status"] == "unknown":
                        break

                except Exception as e:
                    print(f"An error occurred with {model}: {e}")
                    time.sleep(10)  # Short wait after error
                    continue

            print(f"All models failed for attempt {attempt}")
            if attempt < max_attempts:
                print("Waiting 60 seconds before next full attempt...")
                time.sleep(60)

        print(
            f"Failed to solve Part {part} after {max_attempts} attempts with all models."
        )

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
