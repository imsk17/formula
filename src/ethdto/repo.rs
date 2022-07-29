use crate::diesel::ExpressionMethods;
use crate::listener::PgPool;
use crate::schema::ethdto::dsl::ethdto;
use crate::schema::ethdto::dsl::{chain_id, contract, owner};
use crate::schema::ethdto::token_id;
use diesel::result::Error;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use error_stack::{IntoReport, ResultExt};

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
    pub fn nfts(&self, chain: i64, owner_address: String) -> Result<Vec<EthDto>, RepoError> {
        let result: Result<Vec<EthDto>, Error> = ethdto
            .filter(chain_id.eq(chain))
            .filter(owner.eq(owner_address))
            .load(&*self.pool.get().unwrap());
        match result {
            Ok(r) => Ok(r),
            Err(e) => match e {
                Error::NotFound => Err(RepoError::NoEntityFound),
                _ => todo!(),
            },
        }
    }

    pub fn full_in_or_up_gen(&self, nfts: &[NewEthDto]) -> Result<(), RepoError> {
        for nft in nfts {
            let ent = ethdto
                .filter(chain_id.eq(&nft.chain_id))
                .filter(contract.eq(&nft.contract))
                .filter(token_id.eq(&nft.token_id))
                .first::<EthDto>(&*self.pool.get().unwrap())
                .optional();
            if let Ok(opt) = ent {
                match opt {
                    Some(ent) => {
                        diesel::update(&ent)
                            .set(owner.eq(&nft.owner))
                            .execute(&*self.pool.get().unwrap())
                            .unwrap();
                    }
                    None => {
                        diesel::insert_into(ethdto)
                            .values(nft)
                            .execute(&*self.pool.get().unwrap())
                            .unwrap();
                    }
                }
            } else {
                panic!("handle error")
            }
        }
        Ok(())
    }

    pub fn in_or_up_gen(&self, nfts: &[NewEthDto]) -> Result<(), RepoError> {
        for nft in nfts {
            let ent = ethdto
                .filter(chain_id.eq(&nft.chain_id))
                .filter(contract.eq(&nft.contract))
                .filter(token_id.eq(&nft.token_id))
                .first::<EthDto>(&*self.pool.get().unwrap())
                .optional();
            if let Ok(opt) = ent {
                match opt {
                    Some(ent) => {
                        if nft.updated_at > ent.updated_at {
                            // Its a BURN
                            if nft.owner == "0x0000000000000000000000000000000000000000" {
                                diesel::delete(&ent)
                                    .execute(&*self.pool.get().unwrap())
                                    .unwrap();
                                continue;
                            }
                            diesel::update(&ent)
                                .set(owner.eq(&nft.owner))
                                .execute(&*self.pool.get().unwrap())
                                .unwrap();
                            return Ok(());
                        }
                    }
                    None => {
                        diesel::insert_into(ethdto)
                            .values(nft)
                            .on_conflict((chain_id, token_id, contract))
                            .do_update()
                            .set(owner.eq(&nft.owner))
                            .execute(&*self.pool.get().unwrap())
                            .report()
                            .attach_printable_lazy(|| format!("Failed to insert {nft:?}"))
                            .change_context(RepoError::DatabaseError)
                            .unwrap();
                        return Ok(());
                    }
                }
            }
            panic!("handle error")
        }
        Ok(())
    }
}
