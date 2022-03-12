use crate::{
    *,
    models::*
};

#[derive(Serialize, Debug, Display)]
#[serde(tag = "type")]
pub enum ListNodesError {
    DBError
}

impl Error for ListNodesError {}

impl From<ListNodesError> for Status {
    fn from(_: ListNodesError) -> Status {
        Status::InternalServerError
    }
}

impl From<DieselError> for ApiError<ListNodesError> {
    fn from(_: DieselError) -> ApiError<ListNodesError> {
        ListNodesError::DBError.into()
    }
}

#[get("/nodes")]
pub fn list_nodes() -> Result<ApiData<Vec<Node>>, ApiError<ListNodesError>> {
    let conn = establish_connection();
    let results = nodesdsl::nodes.get_results(&conn)?;
    Ok(results.into())
}
