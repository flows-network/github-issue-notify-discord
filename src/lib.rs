use discord_flows::create_text_message_in_channel;
use github_flows::{listen_to_event, EventPayload};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "a-test";
    let label_watch_list = vec![
        "good first issue".to_string(),
        "help wanted".to_string(),
        "LFX mentorship".to_string(),
        "OSPP".to_string(),
        "Hacktoberfest".to_string(),
    ];
    let guild_name = "myserver";
    let channel_name = "general";

    listen_to_event(
        login,
        owner,
        repo,
        vec!["issues", "issue_comment"],
        |payload| handler(guild_name, channel_name, payload, &label_watch_list),
    )
    .await;
}

async fn handler(
    guild_name: &str,
    channel_name: &str,
    payload: EventPayload,
    label_watch_list: &Vec<String>,
) {
    let lowercase_list = label_watch_list
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<String>>();

    match payload {
        EventPayload::IssuesEvent(e) => {
            let issue = e.issue;
            let issue_title = issue.title;
            let issue_url = issue.html_url;
            let user = issue.user.login;
            let labels = issue.labels;

            for label in labels {
                let label_name = label.name;
                if lowercase_list.contains(&label_name.to_lowercase()) {
                    let body = format!(
                        "{label_name}: {issue_title} by {user}\n 
                            {issue_url}"
                    );
                    create_text_message_in_channel(guild_name, channel_name, body, None);

                    return;
                }
            }
        }

        EventPayload::IssueCommentEvent(e) => {
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
                            "A new comment is added for {issue_title}: {comment_content}\n {comment_url}"
                        );
                    create_text_message_in_channel(guild_name, channel_name, body, None);

                    return;
                }
            }
        }

        _ => (),
    }
}
