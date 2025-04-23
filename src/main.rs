use serde::Serialize;

use std::error::Error;
use std::convert::Infallible;

use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;

mod utils;
mod ecvrf;
mod rng;

#[derive(Serialize)]
struct ErrorMessage {
    success: bool,
    code: u16,
    message: String
}

#[derive(Serialize)]
struct SuccessMessage<T> {
    success: bool,
    code: u16,
    data: T
}

#[derive(Debug)]
struct UnknownError;
impl warp::reject::Reject for UnknownError {}

impl warp::reject::Reject for ecvrf::AlphaInvalid {}
impl warp::reject::Reject for ecvrf::SKInvalid {}
impl warp::reject::Reject for ecvrf::PKInvalid {}
impl warp::reject::Reject for ecvrf::PiInvalid {}
impl warp::reject::Reject for utils::IntStringInvalid {}
impl warp::reject::Reject for utils::HexStringInvalid {}
impl warp::reject::Reject for rng::IterationsExceeded {}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "api=info");
        }
    }
    pretty_env_logger::init();

    let vrf_prove = warp::path!("api" / "vrf" / "prove")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_vrf_prove);

    let generate_sk = warp::path!("api" / "vrf" / "sk" / "new")
        .and(warp::get())
        .and_then(handle_generate_sk);

    let get_pk = warp::path!("api" / "vrf" / "pk")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_get_pk);

    let vrf_verify = warp::path!("api" / "vrf" / "verify") // -> success, bet
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_vrf_verify);

    let utils_hex = warp::path!("api" / "utils" / "hex") // type: be/le -> value
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_hex);

    let utils_int = warp::path!("api" / "utils" / "int") // type: be/le -> value
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_int);

    let utils_sha256 = warp::path!("api" / "utils" / "sha256") // -> value
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_sha256);

    let utils_sha512 = warp::path!("api" / "utils" / "sha512") // -> value
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_sha512);

    let gen_random = warp::path!("api" / "random") // seed, times, limit -> [ new_seed, ticket ]
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handle_random);

    let routes = vrf_prove
        .or(generate_sk)
        .or(get_pk)
        .or(vrf_verify)
        .or(utils_hex)
        .or(utils_int)
        .or(utils_sha256)
        .or(utils_sha512)
        .or(gen_random)
        .recover(handle_rejection)
        .with(warp::log("api"));

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 3111)).await;
}

async fn handle_vrf_prove(vrf_inputs: ecvrf::VRFInputs) -> Result<impl Reply, Rejection> {
    match ecvrf::api_vrf_prove(vrf_inputs) {
        Ok(vrf_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: vrf_output })),
        Err(ecvrf::VRFInputError::AlphaInvalid(_)) => Err(warp::reject::custom(ecvrf::AlphaInvalid)),
        Err(ecvrf::VRFInputError::SKInvalid(_)) => Err(warp::reject::custom(ecvrf::SKInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_vrf_verify(vrf_verify_inputs: ecvrf::VRFVerifyInputs) -> Result<impl Reply, Rejection> {
    match ecvrf::api_vrf_verify(vrf_verify_inputs) {
        Ok(vrf_verify_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: vrf_verify_output })),
        Err(ecvrf::VRFVerifyInputError::AlphaInvalid(_)) => Err(warp::reject::custom(ecvrf::AlphaInvalid)),
        Err(ecvrf::VRFVerifyInputError::PKInvalid(_)) => Err(warp::reject::custom(ecvrf::PKInvalid)),
        Err(ecvrf::VRFVerifyInputError::PiInvalid(_)) => Err(warp::reject::custom(ecvrf::PiInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_generate_sk() -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: ecvrf::api_generate_sk() }))
}

async fn handle_get_pk(pk_inputs: ecvrf::PKInputs) -> Result<impl Reply, Rejection> {
    match ecvrf::api_get_pk(pk_inputs) {
        Ok(vrf_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: vrf_output })),
        Err(ecvrf::VRFInputError::SKInvalid(_)) => Err(warp::reject::custom(ecvrf::SKInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_hex(convert_inputs: utils::ConvertInputs) -> Result<impl Reply, Rejection> {
    match utils::api_convert_to_hex(convert_inputs) {
        Ok(convert_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: convert_output })),
        Err(utils::ConvertInputError::IntStringInvalid(_)) => Err(warp::reject::custom(utils::IntStringInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_int(convert_inputs: utils::ConvertInputs) -> Result<impl Reply, Rejection> {
    match utils::api_convert_to_int(convert_inputs) {
        Ok(convert_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: convert_output })),
        Err(utils::ConvertInputError::HexStringInvalid(_)) => Err(warp::reject::custom(utils::HexStringInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_sha256(sha_inputs: utils::ShaInputs) -> Result<impl Reply, Rejection> {
    match utils::api_sha256(sha_inputs) {
        Ok(sha_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: sha_output })),
        Err(utils::ConvertInputError::HexStringInvalid(_)) => Err(warp::reject::custom(utils::HexStringInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_sha512(sha_inputs: utils::ShaInputs) -> Result<impl Reply, Rejection> {
    match utils::api_sha512(sha_inputs) {
        Ok(sha_output) => Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: sha_output })),
        Err(utils::ConvertInputError::HexStringInvalid(_)) => Err(warp::reject::custom(utils::HexStringInvalid)),
        Err(_err) => Err(warp::reject::custom(UnknownError))
    }
}

async fn handle_random(rng_inputs: rng::RngInputs) -> Result<impl Reply, Rejection> {
    if rng_inputs.iterations > 3100 {
        return Err(warp::reject::custom(rng::IterationsExceeded));
    }
    Ok(warp::reply::json(&SuccessMessage{ success: true, code: StatusCode::OK.as_u16(), data: rng::api_random(rng_inputs) }))
}

// fn handle_json_body() -> impl Filter<Extract = (HashMap<String, String>,), Error = Rejection> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(ecvrf::AlphaInvalid) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Alpha is invalid.";
    } else if let Some(ecvrf::SKInvalid) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "VRF secret key is invalid.";
    } else if let Some(ecvrf::PiInvalid) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "VRF proof is invalid.";
    } else if let Some(utils::IntStringInvalid) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Integer string is invalid.";
    } else if let Some(utils::HexStringInvalid) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Hex string is invalid.";
    } else if let Some(rng::IterationsExceeded) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Iterations exceed maximum allowed limit of 3100.";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        message = "BAD_REQUEST";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        success: false,
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
