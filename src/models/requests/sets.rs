use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{
    DiscussionCommentContent, DiscussionPostContent, DiscussionPostTitle, SetDescription, SetName,
    Statement,
};

#[derive(Deserialize)]
pub struct CreateSetReq {
    pub name: SetName,
    pub description: SetDescription,
    pub statement: Statement,
    pub color_theme: String,
}

#[derive(Deserialize)]
pub struct JoinSetRequest {
    pub set_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateSetReq {
    pub name: Option<SetName>,
    pub description: Option<SetDescription>,
    pub statement: Option<Statement>,
    pub profile_picture: Option<String>,
}

#[derive(Deserialize)]
pub struct NewDiscussionPostReq {
    pub title: DiscussionPostTitle,
    pub content: DiscussionPostContent,
    pub work_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateDiscussionPostReq {
    pub title: Option<DiscussionPostTitle>,
    pub content: Option<DiscussionPostContent>,
    pub work_id:Option<Uuid>
}

#[derive(Deserialize)]
pub struct NewDiscussionCommentReq {
    pub discussion_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: DiscussionCommentContent,
}

#[derive(Deserialize)]
pub struct UpdateCommentReq{
    pub content:Option<DiscussionCommentContent>,
    pub work_id:Option<Uuid>
}
