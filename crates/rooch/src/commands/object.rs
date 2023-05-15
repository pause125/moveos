use moveos_client::Client;
use moveos_types::object::ObjectID;
use rooch_types::cli::CliResult;

#[derive(clap::Parser)]
pub struct ObjectCommand {
    /// Object id.
    #[clap(long)]
    pub id: ObjectID,

    /// RPC client options.
    #[clap(flatten)]
    client: Client,
}

impl ObjectCommand {
    pub async fn execute(self) -> CliResult<()> {
        let resp = self.client.object(self.id).await?;
        println!("{:?}", resp);
        Ok(())
    }
}
