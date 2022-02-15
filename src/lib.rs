use prost::Message;
use decide_protocol::{
    proto::{reply, ComponentParams, Config, Pub, Reply, StateChange},
    Component, ComponentName, ComponentRequest, GeneralRequest, Request, RequestType, PUB_ENDPOINT,
    REQ_ENDPOINT,
};
use tokio;
use prost;
use prost_types::Any;
use tmq::{publish, Context, Result, subscribe, Multipart};
use tmq::subscribe::Subscribe;
use futures::stream::StreamExt;

const HS_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/house_light_state";
const HS_STATE_TYPE_URL: &'static str = "melizalab.org/proto/house_light_state";
const PK_STATE_TYPE_URL: &'static str = "melizalab.org/proto/key_state";
const PK_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/key_params";

pub mod components_proto {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub async fn set_parameters() {
    let params = Any {
        type_url: String::from(HS_PARAMS_TYPE_URL),
        value: components_proto::HouseLightParams {
            clock_interval: 300
        }.encode_to_vec(),
    };
    let params_message = ComponentParams {
        parameters: Some(params.clone()),
    };
    let request = Request {
        request_type: RequestType::Component(ComponentRequest::SetParameters),
        component: Some(ComponentName(String::from("house-light"))),
        body: params_message.encode_to_vec(),
    };
    let result = send_request(request).await.unwrap();
    assert_eq!(result, reply::Result::Ok(()));
    let request = Request {
        request_type: RequestType::Component(ComponentRequest::GetParameters),
        component: Some(ComponentName::from("house-light")),
        body: vec![],
    };
    let result = send_request(request).await.unwrap();
    assert_eq!(result, reply::Result::Params(params));
}
pub async fn set_state() {
    let state = Any {
        type_url: String::from(HS_STATE_TYPE_URL),
        value: components_proto::HouseLightState {
            switch: true,
            fake_clock: true,
            brightness: 255,
        }.encode_to_vec(),
    };
    let state_message = StateChange {
        state: Some(state.clone()),
    };
    let request = Request {
        request_type: RequestType::Component(ComponentRequest::ChangeState),
        component: Some(ComponentName::from("house-light")),
        body: state_message.encode_to_vec(),
    };
    // the subscriber must be initialized before the state change is
    // sent because the publish socket doesn't buffer messages
    let result = send_request(request).await.unwrap();
    assert_eq!(result, reply::Result::Ok(()));
}
/*pub async fn log_handler() {
    let socket = subscribe(&Context::new())
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"log/info")
        .map(|message| {
            let mut message = message.unwrap();
            trace!("received log pub");
            let _topic = message.pop_front().unwrap();
            let encoded_pub = message.pop_front().unwrap();
            Pub::decode(&encoded_pub[..]).expect("could not decode protobuf")
        });
}*/
async fn send_request(message: Request) -> Result<reply::Result> {
    let ctx = Context::new();
    tracing::trace!("trying to connect");
    let req_sock = tmq::request(&ctx).connect(REQ_ENDPOINT)?;
    tracing::trace!("connected");

    let message = Multipart::from(message);
    tracing::trace!("trying to send message");
    let reply_sock = req_sock.send(message).await?;
    tracing::trace!("sent message");
    let (multipart, _req) = reply_sock.recv().await?;
    tracing::trace!("received reply");
    let reply = Reply::from(multipart);
    println!("{:?}", reply);
    Ok(reply.result.unwrap())
}