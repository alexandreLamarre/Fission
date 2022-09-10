pub mod resources;

use futures::stream::StreamExt;
use kube::Resource;
use kube::ResourceExt;
use kube::{
    api::ListParams, client::Client, runtime::controller::Action, runtime::Controller, Api,
};
use resources::example::{
    deployment_delete, deployment_deploy, finalizer_add, finalizer_delete, Echo, EchoAction,
};
use std::sync::Arc;

pub async fn run_controller() {
    let kubernetes_client: Client = Client::try_default()
        .await
        .expect("Cannot find a default kubernetes config");

    let crd_api: Api<Echo> = Api::all(kubernetes_client.clone());
    let context: Arc<ContextData> = Arc::new(ContextData::new(kubernetes_client.clone()));
    // The controller comes from the `kube_runtime` crate and manages the reconciliation process.
    // It requires the following information:
    // - `kube::Api<T>` this controller "owns". In this case, `T = Echo`, as this controller owns the `Echo` resource,
    // - `kube::api::ListParams` to select the `Echo` resources with. Can be used for Echo filtering `Echo` resources before reconciliation,
    // - `reconcile` function with reconciliation logic to be called each time a resource of `Echo` kind is created/updated/deleted,
    // - `on_error` function to call whenever reconciliation fails.
    Controller::new(crd_api.clone(), ListParams::default())
        .run(reconcile, on_error, context)
        .for_each(|reconciliation_result| async move {
            match reconciliation_result {
                Ok(echo_resource) => {
                    println!("Reconciled: {:?}", echo_resource);
                }
                Err(reconciliation_err) => {
                    eprintln!("Reconciliation error: {:?}", reconciliation_err);
                }
            }
        })
        .await;
}

struct ContextData {
    client: Client,
}

impl ContextData {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

async fn reconcile(echo: Arc<Echo>, context: Arc<ContextData>) -> Result<Action, Error> {
    let client: Client = context.client.clone();

    let namespace: String = match echo.namespace() {
        None => {
            return Err(Error::UserInputError(
                "Expected Echo resource to be namespaced. Can't deploy to an unknown namespace."
                    .to_owned(),
            ));
        }
        Some(namespace) => namespace.to_owned(),
    };
    return match determine_action(&echo) {
        EchoAction::Create => {
            let name = echo.name_any();
            finalizer_add(client.clone(), &name, &namespace).await?;
            deployment_deploy(client, &echo.name_any(), echo.spec.replicas, &namespace).await?;
            Ok(Action::requeue(std::time::Duration::from_secs(10)))
        }
        EchoAction::Delete => {
            deployment_delete(client.clone(), &echo.name_any(), &namespace).await?;
            finalizer_delete(client.clone(), &echo.name_any(), &namespace).await?;
            Ok(Action::await_change())
        }
        EchoAction::NoOp => Ok(Action::requeue(std::time::Duration::from_secs(10))),
    };
}

fn determine_action(echo: &Echo) -> EchoAction {
    return if echo.meta().deletion_timestamp.is_some() {
        EchoAction::Delete
    } else if echo
        .meta()
        .finalizers
        .as_ref()
        .map_or(true, |finalizers| finalizers.is_empty())
    {
        EchoAction::Create
    } else {
        EchoAction::NoOp
    };
}

/// All errors possible to occur during reconciliation
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any error originating from the `kube-rs` crate
    #[error("Kubernetes reported error: {source}")]
    KubeError {
        #[from]
        source: kube::Error,
    },
    /// Error in user input or Echo resource definition, typically missing fields.
    #[error("Invalid Echo CRD: {0}")]
    UserInputError(String),
}

/// Actions to be taken when a reconciliation fails - for whatever reason.
/// Prints out the error to `stderr` and requeues the resource for another reconciliation after
/// five seconds.
///
/// # Arguments
/// - `error`: A reference to the `kube::Error` that occurred during reconciliation.
/// - `_context`: Unused argument. Context Data "injected" automatically by kube-rs.
fn on_error(error: &Error, _context: Arc<ContextData>) -> Action {
    eprintln!("Reconciliation error:\n{:?}", error);
    Action::requeue(std::time::Duration::from_secs(5))
}
