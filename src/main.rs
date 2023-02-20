use product_config::v1::ProductConfig;

mod product_config;

struct RgConfig {
    cluster_name: String,
}

fn reconciler_logic_stuff(RgConfig { cluster_name }: RgConfig) -> ProductConfig {
    ProductConfig {
        cluster_url: format!("http://{cluster_name}.default.svc.cluster.local/"),
        cluster_name,
    }
}

fn main() {
    // merged rolegroup config
    // this would likely use fragment infra in practice, here we assume that this is already done
    let rg = RgConfig {
        cluster_name: "foo".to_string(),
    };
    let product_config = reconciler_logic_stuff(rg);
    // in practice we'd split this for config files/env/args as before here
    let product_config_json = serde_json::to_string_pretty(&product_config).unwrap();
    // apply to k8s CM...
    println!("{product_config_json}");
}
