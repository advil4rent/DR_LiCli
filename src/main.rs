use decide_protocol::{ComponentRequest, proto, PUB_ENDPOINT, RequestType};
use futures::stream::StreamExt;
use prost::Message;
use prost_types::Any;
use tmq::{Context, Multipart, publish, Result, subscribe};
use tokio;
use tracing;

use decide_rs_lights::components_proto;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();
    decide_rs_lights::hs_set_parameters().await;
    decide_rs_lights::hs_set_state().await;
    decide_rs_lights::pl_set_parameters().await;
    decide_rs_lights::pl_set_state().await;
    let ctx = Context::new();
    let mut hs_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/house-light").unwrap();

    let mut pb_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/peck-keys").unwrap();
    loop {
        tokio::select! {
            hs_state = hs_listen.next() => {
                let hs_state = hs_state.map(|message|{
                    let mut message = message.unwrap();
                    tracing::info!("received house-light pub");
                    let _topic = message.pop_front().unwrap();
                    let encoded_pub = message.pop_front().unwrap();
                    let decoded_pub = proto::Pub::decode(&encoded_pub[..]).expect("could not decode protobuf");
                    components_proto::HouseLightState::decode(&*decoded_pub.state.unwrap().value)
                    .expect("could not decode state change")})
                .unwrap();
                println!("Changed house-light state: switch {:?}, fake_clock {:?}, brightness {:?}", &hs_state.switch,  &hs_state.fake_clock, &hs_state.brightness)}
            pb_state = pb_listen.next() => {
                let pb_state = pb_state.map(|message|{
                    let mut message = message.unwrap();
                    tracing::info!("received peck-keys pub");
                    let _topic = message.pop_front().unwrap();
                    let encoded_pub = message.pop_front().unwrap();
                    let decoded_pub = proto::Pub::decode(&encoded_pub[..]).expect("could not decode protobuf");
                    components_proto::KeyState::decode(&*decoded_pub.state.unwrap().value)
                    .expect("could not decode state change")})
                .unwrap();
                println!("Changed peck-key state: left {:?}, center {:?}, right {:?}", &pb_state.peck_left,  &pb_state.peck_center, &pb_state.peck_right)}
        }
    }
}




