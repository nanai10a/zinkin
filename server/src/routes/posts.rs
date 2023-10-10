use crate::routes::uses::*;

pub async fn get<PR: PostRepository>(repo: web::Data<PR>) -> impl Responder {
    let result: anyhow::Result<_> = try {
        let models = repo.all().await?;
        let jsons = models
            .into_iter()
            .map(Post::from_model)
            .try_collect::<Vec<_>>()?;

        web::Json(jsons)
    };

    result_as_response!(result)
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
    let Create { content } = data.into_inner();

    let id = rand::random();
    let now = chrono::Local::now().fixed_offset();

    let model = models::Post::new(id, content, now);
    let id = model.id;

    let result: anyhow::Result<_> = try {
        repo.create(model).await?;
        let model = repo.find_one(id).await?;

        web::Json(model.map(Post::from_model).transpose()?)
    };

    result_as_response!(result)
}

pub mod _id_ {
    use crate::routes::uses::*;

    pub async fn get<PR: PostRepository>(
        repo: web::Data<PR>,
        id: web::Path<u32>,
    ) -> impl Responder {
        let result: anyhow::Result<_> = try {
            let model = repo
                .find_one(*id)
                .await?
                .map(Post::from_model)
                .transpose()?;

            web::Json(model)
        };

        result_as_response!(result)
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
        let result: anyhow::Result<_> = try {
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
            web::Json(model.map(Post::from_model).transpose()?)
        };

        result_as_response!(result)
    }
}
