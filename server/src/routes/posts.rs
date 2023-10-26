use crate::routes::uses::*;

pub async fn get<PR: PostRepository>(repo: web::Data<PR>) -> impl Responder {
    try_into_responder!({
        let models = repo.all().await?;
        let jsons = models
            .into_iter()
            .map(Post::from_model)
            .try_collect::<Vec<_>>()?;

        HttpResponse::Ok().json(jsons)
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub content: String,
}

pub async fn create<PR: PostRepository>(
    repo: web::Data<PR>,
    data: web::Json<Create>,
) -> impl Responder {
    try_into_responder!({
        let Create { content } = data.into_inner();

        let id = rand::random();
        let now = chrono::Local::now().fixed_offset();

        let model = models::Post::new(id, content, now);
        let id = model.id;

        repo.create(model).await?;
        let model = repo.find_one(id).await?;

        HttpResponse::Ok().json(model.map(Post::from_model).transpose()?)
    })
}

pub mod _id_ {
    use crate::routes::uses::*;

    pub async fn get<PR: PostRepository>(
        repo: web::Data<PR>,
        id: web::Path<u32>,
    ) -> impl Responder {
        try_into_responder!({
            let model = repo
                .find_one(*id)
                .await?
                .map(Post::from_model)
                .transpose()?;

            HttpResponse::Ok().json(model)
        })
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum Update {
        #[serde(rename_all = "camelCase")]
        Modify { content: String },

        #[serde(rename_all = "camelCase")]
        Deleting { is_deleted: bool },
    }

    pub async fn update<PR: PostRepository>(
        repo: web::Data<PR>,
        id: web::Path<u32>,
        data: web::Json<Update>,
    ) -> impl Responder {
        try_into_responder!({
            match data.into_inner() {
                Update::Modify { content } => {
                    let now = chrono::Local::now().fixed_offset();
                    repo.update(*id, content, now).await?;
                },
                Update::Deleting { is_deleted: true } => {
                    repo.delete(*id).await?;
                },
                Update::Deleting { is_deleted: false } => {
                    repo.restore(*id).await?;
                },
            }

            let model = repo.find_one(*id).await?;
            HttpResponse::Ok().json(model.map(Post::from_model).transpose()?)
        })
    }
}
