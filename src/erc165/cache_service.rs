use std::collections::{HashMap, HashSet};

use std::sync::Arc;

use crate::erc165::errors::Erc165ServiceErrors;
use crate::erc165::model::{Erc165Dto, NewErc165Dto};
use crate::erc165::service::Erc165Res;
use crate::erc165::service::Erc165Service;
use crate::schema::erc165dto::{chain_id, contract};

use diesel::pg::PgConnection;

use diesel::{
    r2d2,
    r2d2::{ConnectionManager, Pool},
};
use diesel::{QueryDsl, RunQueryDsl};
use ethers::prelude::H160;
use ethers::utils;

use super::erc165_interfaces::*;
use super::network_service::Erc165NetworkService;

pub struct Erc165CacheService {
    db: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
    network_service: Erc165NetworkService,
    chain_id: i64,
}

impl Erc165CacheService {
    pub fn new(
        conn: Pool<ConnectionManager<PgConnection>>,
        network_service: Erc165NetworkService,
        chainid: i64,
    ) -> Self {
        Erc165CacheService {
            db: Arc::new(conn),
            network_service,
            chain_id: chainid,
        }
    }
}

#[async_trait::async_trait]
impl Erc165Service for Erc165CacheService {
    async fn supported_traits(
        &self,
        contracts: &[&H160],
    ) -> Result<Erc165Res, Erc165ServiceErrors> {
        use crate::diesel::ExpressionMethods;
        use crate::schema::erc165dto::dsl::erc165dto;

        let mut result = HashMap::new();
        let mut to_find = vec![];
        contracts.iter().for_each(|c| {
            let entity = erc165dto
                .filter(contract.eq(utils::to_checksum(c, None)))
                .filter(chain_id.eq(self.chain_id))
                .first::<Erc165Dto>(&*self.db.get().unwrap());

            if let Ok(e) = entity {
                let ent: (String, HashSet<Erc165Interface>) = e.into();
                result.insert(ent.0, ent.1);
            } else {
                to_find.push(*c);
            }
        });

        let new = self.network_service.supported_traits(&to_find).await?;
        let insert = new
            .iter()
            .map(|(k, v)| NewErc165Dto::new(k.clone(), self.chain_id, v))
            .collect::<Vec<NewErc165Dto>>();

        diesel::insert_into(erc165dto)
            .values(insert)
            .on_conflict_do_nothing()
            .execute(&*self.db.get().unwrap())
            .map_err(|e| Erc165ServiceErrors::FailedToQueryDB(e))?;

        result.extend(new);
        Ok(result)
    }
}
