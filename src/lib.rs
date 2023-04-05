use discord_flows::create_text_message_in_dm;
use github_flows::{listen_to_event, EventPayload};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "a-test";
    let label_watch_list = vec!["good first issue".to_string()];
    let username = "jaykchen";

    listen_to_event(
        login,
        owner,
        repo,
        vec!["issues", "issue_comment"],
        |payload| handler(&username, payload, &label_watch_list),
    )
    .await;
}

async fn handler(username: &str, payload: EventPayload, label_watch_list: &Vec<String>) {
    let lowercase_list = label_watch_list
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<String>>();

    let mut issue = None;

    match payload {
        EventPayload::IssuesEvent(e) => {
            issue = Some(e.issue);
        }

        EventPayload::IssueCommentEvent(e) => {
            issue = Some(e.issue);
        }

        _ => (),
    }

    if let Some(issue) = issue {
        let issue_title = issue.title;
        let issue_url = issue.html_url;
        let user = issue.user.login;
        let labels = issue.labels;

        for label in labels {
            let label_name = label.name.to_lowercase();
            if lowercase_list.contains(&label_name) {
                let body = format!(
                    "A good first issue: {issue_title} by {user}\n 
                    {issue_url}"
                );
                create_text_message_in_dm(username, body, None);

                return;
            }
        }
    }
}
