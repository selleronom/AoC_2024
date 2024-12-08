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

    def submit_solution(self, year: int, day: int, level: int, answer: str) -> dict:
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

        # Extract wait time from message if present
        wait_time = 0
        if "please wait" in message.lower():
            if "wait 5 minutes" in message:
                wait_time = 5 * 60
            elif "wait one minute" in message:
                wait_time = 60
            # Add more patterns if needed for other wait time messages

        if "That's the right answer" in message:
            return {
                "status": "ok",
                "message": message,
                "wait_time": 0
            }
        elif "That's not the right answer" in message:
            return {
                "status": "wrong_answer",
                "message": message,
                "wait_time": wait_time
            }
        elif "You gave an answer too recently" in message:
            return {
                "status": "too_frequent",
                "message": message,
                "wait_time": wait_time
            }
        else:
            return {
                "status": "unknown",
                "message": message,
                "wait_time": wait_time
            }

    def global_leaderboard_full(self, year: int, day: int) -> bool:
        """
        Fetch the leaderboard for a specific day.
        Returns True if there are two entries with rank 100.
        """
        url = f"https://adventofcode.com/{year}/leaderboard/day/{day}"
        response = requests.get(url, cookies={"session": self.session_cookie})

        if response.status_code != 200:
            raise Exception(f"Failed to fetch leaderboard: HTTP {response.status_code}")

        soup = BeautifulSoup(response.text, "html.parser")
        rank_100_count = 0

        for entry in soup.find_all("div", class_="leaderboard-entry"):
            rank_elem = entry.find("span", class_="leaderboard-position")
            if rank_elem and int(rank_elem.get_text().strip().replace(")", "")) == 100:
                rank_100_count += 1
                if rank_100_count == 2:
                    return True

        return False
