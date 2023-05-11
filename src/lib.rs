use discord_flows::create_text_message_in_channel;
use github_flows::{
    listen_to_event,
    octocrab::models::events::payload::{IssueCommentEventAction, IssuesEventAction},
    EventPayload,
    GithubLogin::Provided,
};

use dotenv::dotenv;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let github_login = env::var("github_login").unwrap_or("alabulei1".to_string());
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());
    let label_watch_list = vec![
        "good first issue".to_string(),
        "help wanted".to_string(),
        "LFX mentorship".to_string(),
        "OSPP".to_string(),
        "Hacktoberfest".to_string(),
    ];

    listen_to_event(
        &Provided(github_login),
        &github_owner,
        &github_repo,
        vec!["issues", "issue_comment"],
        |payload| handler(&label_watch_list, payload),
    )
    .await;
}

async fn handler(label_watch_list: &Vec<String>, payload: EventPayload) {
    let discord_server = env::var("discord_server").unwrap_or("Vivian Hu's server".to_string());
    let discord_channel = env::var("discord_channel").unwrap_or("general".to_string());

    let lowercase_list = label_watch_list
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<String>>();

    match payload {
        EventPayload::IssuesEvent(e) => {
            if e.action == IssuesEventAction::Closed || e.action == IssuesEventAction::Edited {
                return;
            }
            let issue = e.issue;
            let issue_title = issue.title;
            let issue_url = issue.html_url;
            let user = issue.user.login;
            let labels = issue.labels;

            for label in labels {
                let label_name = label.name;
                if lowercase_list.contains(&label_name.to_lowercase()) {
                    let body = format!("{label_name}: {issue_title} by {user}\n{issue_url}");
                    create_text_message_in_channel(&discord_server, &discord_channel, body, None);


                    return;
                }
            }
        }

        EventPayload::IssueCommentEvent(e) => {
            if e.action == IssueCommentEventAction::Deleted
                || e.action == IssueCommentEventAction::Edited
            {
                return;
            }

            let issue = e.issue;
            let comment = e.comment;
            let issue_title = issue.title;
            let labels = issue.labels;
            let comment_url = comment.html_url;
            let comment_content = comment.body.unwrap();

            for label in labels {
                let label_name = label.name.to_lowercase();
                if lowercase_list.contains(&label_name) {
                    let body = format!(
                            "A new comment is added for {issue_title}: {comment_content}\n{comment_url}"
                        );
                    create_text_message_in_channel(&discord_server, &discord_channel, body, None);

                    return;
                }
            }
        }

        _ => (),
    }
}
