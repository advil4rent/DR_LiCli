use decide_protocol::{proto, PUB_ENDPOINT};
use futures::stream::StreamExt;
use prost::Message;
use tmq::{Context, subscribe};
use tokio;
use tracing;

use rusty_2ac;
use house_light::proto as hl_proto;
use peckboard::proto as pb_proto;
use sound_alsa::proto as sa_proto;
use stepper_motor::proto as sm_proto;

const HS_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/house_light_state";
const HS_STATE_TYPE_URL: &'static str = "melizalab.org/proto/house_light_state";
const PL_STATE_TYPE_URL: &'static str = "melizalab.org/proto/led_state";
const PL_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/led_params";
const SA_STATE_TYPE_URL: &'static str = "melizalab.org/proto/sound_alsa_state";
const SA_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/sound_alsa_params";
const SM_TATE_TYPE_URL: &'static str = "melizalab.org/proto/stepper_motor_state";
const SM_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/stepper_motor_params";


struct Experiment {
    subject: String,
    addr: String,
    experiment: String,
    trial: u32,
    stim: String,
    correction: u32,
    lights: String,
    time: String,
    response: String,
    correct: bool,
    result: String,
    rt: u32,
}

const MAX_CORRECTION: u8 = 50;
const AUDIO_DIR: String = String::from("~/colony-noise-stimuli/stimuli/clean-stim");
const MOTOR_TIMEOUT: u64 = 4000;
const LIGHTSOUT_DUR: u64 = 10000;
const EXPERIMENT: String = String::from("2ac_rs");
const CORRECTION_LIGHT: String = String::from("blue");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();

    let hs_param = prost_types:: Any {
        type_url: String::from(HS_PARAMS_TYPE_URL),
        value: hl_proto::Params {
            clock_interval: 300
        }.encode_to_vec(),
    };

    let pl_param = prost_types:: Any {
        type_url: String::from(PL_STATE_TYPE_URL),
        value: pb_proto::KeyParams {}.encode_to_vec(),
    };

    let sm_param = prost_types:: Any {
        type_url: String::from(SM_PARAMS_TYPE_URL),
        value: sm_proto::Params {
            timeout: MOTOR_TIMEOUT
        }.encode_to_vec(),
    };
    let sa_param = prost_types:: Any {
        type_url: String::from(SA_PARAMS_TYPE_URL),
        value: sa_proto::Params {
            audio_dir: AUDIO_DIR,
        }.encode_to_vec(),
    };

    rusty_2ac::set_params("house-light", hs_param);
    rusty_2ac::set_params("stepper-motor", sm_param);
    rusty_2ac::set_params("sound-alsa", sa_param);
    rusty_2ac::set_params("peck-leds-left", pl_param);

    let hl_state = prost_types::Any {
        type_url: String::from(HS_PARAMS_TYPE_URL),
        value: hl_proto::State {
            switch: true,
            light_override: false,
            ephemera: false,
            brightness: 0,
        }.encode_to_vec(),
    };
    rusty_2ac::set_state("house-light", hl_state);

    let ctx = Context::new();
    let mut hs_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/house-light").unwrap();

    let mut pb_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/peck-keys").unwrap();
    let mut sm_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/stepper-motor").unwrap();
    let mut sa_listen = subscribe(&ctx)
        .connect(PUB_ENDPOINT).unwrap()
        .subscribe(b"state/sound-alsa").unwrap();

    let current = Experiment {
        subject: "F5".to_string(),
        addr: "beagle-0".to_string(),
        experiment: EXPERIMENT,
        trial: 0,
        stim: "pmkimwop_30".to_string(),
        correction: 0,
        lights: "none".to_string(),
        time: "".to_string(),
        response: "timeout".to_string(),
        correct: false,
        result: "none".to_string(),
        rt: 0,
    };

    loop {

    }
}




