use webauthn_rs::prelude as wan;

use crate::auth::Token;
use crate::routes::uses::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(u128);

impl SessionId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self { Self(rand::random()) }
}

impl core::fmt::Display for SessionId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result { write!(f, "{:32x}", self.0) }
}

impl From<u128> for SessionId {
    fn from(val: u128) -> Self { Self(val) }
}

pub async fn register<KR: KeyRepository, RS: Store<wan::PasskeyRegistration, Key = SessionId>>(
    repo: web::Data<KR>,
    store: web::Data<RS>,
    site: web::Data<wan::Webauthn>,
    data: web::Json<Option<wan::RegisterPublicKeyCredential>>,
    mut ck: Cookies,
) -> impl Responder {
    try_into_responder!({
        match (ck.status, &*data) {
            (None | Some(_), None) => {
                let excludes = repo
                    .all()
                    .await?
                    .into_iter()
                    .map(|p| p.cred_id().clone())
                    .collect::<Vec<_>>();

                let (ccr, pr) = site.start_passkey_registration(
                    wan::Uuid::new_v4(),
                    "owner",
                    "owner",
                    Some(excludes),
                )?;

                let id = SessionId::new();

                assert!(store.entry(id).await?.set(pr).await?);

                ck.status.replace(id);

                HttpResponse::Accepted().apply_cookies(ck)?.json(ccr)
            },

            (Some(id), Some(data)) => {
                let pr =
                    store.entry(id).await?.get().await?.ok_or_else(|| {
                        actix_web::error::ErrorBadRequest("registration isn't found")
                    })?;

                let result = site.finish_passkey_registration(data, &pr);

                if let Ok(ref p) = result {
                    repo.push(rand::random(), p.clone()).await?;
                }

                ck.status.take();

                HttpResponse::Ok().apply_cookies(ck)?.json(result.is_ok())
            },

            (None, Some(_)) => HttpResponse::BadRequest().finish(),
        }
    })
}

pub async fn claim<KR: KeyRepository, AS: Store<wan::PasskeyAuthentication, Key = SessionId>>(
    repo: web::Data<KR>,
    store: web::Data<AS>,
    site: web::Data<wan::Webauthn>,
    data: web::Json<Option<wan::PublicKeyCredential>>,
    mut ck: Cookies,
) -> impl Responder {
    try_into_responder!({
        if let Some(ref mut token) = ck.refresh {
            *token = Token::issue_refresh();

            return HttpResponse::Ok().apply_cookies(ck)?.finish();
        }

        match (ck.status, &*data) {
            (None | Some(_), None) => {
                let keys = repo.all().await?;

                let (rcr, pa) = site.start_passkey_authentication(&keys)?;

                let id = SessionId::new();

                assert!(store.entry(id).await?.set(pa).await?);

                ck.status.replace(id);

                HttpResponse::Accepted().apply_cookies(ck)?.json(rcr)
            },

            (Some(id), Some(data)) => {
                let pa = match store.entry(id).await?.get().await? {
                    None => return HttpResponse::BadRequest().finish(),
                    Some(pa) => pa,
                };

                let result = site.finish_passkey_authentication(data, &pa);

                if let Ok(_) = result {
                    let token = Token::issue_refresh();
                    ck.refresh.replace(token);
                }

                ck.status.take();

                HttpResponse::Ok().apply_cookies(ck)?.finish()
            },

            (None, Some(_)) => HttpResponse::BadRequest().finish(),
        }
    })
}

pub async fn refresh(mut ck: Cookies) -> impl Responder {
    try_into_responder!({
        match ck.refresh {
            None => HttpResponse::Unauthorized().finish(),
            Some(_) => {
                let token = Token::issue_session();
                ck.session.replace(token);
                HttpResponse::Ok().apply_cookies(ck)?.finish()
            },
        }
    })
}
