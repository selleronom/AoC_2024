from anthropic import Anthropic


class ClaudeClient:
    """Client for interacting with Anthropic's Claude API."""

    def __init__(self, api_key: str):
        self.client = Anthropic(api_key=api_key)

    def get_solution(
        self, challenge: str, part: int, model: str = "claude-3-5-sonnet-20241022"
    ) -> str:
        """Get solution suggestion from Claude."""
        message = self.client.messages.create(
            model=model,
            max_tokens=4000,
            messages=[
                {
                    "role": "user",
                    "content": """
                Advent of Code prompts.
                Answer with code suggestions in Rust language.
                My main.rs is calling a helper function to download the input data from AoC.
                It then tries to solve each part with sub modules, for example part1.rs and part2.rs.

                See example:
                ```rs
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
                ```
                Suggest only the Rust code for the partX, no explanations, just the code.
                Make sure to include the necessary imports and use statements.
                """
                    f"Solve part {part} of this Advent of Code challenge in Rust:\n\n{challenge}",
                }
            ],
        )

        return message.content[0].text
