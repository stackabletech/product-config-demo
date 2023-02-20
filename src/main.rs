use clap::Parser;
use product_config::v3::{ProductConfig, Tls};

use crate::{
    product_config::{v1, v2},
    versioning::Versioner,
};

mod product_config;
mod versioning;

struct RgConfig {
    cluster_name: String,
    tls: bool,
}

fn reconciler_logic_stuff(RgConfig { cluster_name, tls }: RgConfig) -> ProductConfig {
    ProductConfig {
        insecure_cluster_url: format!("http://{cluster_name}.default.svc.cluster.local/"),
        cluster_name,
        tls: tls.then(|| Tls {
            key_file: "/stackable/tls/key.pem".to_string(),
            cert_file: "/stackable/tls/cert.pem".to_string(),
            ca_file: "/stackable/tls/ca.pem".to_string(),
        }),
    }
}

#[derive(clap::Parser)]
struct Opts {
    version: String,
    #[clap(long)]
    tls: bool,
}

fn main() {
    let opts = Opts::parse();
    let versioner = Versioner::<ProductConfig>::new()
        .with_old_version::<v2::ProductConfig>()
        .with_old_version::<v1::ProductConfig>();

    // merged rolegroup config
    // this would likely use fragment infra in practice, here we assume that this is already done
    let rg = RgConfig {
        cluster_name: "foo".to_string(),
        tls: opts.tls,
    };
    let product_config = reconciler_logic_stuff(rg);
    let product_config = versioner.downgrade_to(&opts.version, product_config);
    // in practice we'd split this for config files/env/args as before here
    let product_config_json = serde_json::to_string_pretty(&product_config).unwrap();
    // apply to k8s CM...
    println!("{product_config_json}");
}
