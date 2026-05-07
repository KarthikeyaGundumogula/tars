mod utils;
use chrono::Utc;
use reqwest::{Client, Response};

use crate::utils::setups::setup_original_registration;

#[tokio::test]
async fn create_original_return_success_on_correct_data() {
    let (artists, app) = setup_original_registration().await;
    let client = Client::new();

    let body = serde_json::json!({
        "title":"They Call him Og",
        "description":"fuck you staya dada",
        "cover_img":"canada is fucked",
        "password": "Kap@123456",
        "associated_with":artists[0], // this is an uuid that can be set
        "release_date":Utc::now(),
        "genres":["action","drama"],
        "stars":[{
            "role":"Ojas Ghambheera",
            "artist":artists[1] // this is also a uuid
        },{
            "role":"Kanmani",
            "artist":artists[2]
        }],
        "makers":[{
            "role":"Music Director",
            "artist":artists[3]
        },{
            "role":"Director",
            "artist":artists[1]
        }]
    });
    let response: Response = client
        .post(&format!("{}/originals/new", app.address))
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");
    println!("Response: {:#?}", response);
    assert!(response.status().is_success());
    let description = sqlx::query_scalar!(
        r#"SELECT description FROM originals WHERE title=$1"#,
        "They Call him Og"
    )
    .fetch_one(&app.state.pool)
    .await
    .expect("db query failed");
    let actors: Vec<String> = sqlx::query_scalar!(r#"SELECT role_name FROM roles;"#)
        .fetch_all(&app.state.pool)
        .await
        .expect("db query failed");
    assert_eq!(description, "fuck you staya dada".to_string());
    assert_eq!(actors[0], "Ojas Ghambheera".to_string());
    assert_eq!(actors[1], "Kanmani".to_string());
    println!("{:?}", actors);
}
