use axum::{
    extract::{Path, State},
    routing::get,
    routing::Router,
    Json,
};

use crate::ethdto::{dto::EthDto, read_repo::EthReadRepo};

struct GetNftsResponse;

#[derive(serde::Deserialize)]
struct GetNftQueryParams {
    address: String,
    chain_id: i64,
}

pub fn create_nft_router(ethrepo: EthReadRepo) -> Router {
    Router::new()
        .route("/nfts/:chain_id/:address", get(get_nfts))
        .with_state(ethrepo)
}

async fn get_nfts(
    Path(params): Path<GetNftQueryParams>,
    State(repo): State<EthReadRepo>,
) -> Json<Vec<EthDto>> {
    let nfts = repo.nfts(params.chain_id, params.address).unwrap();

    return Json(nfts);
}
