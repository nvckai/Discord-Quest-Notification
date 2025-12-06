use serde_json::json;

/// Data structure for building Discord webhook payload
pub struct WebhookPayloadData<'a> {
    pub accent_color: i32,
    pub game_title: &'a str,
    pub game_publisher: &'a str,
    pub cta_link: &'a str,
    pub banner_url: String,
    pub start_timestamp: i64,
    pub expires_timestamp: i64,
    pub app_name: &'a str,
    pub app_link: &'a str,
    pub app_id: &'a str,
    pub features_string: String,
    pub tasks_string: String,
    pub reward_icon_url: String,
    pub reward_info: String,
    pub quest_id: &'a str,
}

/// Build Discord webhook payload using Components V2 format
#[must_use]
pub fn build_webhook_payload(data: &WebhookPayloadData) -> serde_json::Value {
    json!({
        "components": [
            {
                "type": 17,
                "accent_color": data.accent_color,
                "components": [
                    {
                        "type": 10,
                        "content": format!("## **New Quest** - [{}]({})", data.game_title, data.cta_link)
                    },
                    {
                        "type": 12,
                        "items": [
                            {
                                "media": {
                                    "url": data.banner_url
                                },
                                "description": null,
                                "spoiler": false
                            }
                        ]
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10,
                        "content": format!(
                            "\n# Quest Info\n**Duration**: <t:{}:d> - <t:{}:d>\n**Reedemable Platforms**: Cross Platform\n**Game**: {} ({})\n**Application**: [{}]({}) (``{}``)\n**Features**: {}",
                            data.start_timestamp,
                            data.expires_timestamp,
                            data.game_title,
                            data.game_publisher,
                            data.app_name,
                            data.app_link,
                            data.app_id,
                            data.features_string
                        )
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10,
                        "content": format!("# Tasks\nUser must complete any of the following tasks\n{}", data.tasks_string)
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 9,
                        "accessory": {
                            "type": 11,
                            "media": {
                                "url": data.reward_icon_url
                            },
                            "description": null,
                            "spoiler": false
                        },
                        "components": [
                            {
                                "type": 10,
                                "content": format!("# Rewards\n{}", data.reward_info)
                            }
                        ]
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10,
                        "content": format!("Quest ID: `{}`", data.quest_id)
                    }
                ]
            }
        ],
        "flags": 32768
    })
}
