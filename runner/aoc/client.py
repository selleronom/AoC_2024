import requests
from bs4 import BeautifulSoup
from typing import List, Dict, Union
import re


class AoCClient:
    """Client for interacting with Advent of Code website."""

    def __init__(self, session_cookie: str):
        self.session_cookie = session_cookie

    def fetch_challenge(self, year: int, day: int) -> str:
        """Fetch challenge description from AoC website."""
        url = f"https://adventofcode.com/{year}/day/{day}"
        response = requests.get(url, cookies={"session": self.session_cookie})
        soup = BeautifulSoup(response.text, "html.parser")
        # Find all articles with class 'day-desc'
        articles = soup.find_all("article", class_="day-desc")

        # Extract text from each article and return as a list
        return "\n\n".join(article.get_text() for article in articles)

    def has_part_two(self, challenge: str) -> bool:
        """Check if Part Two of the challenge is available in any of the descriptions."""
        if re.search(r"---\s*Part Two\s*---", challenge, re.IGNORECASE):
            return True
        return False

    def fetch_input(self, year: int, day: int) -> str:
        """Fetch input data for the challenge from AoC website."""
        url = f"https://adventofcode.com/{year}/day/{day}/input"
        response = requests.get(url, cookies={"session": self.session_cookie})

        if response.status_code != 200:
            raise Exception(f"Failed to fetch input data: HTTP {response.status_code}")

        return response.text.strip()

    def submit_solution(self, year: int, day: int, level: int, answer: str) -> str:
        """
        Submit solution to AoC website.
        Returns 'ok' if the answer was correct, or an error message if incorrect.
        """
        url = f"https://adventofcode.com/{year}/day/{day}/answer"
        data = {"level": level, "answer": answer}
        response = requests.post(
            url, cookies={"session": self.session_cookie}, data=data
        )

        soup = BeautifulSoup(response.text, "html.parser")
        message = soup.find("article").get_text().strip()

        if "That's the right answer" in message:
            return "ok"
        elif "That's not the right answer" in message:
            return f"error: wrong answer. {message}"
        elif "You gave an answer too recently" in message:
            return f"error: too many attempts. {message}"
        else:
            return f"error: unknown response. {message}"

    def fetch_leaderboard(
        self, year: int, day: int
    ) -> Dict[str, List[Dict[str, Union[str, int]]]]:
        """
        Fetch the leaderboard for a specific day.
        Returns a dictionary with 'part1' and 'part2' lists containing leaderboard entries.
        Each entry contains 'rank', 'time', 'score', and 'user' information.
        """
        url = f"https://adventofcode.com/{year}/leaderboard/day/{day}"
        response = requests.get(url, cookies={"session": self.session_cookie})

        if response.status_code != 200:
            raise Exception(f"Failed to fetch leaderboard: HTTP {response.status_code}")

        soup = BeautifulSoup(response.text, "html.parser")
        leaderboard = {"part1": [], "part2": []}

        # Find the leaderboard entries
        entries = soup.find_all("div", class_="leaderboard-entry")

        current_part = "part1"
        for entry in entries:
            # Check for part separator
            if "---" in entry.get_text():
                current_part = "part2"
                continue

        # Extract entry information
        rank_elem = entry.find("span", class_="leaderboard-position")
        time_elem = entry.find("span", class_="leaderboard-time")
        score_elem = entry.find("span", class_="leaderboard-score")
        user_elem = entry.find("span", class_="leaderboard-userphoto")

        if all([rank_elem, time_elem, user_elem]):
            entry_data = {
                "rank": int(rank_elem.get_text().strip().replace(")", "")),
                "time": time_elem.get_text().strip(),
                "score": int(score_elem.get_text().strip()) if score_elem else None,
                "user": user_elem.get("title", "Anonymous"),
            }
            leaderboard[current_part].append(entry_data)

        return leaderboard
