use decide_protocol::{ComponentRequest, proto, PUB_ENDPOINT, RequestType};
use tokio;
use prost::Message;
use prost_types::Any;
use tmq::{publish, Context, Result, subscribe, Multipart};
use decide_rs_lights::components_proto;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    decide_rs_lights::set_parameters();
    decide_rs_lights::set_state();
    println!("Init completed");
    let ctx = Context::new();
    let mut hs_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/house-light").unwrap();

    let mut pb_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/peck-keys").unwrap();
    println!("Beginning loop");
    loop {
        tokio::select! {
            hs_state = hs_listen.next() => {
                let hs_state = hs_state.map(|message|{
                    let mut message = message.unwrap();
                    println!("received house-light pub");
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
                    println!("received peck-keys pub");
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




