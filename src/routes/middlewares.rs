use crate::app::Ctx;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};

pub async fn auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let ctx = req.app_data::<web::Data<Ctx>>().unwrap();
    if credentials.token() == ctx.config.api_key {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default);

        Err(AuthenticationError::from(config).into())
    }
}
