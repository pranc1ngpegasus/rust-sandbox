use derive_new::new;
use std::{str::FromStr, sync::Arc};
use tonic::{Request, Response, Status};
use trait_usecase::Usecase;
use user_model::user_id::UserID;
use user_proto::user::v1::{
    user_service_server::UserService, QueryUsersByIdsRequest, QueryUsersByIdsResponse, User,
};

#[derive(new)]
pub struct UserServiceImpl {
    query_users_by_ids_usecase: Arc<
        dyn Usecase<
            Input = user_usecase::query_users_by_ids::Input,
            Output = user_usecase::query_users_by_ids::Output,
        >,
    >,
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn query_users_by_ids(
        &self,
        req: Request<QueryUsersByIdsRequest>,
    ) -> Result<Response<QueryUsersByIdsResponse>, Status> {
        let ids: Vec<UserID> = req
            .get_ref()
            .ids
            .iter()
            .map(|id| UserID::from_str(id))
            .collect::<anyhow::Result<_>>()
            .map_err(|e| Status::internal(e.to_string()))?;

        match self
            .query_users_by_ids_usecase
            .handle(&user_usecase::query_users_by_ids::Input { ids })
            .await
        {
            Ok(result) => Ok(Response::new(QueryUsersByIdsResponse {
                users: result
                    .users
                    .into_iter()
                    .map(|user| User {
                        id: user.id.inner().to_string(),
                    })
                    .collect(),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
