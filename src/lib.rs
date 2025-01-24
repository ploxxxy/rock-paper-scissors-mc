mod game;

use async_trait::async_trait;
use game::{get_random_choice, Choice, Outcome};
use pumpkin::{
    command::{
        args::ConsumedArgs, dispatcher::CommandError, tree::CommandTree, tree_builder::literal,
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

const NAMES: [&str; 2] = ["rps", "rockpaperscissors"];
const DESCRIPTION: &str = "Play Rock Paper Scissors with the server.";

struct RockPaperScissorsExecutor(Choice);

#[async_trait]
impl CommandExecutor for RockPaperScissorsExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let player_choice = self.0;

        let computer_choice = get_random_choice();

        sender.send_message(TextComponent::text("")).await;

        sender
            .send_message(
                TextComponent::text("You chose: ")
                    .add_text(format!("{:?}", player_choice))
                    .color_named(NamedColor::Aqua),
            )
            .await;

        sender
            .send_message(
                TextComponent::text("I chose: ")
                    .add_text(format!("{:?}", computer_choice))
                    .color_named(NamedColor::Gold),
            )
            .await;

        match player_choice.beats(&computer_choice) {
            Outcome::Win => {
                sender
                    .send_message(TextComponent::text("You win!").color_named(NamedColor::Green))
                    .await;
            }
            Outcome::Lose => {
                sender
                    .send_message(TextComponent::text("You lose!").color_named(NamedColor::Red))
                    .await;
            }
            Outcome::Draw => {
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

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    let command = CommandTree::new(NAMES, DESCRIPTION)
        .with_child(literal("rock").execute(RockPaperScissorsExecutor(Choice::Rock)))
        .with_child(literal("paper").execute(RockPaperScissorsExecutor(Choice::Paper)))
        .with_child(literal("scissors").execute(RockPaperScissorsExecutor(Choice::Scissors)));

    server.register_command(command, PermissionLvl::Zero).await;

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
