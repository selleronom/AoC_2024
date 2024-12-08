from openai import OpenAI


class OpenAIClient:
    """Client for interacting with OpenAI API."""

    def __init__(self, api_key: str):
        self.client = OpenAI(api_key=api_key)

    def get_solution(self, challenge: str, part: int) -> str:
        """Get solution suggestion from OpenAI."""
        completion = self.client.chat.completions.create(
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
        return completion.choices[0].message.content
