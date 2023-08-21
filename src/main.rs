use anyhow::{Context, Ok, Result, bail};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, StdoutLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,

    #[serde(flatten)]
    payload: Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk, 
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::InitOk { .. } => { bail!("Received InitOk message")},
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: input.body.id,
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                reply
                    .serialize(output)
                    .context("serialized response to init")?;
                self.id += 1;
            },
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: input.body.id,
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                reply
                    .serialize(output)
                    .context("serialized response to echo")?;
                self.id += 1;
            }
            Payload::EchoOk { echo } => {
                println!("{}: {}", self.id, echo);
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let stdin = stdin().lock();
    let mut inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let stdout = stdout().lock();
    let mut output = serde_json::Serializer::new(stdout);

    let mut state = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("Failed to read input")?;
        state
            .step(input, &mut output)
            .context("Failed to process input")?;
    }

    Ok(())
}
