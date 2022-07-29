use crate::diesel::ExpressionMethods;
use crate::listener::PgPool;
use crate::schema::ethdto::dsl::ethdto;
use crate::schema::ethdto::dsl::{chain_id, contract, owner};
use crate::schema::ethdto::token_id;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use error_stack::{IntoReport, Result, ResultExt};

use super::dto::{EthDto, NewEthDto};
use super::errors::RepoError;

#[derive(Clone)]
pub struct EthRepo {
    pool: PgPool,
}

impl EthRepo {
    pub fn new(pool: PgPool) -> EthRepo {
        EthRepo { pool }
    }
}

impl EthRepo {
    fn _get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, RepoError> {
        self.pool
            .get()
            .report()
            .change_context(RepoError::FailedToGetConnection)
    }

    pub fn nfts(&self, chain: i64, owner_address: String) -> Result<Vec<EthDto>, RepoError> {
        ethdto
            .filter(chain_id.eq(chain))
            .filter(owner.eq(owner_address.clone()))
            .load(&self._get_conn()?)
            .report()
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
                .first::<EthDto>(&self._get_conn()?)
                .optional()
                .report()
                .attach_printable_lazy(|| {
                    format!("Encountered Error While Querying This NFT {:?}", nft)
                })
                .change_context(RepoError::FailedToQuery)?;

            match opt {
                Some(ent) => {
                    diesel::update(&ent)
                        .set(owner.eq(&nft.owner))
                        .execute(&self._get_conn()?)
                        .report()
                        .attach_printable_lazy(|| {
                            format!("Failed to update {ent:?} with values {nft:?}")
                        })
                        .change_context(RepoError::DatabaseError)?;
                }
                None => {
                    diesel::insert_into(ethdto)
                        .values(nft)
                        .execute(&self._get_conn()?)
                        .report()
                        .attach_printable_lazy(|| format!("Failed to insert {nft:?}"))
                        .change_context(RepoError::DatabaseError)?;
                }
            }
        }
        Ok(())
    }

    pub fn in_or_up_gen(&self, nfts: &[NewEthDto]) -> Result<(), RepoError> {
        for nft in nfts {
            let opt = ethdto
                .filter(chain_id.eq(&nft.chain_id))
                .filter(contract.eq(&nft.contract))
                .filter(token_id.eq(&nft.token_id))
                .first::<EthDto>(&self._get_conn()?)
                .optional()
                .report()
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
                                .execute(&self._get_conn()?)
                                .report()
                                .attach_printable_lazy(|| format!("Failed to delete {ent:?}"))
                                .change_context(RepoError::DatabaseError)?;
                            return Ok(());
                        }
                        diesel::update(&ent)
                            .set(owner.eq(&nft.owner))
                            .execute(&self._get_conn()?)
                            .report()
                            .attach_printable_lazy(|| {
                                format!("Failed to update {ent:?} with values {nft:?}")
                            })
                            .change_context(RepoError::DatabaseError)?;
                        return Ok(());
                    }
                }
                None => {
                    diesel::insert_into(ethdto)
                        .values(nft)
                        .on_conflict((chain_id, token_id, contract))
                        .do_update()
                        .set(owner.eq(&nft.owner))
                        .execute(&self._get_conn()?)
                        .report()
                        .attach_printable_lazy(|| format!("Failed to insert {nft:?}"))
                        .change_context(RepoError::DatabaseError)?;
                    return Ok(());
                }
            }
        }
        return Ok(());
    }
}
