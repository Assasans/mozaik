use anyhow::{Result, Context};
use async_trait::async_trait;
use twilight_model::{gateway::payload::incoming::InteractionCreate, application::interaction::{application_command::CommandOptionValue, InteractionData}};

use crate::{try_unpack, State, interaction_response, get_option_as, player::Player, reply, update_reply};

use super::CommandHandler;

pub struct JoinCommand;

#[async_trait]
impl CommandHandler for JoinCommand {
  async fn run(&self, state: State, interaction: Box<InteractionCreate>) -> Result<()> {
    reply!(state, interaction, &interaction_response!(
      DeferredChannelMessageWithSource,
      content("Joining...")
    )).await?;

    let command = try_unpack!(interaction.data.as_ref().context("no interaction data")?, InteractionData::ApplicationCommand)?;
    let guild_id = interaction.guild_id.unwrap();
    let voice_state = state.cache.voice_state(interaction.member.as_ref().unwrap().user.as_ref().unwrap().id, guild_id);
    let channel_id = get_option_as!(command, "channel", CommandOptionValue::Channel)
      .map(|it| *it.unwrap())
      .or(voice_state.map(|it| it.channel_id()))
      .unwrap();

    let call = state.songbird.join(guild_id, channel_id).await?;

    let mut player = Player::new(state.clone(), guild_id);
    player.channel_id = Some(channel_id);
    player.call = Some(call);
    state.players.write().await.insert(guild_id, player);

    update_reply!(state, interaction)
      .content(Some(&format!("Joined channel <#{}>", channel_id)))?
      .await?;

    Ok(())
  }
}
