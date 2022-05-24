use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::CustomResourceExt;
use std::io::{stdout, Write};
use std::process::exit;
use std::{env, error, fs};

fn crds() -> Vec<CustomResourceDefinition> {
    vec![controller::api::batch::v1::cronjob::CronJob::crd()]
}

fn write(mut writer: impl Write) -> Result<(), Box<dyn error::Error>> {
    crds()
        .into_iter()
        .map(|c| serde_yaml::to_writer(&mut writer, &c).map_err(|e| e.into()))
        .collect()
}

fn main() {
    let path = env::args()
        .nth(1)
        .map(|p| fs::canonicalize(p).ok()) // make the path absolute
        .flatten(); // unwrap the option

    let result = match path {
        Some(p) => fs::File::create(p).map_err(|e| e.into()).and_then(write),
        None => write(stdout()),
    };

    match result {
        Ok(_) => exit(0),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
