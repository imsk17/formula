use crate::diesel::ExpressionMethods;

use crate::listener::PgPool;
use crate::schema::ethdto::dsl::ethdto;
use crate::schema::ethdto::dsl::{chain_id, owner};

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use error_stack::{IntoReport, Result, ResultExt};

use super::dto::EthDto;
use super::errors::RepoError;

#[derive(Clone)]
pub struct EthReadRepo {
    pool: PgPool,
}

impl EthReadRepo {
    pub fn new(pool: PgPool) -> EthReadRepo {
        EthReadRepo { pool }
    }
}

impl EthReadRepo {
    fn _get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, RepoError> {
        self.pool
            .get()
            .into_report()
            .change_context(RepoError::FailedToGetConnection)
    }

    pub fn nfts(&self, chain: i64, owner_address: String) -> Result<Vec<EthDto>, RepoError> {
        ethdto
            .filter(chain_id.eq(chain))
            .filter(owner.eq(owner_address.clone()))
            .load(&mut self._get_conn()?)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Failed to get NFTs for chain {} and owner {}",
                    chain, owner_address
                )
            })
            .change_context(RepoError::FailedToQuery)
    }
}
