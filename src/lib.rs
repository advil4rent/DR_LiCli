use decide_protocol::{Component, ComponentName, ComponentRequest, proto::{ComponentParams, reply, Reply, StateChange}, REQ_ENDPOINT, Request, RequestType};
use house_light::proto as hl_proto;
use peckboard::proto as pb_proto;
use prost;
use prost_types::Any;
use prost::Message;
use tmq::{Context, Multipart, Result};

pub async fn hs_set_parameters() {
    let params = Any {
        type_url: String::from(house_light::HouseLight::PARAMS_TYPE_URL),
        value: hl_proto::Params {
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
pub async fn hs_set_state() {
    let state = Any {
        type_url: String::from(house_light::HouseLight::STATE_TYPE_URL),
        value: hl_proto::State {
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
pub async fn pl_set_parameters() {
    let params = Any {
        type_url: String::from(peckboard::PeckLeds::PARAMS_TYPE_URL),
        value: pb_proto::LedParams {
        }.encode_to_vec(),
    };
    let params_message = ComponentParams {
        parameters: Some(params.clone()),
    };
    let request = Request {
        request_type: RequestType::Component(ComponentRequest::SetParameters),
        component: Some(ComponentName(String::from("peck-leds-left"))),
        body: params_message.encode_to_vec(),
    };
    let result = send_request(request).await.unwrap();
    assert_eq!(result, reply::Result::Ok(()));
    let request = Request {
        request_type: RequestType::Component(ComponentRequest::GetParameters),
        component: Some(ComponentName::from("peck-leds-left")),
        body: vec![],
    };
    let result = send_request(request).await.unwrap();
    assert_eq!(result, reply::Result::Params(params));
}
pub async fn pl_set_state() {
    let state = Any {
        type_url: String::from(peckboard::PeckLeds::STATE_TYPE_URL),
        value: pb_proto::LedState {
            led_state: String::from("red")
        }.encode_to_vec(),
    };
    let state_message = StateChange {
        state: Some(state.clone()),
    };
    let request = Request {
        request_type: RequestType::Component(ComponentRequest::ChangeState),
        component: Some(ComponentName::from("peck-leds-left")),
        body: state_message.encode_to_vec(),
    };
    // the subscriber must be initialized before the state change is
    // sent because the publish socket doesn't buffer messages
    let result = send_request(request).await.unwrap();
    assert_eq!(result, reply::Result::Ok(()));
}


async fn send_request(message: Request) -> Result<reply::Result> {
    let ctx = Context::new();
    //tracing::trace!("trying to connect");
    let req_sock = tmq::request(&ctx).connect(REQ_ENDPOINT)?;
    tracing::trace!("connected");

    let message = Multipart::from(message);
    //tracing::trace!("trying to send message");
    let reply_sock = req_sock.send(message).await?;
    tracing::trace!("sent message");
    let (multipart, _req) = reply_sock.recv().await?;
    tracing::trace!("received reply");
    let reply = Reply::from(multipart);
    tracing::trace!("{:?}", reply);
    Ok(reply.result.unwrap())
}