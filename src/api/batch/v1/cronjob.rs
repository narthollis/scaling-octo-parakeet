use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "batch.narthollis.net",
    version = "v1",
    kind = "CronJob",
    plural = "cronjobs",
    namespaced
)]
#[kube(status = "CronJobStatus")]
#[kube(
    printcolumn = r#"{"name":"Is Bad", "type":"string", "description":"Is Bad", "jsonPath":".status.is_bad"}"#
)]
pub struct CronJobSpec {
    #[schemars(title = "Name", description = "Name of the thing with the stuff")]
    name: String,
    #[schemars(title = "Information", description = "Info things and others")]
    info: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct CronJobStatus {
    #[schemars(title = "Is Bad", description = "Is the thing bad?")]
    is_bad: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        title = "Last Updated",
        description = "Last time the thing was updated"
    )]
    last_updated: Option<Time>,
}
