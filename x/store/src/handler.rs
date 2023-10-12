
use database::Database;
use gears::{error::AppError, types::context::TxContext};
use ibc_proto::protobuf::Protobuf;
use prost::Message as ProstMessage;
use store::StoreKey;
use tendermint_proto::abci::RequestBeginBlock;

use crate::{
    Config, Keeper, Message,
};

#[derive(Debug, Clone)]
pub struct Handler<SK: StoreKey> {
    keeper: Keeper<SK>,
    config: Config,
}

impl<SK: StoreKey> Handler<SK> {
    pub fn new(keeper: Keeper<SK>, config: Config) -> Self {
        Handler { keeper, config }
    }

    pub fn handle<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, SK>,
        msg: &Message,
    ) -> Result<(), AppError> {
        match msg {
            Message::Store(msg) => self.keeper.store_message(ctx, self.config.clone(), msg),
            Message::Get(msg) => self.keeper.get_message(ctx, msg),
        }
    }

    pub fn handle_begin_block<DB: Database>(
        &self,
        ctx: &mut TxContext<DB, SK>,
        _request: RequestBeginBlock,
    ) {
        let contribution_threshold: u32 = 2;
        let block_time = ctx.get_header().time.unix_timestamp();

        let (need_pub_keys, need_secret_keys) = self.keeper.get_empty_keypairs(ctx);

        self.keeper
            .make_public_keys(ctx, need_pub_keys, block_time, contribution_threshold);

        self.keeper.make_secret_keys(ctx, need_secret_keys);
    }

    pub fn handle_query<DB: Database>(
        &self,
        ctx: &gears::types::context::QueryContext<DB, SK>,
        query: tendermint_proto::abci::RequestQuery,
    ) -> std::result::Result<bytes::Bytes, AppError> {
        match query.path.as_str() {
            "/st.store.v1beta1.Query/GetAllMessages" => Ok(self
                .keeper
                .query_all_messages(&ctx)
                .into()),
            _ => Err(AppError::InvalidRequest("query path not found".into())),
        }
    }
}
