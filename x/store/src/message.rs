use bytes::Bytes;
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use proto_messages::cosmos::bank::v1beta1::MsgSend;
use proto_types::AccAddress;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Message)]
pub struct RawMsgKeyPair {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(int64, tag = "2")]
    pub pubkey_time: i64,
    #[prost(string, tag = "3")]
    pub public_key: String,
    #[prost(string, tag = "4")]
    pub private_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MsgKeyPair {
    pub id: u32,
    pub pubkey_time: i64,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawMsgVal {
    #[prost(string, tag = "1")]
    pub address: String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    #[prost(string, tag = "3")]
    pub msg: String,
}

/// Struct that keeps message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgVal {
    pub address: AccAddress,
    pub id: u64,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@type")]
pub enum Message {
    #[serde(rename = "/st.store.v1beta1.MsgVal")]
    Store(MsgVal),
    // TODO: design request
    Get(MsgVal),
}

impl proto_messages::cosmos::tx::v1beta1::Message for Message {
    fn get_signers(&self) -> Vec<&AccAddress> {
        match &self {
            Message::Store(msg) => return vec![&msg.address],
            Message::Get(msg) => return vec![&msg.address],
        }
    }

    fn validate_basic(&self) -> Result<(), String> {
        match &self {
            Message::Store(_) => Ok(()),
            Message::Get(_) => Ok(()),
        }
    }
}

impl From<Message> for Any {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Store(msg) => Any {
                type_url: "/st.store.v1beta1.MsgVal".to_string(),
                value: msg.encode_vec(),
            },
            Message::Get(msg) => Any {
                type_url: "/st.store.v1beta1.MsgVal".to_string(),
                value: msg.encode_vec(),
            },
        }
    }
}

impl TryFrom<Any> for Message {
    type Error = proto_messages::Error;

    fn try_from(value: Any) -> Result<Self, Self::Error> {
        match value.type_url.as_str() {
            "/st.store.v1beta1.MsgVal" => {
                let msg = MsgSend::decode::<Bytes>(value.value.clone().into())?;
                Ok(Message::Send(msg))
            }
            _ => Err(proto_messages::Error::DecodeGeneral(
                "message type not recognized".into(),
            )),
        }
    }
}