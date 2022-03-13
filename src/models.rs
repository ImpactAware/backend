use crate::schema::*;
use crate::schema;
use crate::establish_connection;
use diesel::{expression::NonAggregate, prelude::*, deserialize::FromSql, serialize::ToSql};

pub trait HasId: Sized + Clone + Send + Sync + std::fmt::Debug {
    fn get_id(&self) -> i32;
}

#[derive(Clone, Debug, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = "diesel::sql_types::Integer")]
#[serde(untagged)]
pub enum RowRef<RN: HasId> {
    Id(i32),
    Data(RN)
}

impl<R: HasId> FromSql<diesel::sql_types::Integer, diesel::pg::Pg> for RowRef<R> {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        match <i32 as FromSql<diesel::sql_types::Integer, diesel::pg::Pg>>::from_sql(bytes) {
            Ok(id) => Ok(RowRef::Id(id)),
            Err(e) => Err(e)
        }
    }
}

impl<R: HasId> ToSql<diesel::sql_types::Integer, diesel::pg::Pg> for RowRef<R> {
    fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, diesel::pg::Pg>) -> diesel::serialize::Result {
        <i32 as ToSql<diesel::sql_types::Integer, diesel::pg::Pg>>::to_sql(&self.get_id(), out)
    }
}

impl<R: HasId> RowRef<R> {
    pub fn get_id(&self) -> i32 {
        match self {
            RowRef::Id(id) => *id,
            RowRef::Data(r) => r.get_id()
        }
    }
}

impl<R> From<i32> for RowRef<R> where R: HasId + Clone + std::fmt::Debug {
    fn from(id: i32) -> RowRef<R> {
        RowRef::Id(id)
    }
}

impl<R> From<R> for RowRef<R> where R: HasId + Clone + std::fmt::Debug {
    fn from(r: R) -> RowRef<R> {
        RowRef::Data(r)
    }
}

pub enum Table {
    Classes,
    Customers,
    Meetings,
    Offerings,
    Payments,
    Slots,
    Subscriptions,
    Tutors
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
pub struct Node {
    device_id: i64,
    hits: i32,
    last_hit_at_epoch: i64,
    connected: bool
}
