use core::{error::{ConduitResult, MyAppError}, users::{repository::DynUsersRepository, service::UsersService}, utils::{security_service::DynSecurityService, token_service::DynTokenService}};

use anyhow::Ok;
use async_trait::async_trait;
use domain::user::{request::RegisterUserDto, user::User};
use tracing::{error, info};


#[derive(Clone)]
pub struct ConduitUsersService {
    repository: DynUsersRepository,
    security_service: DynSecurityService,
    token_service: DynTokenService,
}

impl ConduitUsersService {
    pub fn new(
        repository: DynUsersRepository,
        security_service: DynSecurityService,
        token_service: DynTokenService,
    ) -> Self {
        Self {
            repository,
            security_service,
            token_service,
        }
    }
}

