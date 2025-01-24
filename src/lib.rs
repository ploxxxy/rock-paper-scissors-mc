use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{arg_message::MsgArgConsumer, Arg, ConsumedArgs},
        dispatcher::CommandError,
        tree::CommandTree,
        tree_builder::argument,
        CommandExecutor, CommandSender,
    },
    plugin::Context,
    server::Server,
};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::{
    text::{color::NamedColor, TextComponent},
    PermissionLvl,
};
use rand::{thread_rng, Rng};

const NAMES: [&str; 2] = ["rps", "rockpaperscissors"];
const DESCRIPTION: &str = "Play Rock Paper Scissors with the server.";
const ARG_MESSAGE: &str = "choice";

#[derive(PartialEq, Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn beats(&self, other: &Choice) -> Option<bool> {
        match (self, other) {
            (a, b) if a == b => None,
            (Choice::Rock, Choice::Scissors) => Some(true),
            (Choice::Paper, Choice::Rock) => Some(true),
            (Choice::Scissors, Choice::Paper) => Some(true),
            _ => Some(false),
        }
    }
}

struct RockPaperScissorsExecutor;

#[async_trait]
impl CommandExecutor for RockPaperScissorsExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Msg(choice)) = args.get(ARG_MESSAGE) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_MESSAGE.into())));
        };

        let player_choice = str_to_choice(choice).ok_or_else(|| {
            CommandError::GeneralCommandIssue(
                "Player choice can only be \"rock\", \"paper\" or \"scissors\".".into(),
            )
        })?;

        let computer_choice = get_random_choice();

        sender
            .send_message(TextComponent::text("You chose: ").color_named(NamedColor::Aqua))
            .await;

        sender
            .send_message(
                TextComponent::text(format!("{:?}", player_choice)).color_named(NamedColor::Aqua),
            )
            .await;

        sender
            .send_message(TextComponent::text("I chose: ").color_named(NamedColor::Gold))
            .await;

        sender
            .send_message(
                TextComponent::text(format!("{:?}", computer_choice)).color_named(NamedColor::Gold),
            )
            .await;

        match player_choice.beats(&computer_choice) {
            Some(true) => {
                sender
                    .send_message(TextComponent::text("You win!").color_named(NamedColor::Green))
                    .await;
            }
            Some(false) => {
                sender
                    .send_message(TextComponent::text("You lose!").color_named(NamedColor::Red))
                    .await;
            }
            None => {
                sender
                    .send_message(
                        TextComponent::text("It's a tie!").color_named(NamedColor::Yellow),
                    )
                    .await;
            }
        }

        Ok(())
    }
}

fn str_to_choice(choice: &str) -> Option<Choice> {
    match choice {
        "rock" => Some(Choice::Rock),
        "paper" => Some(Choice::Paper),
        "scissors" => Some(Choice::Scissors),
        _ => None,
    }
}

fn get_random_choice() -> Choice {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let index = thread_rng().gen_range(0..3);
    choices[index]
}

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    server
        .register_command(
            CommandTree::new(NAMES, DESCRIPTION).with_child(
                argument(ARG_MESSAGE, MsgArgConsumer).execute(RockPaperScissorsExecutor),
            ),
            PermissionLvl::Zero,
        )
        .await;

    Ok(())
}

#[plugin_impl]
pub struct MyPlugin;

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}
impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
