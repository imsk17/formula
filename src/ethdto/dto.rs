use crate::schema::ethdto;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone, Identifiable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = ethdto)]
pub struct EthDto {
    pub id: i32,
    pub contract: String,
    pub chain_id: i64,
    pub contract_type: String,
    pub token_id: String,
    pub owner: String,
    pub uri: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    #[serde(skip)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = ethdto)]
pub struct NewEthDto {
    pub contract: String,
    pub chain_id: i64,
    pub contract_type: String,
    pub token_id: String,
    pub owner: String,
    pub uri: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub updated_at: NaiveDateTime,
}
