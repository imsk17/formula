use std::sync::Arc;

use crate::diesel::ExpressionMethods;
use crate::erc165::service::Erc165Res;
use crate::listener::PgPool;
use crate::schema::ethdto::dsl::ethdto;
use crate::schema::ethdto::dsl::{chain_id, contract, owner};
use crate::schema::ethdto::token_id;
use crate::uri_getter::UriGetter;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use error_stack::{Report, Result, ResultExt};
use tokio::task::JoinHandle;
use tracing::{debug, error};

use super::dto::{EthDto, NewEthDto};
use super::errors::RepoError;
use crate::listener::ethnftid::EthNftId;
use crate::uri_getter::eth_uri_getter::EthUriGetter;
#[derive(Clone)]
pub struct EthWriteRepo {
    pool: Arc<PgPool>,
    uri_getter: EthUriGetter,
}

impl EthWriteRepo {
    pub fn new(pool: Arc<PgPool>, uri_getter: EthUriGetter) -> EthWriteRepo {
        EthWriteRepo { pool, uri_getter }
    }
}

impl EthWriteRepo {
    fn _get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, RepoError> {
        self.pool
            .get()
            .map_err(Report::from)
            .change_context(RepoError::FailedToGetConnection)
    }

    pub fn nfts(&self, chain: i64, owner_address: String) -> Result<Vec<EthDto>, RepoError> {
        ethdto
            .filter(chain_id.eq(chain))
            .filter(owner.eq(&owner_address))
            .load(&mut self._get_conn()?)
            .map_err(Report::from)
            .attach_printable_lazy(|| {
                format!(
                    "Failed to get NFTs for chain {} and owner {}",
                    chain, owner_address
                )
            })
            .change_context(RepoError::FailedToQuery)
    }

    pub fn full_in_or_up_gen(&self, nfts: &[NewEthDto]) -> Result<(), RepoError> {
        for nft in nfts {
            let opt = ethdto
                .filter(chain_id.eq(&nft.chain_id))
                .filter(contract.eq(&nft.contract))
                .filter(token_id.eq(&nft.token_id))
                .first::<EthDto>(&mut self._get_conn()?)
                .optional()
                .map_err(Report::from)
                .attach_printable_lazy(|| {
                    format!("Encountered Error While Querying This NFT {:?}", nft)
                })
                .change_context(RepoError::FailedToQuery)?;

            match opt {
                Some(ent) => {
                    diesel::update(&ent)
                        .set(owner.eq(&nft.owner))
                        .execute(&mut self._get_conn()?)
                        .map_err(Report::from)
                        .attach_printable_lazy(|| {
                            format!("Failed to update {ent:?} with values {nft:?}")
                        })
                        .change_context(RepoError::DatabaseError)?;
                }
                None => {
                    diesel::insert_into(ethdto)
                        .values(nft)
                        .execute(&mut self._get_conn()?)
                        .map_err(Report::from)
                        .attach_printable_lazy(|| format!("Failed to insert {nft:?}"))
                        .change_context(RepoError::DatabaseError)?;
                }
            }
        }
        Ok(())
    }

    pub async fn in_or_up_gen(&self, ids: &[EthNftId], res: Erc165Res) -> Result<(), RepoError> {
        debug!("Upserting {} nfts into the database", ids.len());
        for id in ids {
            let opt = self.uri_getter.get_uri(&res, id).await;
            let mut pool = self._get_conn()?;
            let handles: JoinHandle<Result<(), _>> = tokio::task::spawn_blocking(move || {
                if let Some(nft) = opt {
                    let opt = ethdto
                        .filter(chain_id.eq(&nft.chain_id))
                        .filter(contract.eq(&nft.contract))
                        .filter(token_id.eq(&nft.token_id))
                        .first::<EthDto>(&mut pool)
                        .optional()
                        .map_err(Report::from)
                        .attach_printable_lazy(|| {
                            format!("Encountered Error While Querying This NFT {:?}", nft)
                        })
                        .change_context(RepoError::FailedToQuery)?;

                    match opt {
                        Some(ent) => {
                            if nft.updated_at > ent.updated_at {
                                // Its a BURN
                                if nft.owner == "0x0000000000000000000000000000000000000000" {
                                    diesel::delete(&ent)
                                        .execute(&mut pool)
                                        .map_err(Report::from)
                                        .attach_printable_lazy(|| {
                                            format!("Failed to delete {ent:?}")
                                        })
                                        .change_context(RepoError::DatabaseError)?;
                                }
                                diesel::update(&ent)
                                    .set(owner.eq(&nft.owner))
                                    .execute(&mut pool)
                                    .map_err(Report::from)
                                    .attach_printable_lazy(|| {
                                        format!("Failed to update {ent:?} with values {nft:?}")
                                    })
                                    .change_context(RepoError::DatabaseError)?;
                            }
                        }
                        None => {
                            diesel::insert_into(ethdto)
                                .values(&nft)
                                .on_conflict((chain_id, token_id, contract))
                                .do_update()
                                .set(owner.eq(&nft.owner))
                                .execute(&mut pool)
                                .map_err(Report::from)
                                .attach_printable_lazy(|| format!("Failed to insert {nft:?}"))
                                .change_context(RepoError::DatabaseError)?;
                        }
                    }
                }
                Ok(())
            });
            handles
                .await
                .map_err(|e| {
                    error!("{}", e);
                    RepoError::DatabaseError
                })?
                .ok();
        }
        return Ok(());
    }
}
